//! Handle filtering/where clauses translation.

use std::collections::BTreeMap;

use crate::translation::error::Error;
use crate::translation::helpers::{
    ColumnInfo, Env, RootAndCurrentTables, State, TableNameAndReference,
};
use crate::translation::values;

use super::relationships;
use super::root;
use query_engine_metadata::metadata::database;
use query_engine_sql::sql;

/// Translate a boolean expression to a SQL expression.
pub fn translate_expression(
    env: &Env,
    state: &mut State,
    root_and_current_tables: &RootAndCurrentTables,
    predicate: &ndc_models::Expression,
) -> Result<(sql::ast::Expression, Vec<sql::ast::Join>), Error> {
    match predicate {
        ndc_models::Expression::And { expressions } => {
            let mut acc_joins = vec![];
            let and_exprs = expressions
                .iter()
                .map(|expr| translate_expression(env, state, root_and_current_tables, expr))
                .try_fold(
                    sql::ast::Expression::Value(sql::ast::Value::Bool(true)),
                    |acc, expr| {
                        let (right, right_joins) = expr?;
                        acc_joins.extend(right_joins);
                        Ok(sql::ast::Expression::And {
                            left: Box::new(acc),
                            right: Box::new(right),
                        })
                    },
                )?;
            Ok((and_exprs, acc_joins))
        }
        ndc_models::Expression::Or { expressions } => {
            let mut acc_joins = vec![];
            let or_exprs = expressions
                .iter()
                .map(|expr| translate_expression(env, state, root_and_current_tables, expr))
                .try_fold(
                    sql::ast::Expression::Value(sql::ast::Value::Bool(false)),
                    |acc, expr| {
                        let (right, right_joins) = expr?;
                        acc_joins.extend(right_joins);
                        Ok(sql::ast::Expression::Or {
                            left: Box::new(acc),
                            right: Box::new(right),
                        })
                    },
                )?;
            Ok((or_exprs, acc_joins))
        }
        ndc_models::Expression::Not { expression } => {
            let (expr, joins) =
                translate_expression(env, state, root_and_current_tables, expression)?;
            Ok((sql::ast::Expression::Not(Box::new(expr)), joins))
        }
        ndc_models::Expression::BinaryComparisonOperator {
            column,
            operator,
            value,
        } => {
            let left_typ = get_comparison_target_type(env, root_and_current_tables, column)?;
            let (left, left_joins) =
                translate_comparison_target(env, state, root_and_current_tables, column)?;
            let op = env.lookup_comparison_operator(&left_typ, operator)?;
            if op.operator_kind == database::OperatorKind::In {
                let mut joins = vec![];
                joins.extend(left_joins);
                match value {
                    ndc_models::ComparisonValue::Column { column } => {
                        let (right, right_joins) = translate_comparison_target(
                            env,
                            state,
                            root_and_current_tables,
                            column,
                        )?;
                        joins.extend(right_joins);
                        Ok((
                            sql::ast::Expression::BinaryArrayOperation {
                                left: Box::new(left),
                                operator: sql::ast::BinaryArrayOperator::In,
                                right: vec![right],
                            },
                            joins,
                        ))
                    }
                    ndc_models::ComparisonValue::Scalar { value: json_value } => match json_value {
                        serde_json::Value::Array(values) => {
                            // The expression on the left is definitely not IN an empty list of values
                            if values.is_empty() {
                                Ok((sql::helpers::false_expr(), joins))
                            } else {
                                let right = values
                                    .iter()
                                    .map(|value| {
                                        let (right, right_joins) = translate_comparison_value(
                                            env,
                                            state,
                                            root_and_current_tables,
                                            &ndc_models::ComparisonValue::Scalar {
                                                value: value.clone(),
                                            },
                                            &left_typ.clone(),
                                        )?;
                                        joins.extend(right_joins);
                                        Ok(right)
                                    })
                                    .collect::<Result<Vec<sql::ast::Expression>, Error>>()?;

                                Ok((
                                    sql::ast::Expression::BinaryArrayOperation {
                                        left: Box::new(left),
                                        operator: sql::ast::BinaryArrayOperator::In,
                                        right,
                                    },
                                    joins,
                                ))
                            }
                        }
                        _ => Err(Error::TypeMismatch(json_value.clone(), left_typ)),
                    },
                    ndc_models::ComparisonValue::Variable { .. } => {
                        // TODO(PY): array type
                        let array_type = left_typ;
                        let (right, right_joins) = translate_comparison_value(
                            env,
                            state,
                            root_and_current_tables,
                            value,
                            &array_type,
                        )?;
                        joins.extend(right_joins);

                        Ok((
                            sql::ast::Expression::BinaryOperation {
                                left: Box::new(left),
                                operator: sql::ast::BinaryOperator(op.operator_name.clone()),
                                right: Box::new(right),
                            },
                            joins,
                        ))
                    }
                }
            } else {
                let mut joins = vec![];
                joins.extend(left_joins);
                let (right, right_joins) = translate_comparison_value(
                    env,
                    state,
                    root_and_current_tables,
                    value,
                    &op.argument_type,
                )?;
                joins.extend(right_joins);
                Ok((
                    sql::ast::Expression::BinaryOperation {
                        left: Box::new(left),
                        operator: sql::ast::BinaryOperator(op.operator_name.clone()),
                        right: Box::new(right),
                    },
                    joins,
                ))
            }
        }

        ndc_models::Expression::Exists {
            in_collection,
            predicate,
        } => match predicate {
            None => Ok((sql::helpers::true_expr(), vec![])),
            Some(predicate) => Ok((
                translate_exists_in_collection(
                    env,
                    state,
                    root_and_current_tables,
                    in_collection.clone(),
                    predicate,
                )?,
                vec![],
            )),
        },
        ndc_models::Expression::UnaryComparisonOperator { column, operator } => match operator {
            ndc_models::UnaryComparisonOperator::IsNull => {
                let (value, joins) =
                    translate_comparison_target(env, state, root_and_current_tables, column)?;

                Ok((
                    sql::ast::Expression::UnaryOperation {
                        expression: Box::new(value),
                        operator: sql::ast::UnaryOperator::IsNull,
                    },
                    joins,
                ))
            }
        },
    }
}

/// Given a vector of PathElements and the table alias for the table the
/// expression is over, we return a join in the form of:
///
///   INNER JOIN LATERAL
///   (
///     SELECT *
///     FROM
///       <table of path[0]> AS <fresh name>
///     WHERE
///       <table 0 join condition>
///       AND <predicate of path[0]>
///     AS <fresh name>
///   )
///   INNER JOIN LATERAL
///   (
///     SELECT *
///     FROM
///        <table of path[1]> AS <fresh name>
///     WHERE
///        <table 1 join condition on table 0>
///        AND <predicate of path[1]>
///   ) AS <fresh name>
///   ...
///   INNER JOIN LATERAL
///   (
///       SELECT *
///       FROM
///          <table of path[m]> AS <fresh name>
///       WHERE
///          <table m join condition on table m-1>
///          AND <predicate of path[m]>
///   ) AS <fresh name>
///
/// and the aliased table name under which the sought colum can be found, i.e.
/// the last drawn fresh name. Or, in the case of an empty paths vector, simply
/// the alias that was input.
///
fn translate_comparison_pathelements(
    env: &Env,
    state: &mut State,
    root_and_current_tables: &RootAndCurrentTables,
    path: &[ndc_models::PathElement],
) -> Result<(TableNameAndReference, Vec<sql::ast::Join>), Error> {
    let mut joins = vec![];
    let RootAndCurrentTables { current_table, .. } = root_and_current_tables;

    let final_ref = path.iter().try_fold(
        current_table.clone(),
        |current_table_ref,
         ndc_models::PathElement {
             relationship,
             predicate,
             arguments,
         }| {
            // get the relationship table
            let relationship_name = &relationship;
            let relationship = env.lookup_relationship(relationship_name)?;

            // new alias for the target table
            let target_table_alias: sql::ast::TableAlias = state
                .make_boolean_expression_table_alias(
                    &relationship.target_collection.clone().to_string(),
                );

            let arguments = relationships::make_relationship_arguments(
                relationships::MakeRelationshipArguments {
                    caller_arguments: arguments.clone(),
                    relationship_arguments: relationship.arguments.clone(),
                },
            )?;

            // create a from clause and get a reference of inner query.
            let (table, from_clause) = root::make_from_clause_and_reference(
                &relationship.target_collection,
                &arguments,
                env,
                state,
                &target_table_alias,
            )?;

            // build a SELECT querying this table with the relevant predicate.
            let mut select = sql::helpers::simple_select(vec![]);
            select.from = Some(from_clause);

            select.select_list = sql::ast::SelectList::SelectStar;

            let new_root_and_current_tables = RootAndCurrentTables {
                root_table: root_and_current_tables.root_table.clone(),
                current_table: TableNameAndReference {
                    reference: table.reference.clone(),
                    name: table.name.clone(),
                },
            };
            // relationship-specfic filter
            let (rel_cond, rel_joins) = match predicate {
                None => (sql::helpers::true_expr(), vec![]),
                Some(predicate) => {
                    translate_expression(env, state, &new_root_and_current_tables, predicate)?
                }
            };

            // relationship where clause
            let cond = relationships::translate_column_mapping(
                env,
                &current_table_ref,
                &table.reference,
                rel_cond,
                relationship,
            )?;

            select.joins = rel_joins;

            joins.push(sql::ast::Join::InnerJoin(sql::ast::InnerJoin {
                select: Box::new(select),
                alias: target_table_alias,
                on: cond,
            }));
            Ok(new_root_and_current_tables.current_table)
        },
    )?;

    Ok((final_ref, joins))
}

/// translate a comparison target.
fn translate_comparison_target(
    env: &Env,
    state: &mut State,
    root_and_current_tables: &RootAndCurrentTables,
    column: &ndc_models::ComparisonTarget,
) -> Result<(sql::ast::Expression, Vec<sql::ast::Join>), Error> {
    match column {
        ndc_models::ComparisonTarget::Column {
            name,
            path,
            field_path: _,
        } => {
            let (table_ref, joins) =
                translate_comparison_pathelements(env, state, root_and_current_tables, path)?;

            // get the unrelated table information from the metadata.
            let collection_info = env.lookup_collection(&table_ref.name.into())?;
            let ColumnInfo { name, .. } = collection_info.lookup_column(name)?;

            Ok((
                sql::ast::Expression::ColumnReference(sql::ast::ColumnReference::TableColumn {
                    table: table_ref.reference,
                    name,
                }),
                joins,
            ))
        }

        // Compare a column from the root table.
        ndc_models::ComparisonTarget::RootCollectionColumn {
            name,
            field_path: _,
        } => {
            let RootAndCurrentTables { root_table, .. } = root_and_current_tables;
            // get the unrelated table information from the metadata.
            let collection_info = env.lookup_collection(&root_table.name.clone().into())?;

            // find the requested column in the tables columns.
            let ColumnInfo { name, .. } = collection_info.lookup_column(name)?;

            Ok((
                sql::ast::Expression::ColumnReference(sql::ast::ColumnReference::TableColumn {
                    table: root_table.reference.clone(),
                    name,
                }),
                vec![],
            ))
        }
    }
}

/// translate a comparison value.
fn translate_comparison_value(
    env: &Env,
    state: &mut State,
    root_and_current_tables: &RootAndCurrentTables,
    value: &ndc_models::ComparisonValue,
    typ: &database::ScalarType,
) -> Result<(sql::ast::Expression, Vec<sql::ast::Join>), Error> {
    match value {
        ndc_models::ComparisonValue::Column { column } => {
            translate_comparison_target(env, state, root_and_current_tables, column)
        }
        ndc_models::ComparisonValue::Scalar { value: json_value } => {
            Ok((values::translate_json_value(json_value, typ)?, vec![]))
        }
        ndc_models::ComparisonValue::Variable { name: var } => {
            Ok((values::translate_variable(var, typ), vec![]))
        }
    }
}

/// Translate an EXISTS clause into a SQL subquery of the following form:
///
/// > EXISTS (SELECT 1 as 'one' FROM <table> AS <alias> WHERE <predicate>)
pub fn translate_exists_in_collection(
    env: &Env,
    state: &mut State,
    root_and_current_tables: &RootAndCurrentTables,
    in_collection: ndc_models::ExistsInCollection,
    predicate: &ndc_models::Expression,
) -> Result<sql::ast::Expression, Error> {
    match in_collection {
        ndc_models::ExistsInCollection::NestedCollection {
            column_name: _,
            arguments: _,
            field_path: _,
        } => todo!("Not implemented"),
        ndc_models::ExistsInCollection::Unrelated {
            collection,
            arguments,
        } => {
            let arguments = relationships::make_relationship_arguments(
                relationships::MakeRelationshipArguments {
                    caller_arguments: BTreeMap::new(),
                    relationship_arguments: arguments,
                },
            )?;

            let table_alias = state.make_table_alias(collection.to_string());

            // create a from clause and get a reference of inner query.
            let (table, from_clause) = root::make_from_clause_and_reference(
                &collection,
                &arguments,
                env,
                state,
                &table_alias,
            )?;

            // CockroachDB doesn't like empty selects, so we do "SELECT 1 as 'one' ..."
            let column_alias = sql::helpers::make_column_alias("one".to_string());

            let select_cols = vec![(
                column_alias,
                sql::ast::Expression::Value(sql::ast::Value::Int8(1)),
            )];

            // build a SELECT querying this table with the relevant predicate.
            let mut select = sql::helpers::simple_select(select_cols);
            select.from = Some(from_clause);

            let new_root_and_current_tables = RootAndCurrentTables {
                root_table: root_and_current_tables.root_table.clone(),
                current_table: TableNameAndReference {
                    reference: table.reference.clone(),
                    name: table.name,
                },
            };

            let (expr, expr_joins) =
                translate_expression(env, state, &new_root_and_current_tables, predicate)?;
            select.where_ = sql::ast::Where(expr);

            select.joins = expr_joins;

            // > EXISTS (SELECT 1 as 'one' FROM <table> AS <alias> WHERE <predicate>)
            Ok(sql::ast::Expression::Exists {
                select: Box::new(select),
            })
        }
        // We get a relationship name in exists, query the target table directly,
        // and build a WHERE clause that contains the join conditions and the specified
        // EXISTS condition.
        ndc_models::ExistsInCollection::Related {
            relationship,
            arguments,
        } => {
            // get the relationship table
            let relationship = env.lookup_relationship(&relationship)?;

            let arguments = relationships::make_relationship_arguments(
                relationships::MakeRelationshipArguments {
                    caller_arguments: arguments,
                    relationship_arguments: relationship.arguments.clone(),
                },
            )?;

            let table_alias = state.make_table_alias(relationship.target_collection.to_string());

            // create a from clause and get a reference of inner query.
            let (table, from_clause) = root::make_from_clause_and_reference(
                &relationship.target_collection,
                &arguments,
                env,
                state,
                &table_alias,
            )?;

            // CockroachDB doesn't like empty selects, so we do "SELECT 1 as 'one' ..."
            let column_alias = sql::helpers::make_column_alias("one".to_string());

            let select_cols = vec![(
                column_alias,
                sql::ast::Expression::Value(sql::ast::Value::Int8(1)),
            )];

            // build a SELECT querying this table with the relevant predicate.
            let mut select = sql::helpers::simple_select(select_cols);
            select.from = Some(from_clause);

            let new_root_and_current_tables = RootAndCurrentTables {
                root_table: root_and_current_tables.root_table.clone(),
                current_table: TableNameAndReference {
                    reference: table.reference.clone(),
                    name: table.name.clone(),
                },
            };

            // exists condition
            let (exists_cond, exists_joins) =
                translate_expression(env, state, &new_root_and_current_tables, predicate)?;

            // relationship where clause
            let cond = relationships::translate_column_mapping(
                env,
                &root_and_current_tables.current_table,
                &table.reference,
                exists_cond,
                relationship,
            )?;

            select.where_ = sql::ast::Where(cond);

            select.joins = exists_joins;

            // > EXISTS (SELECT 1 as 'one' FROM <table> AS <alias> WHERE <predicate>)
            Ok(sql::ast::Expression::Exists {
                select: Box::new(select),
            })
        }
    }
}

/// Extract the scalar type of a comparison target
fn get_comparison_target_type(
    env: &Env,
    root_and_current_tables: &RootAndCurrentTables,
    column: &ndc_models::ComparisonTarget,
) -> Result<database::ScalarType, Error> {
    match column {
        ndc_models::ComparisonTarget::RootCollectionColumn {
            name,
            field_path: _,
        } => {
            let column = env
                .lookup_collection(&root_and_current_tables.root_table.name.clone().into())?
                .lookup_column(name)?;
            Ok(column.r#type)
        }
        ndc_models::ComparisonTarget::Column {
            name,
            path,
            field_path: _,
        } => match path.last() {
            None => {
                let column = env
                    .lookup_collection(&root_and_current_tables.current_table.name.clone().into())?
                    .lookup_column(name)?;
                Ok(column.r#type)
            }
            Some(last) => {
                let column = env
                    .lookup_collection(
                        &env.lookup_relationship(&last.relationship)?
                            .target_collection,
                    )?
                    .lookup_column(name)?;
                Ok(column.r#type)
            }
        },
    }
}
