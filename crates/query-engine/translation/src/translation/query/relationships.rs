//! Handle stuff related to relationships and joins.

use std::collections::BTreeMap;

use super::root;
use crate::translation::error::Error;
use crate::translation::helpers::{Env, RootAndCurrentTables, State, TableNameAndReference};
use query_engine_sql::sql;

pub struct JoinFieldInfo {
    pub table_alias: sql::ast::TableAlias,
    pub relationship_name: String,
    pub arguments: BTreeMap<ndc_models::ArgumentName, ndc_models::RelationshipArgument>,
    pub query: ndc_models::Query,
}

/// translate any joins we should include in the query into our SQL AST
pub fn translate_joins(
    env: &Env,
    state: &mut State,
    root_and_current_tables: &RootAndCurrentTables,
    // We got these by processing the fields selection.
    join_fields: Vec<JoinFieldInfo>,
) -> Result<Vec<sql::ast::Join>, Error> {
    // traverse and build a join.
    join_fields
        .into_iter()
        .map(|join_field| {
            let relationship = env.lookup_relationship(&join_field.relationship_name.into())?;
            let arguments = make_relationship_arguments(MakeRelationshipArguments {
                caller_arguments: join_field.arguments,
                relationship_arguments: relationship.arguments.clone(),
            })?;

            let table_alias = state.make_table_alias(relationship.target_collection.to_string());

            // create a from clause and get a reference of inner query.
            let (target_collection, from_clause) = root::make_from_clause_and_reference(
                &relationship.target_collection,
                &arguments,
                env,
                state,
                &table_alias,
            )?;

            let target_collection_info =
                env.lookup_collection(&target_collection.name.clone().into())?;

            // process inner query and get the SELECTs for the 'rows' and 'aggregates' fields.
            let select_set = super::translate_query(
                env,
                state,
                &target_collection_info,
                &target_collection,
                &from_clause,
                &join_field.query,
                &table_alias,
            )?;

            // add join expressions to row / aggregate selects
            let final_select_set = match select_set {
                // Only rows
                sql::helpers::SelectSet::Rows(mut row_select) => {
                    let sql::ast::Where(row_expr) = row_select.where_;

                    row_select.where_ = sql::ast::Where(translate_column_mapping(
                        env,
                        &root_and_current_tables.current_table,
                        &target_collection.reference,
                        row_expr,
                        relationship,
                    )?);

                    Ok(sql::helpers::SelectSet::Rows(row_select))
                }
                // Only aggregates
                sql::helpers::SelectSet::Aggregates(mut aggregate_select) => {
                    let sql::ast::Where(aggregate_expr) = aggregate_select.where_;

                    aggregate_select.where_ = sql::ast::Where(translate_column_mapping(
                        env,
                        &root_and_current_tables.current_table,
                        &target_collection.reference,
                        aggregate_expr,
                        relationship,
                    )?);

                    Ok(sql::helpers::SelectSet::Aggregates(aggregate_select))
                }
                // Both
                sql::helpers::SelectSet::RowsAndAggregates(
                    mut row_select,
                    mut aggregate_select,
                ) => {
                    let sql::ast::Where(row_expr) = row_select.where_;

                    row_select.where_ = sql::ast::Where(translate_column_mapping(
                        env,
                        &root_and_current_tables.current_table,
                        &target_collection.reference,
                        row_expr,
                        relationship,
                    )?);

                    let sql::ast::Where(aggregate_expr) = aggregate_select.where_;

                    aggregate_select.where_ = sql::ast::Where(translate_column_mapping(
                        env,
                        &root_and_current_tables.current_table,
                        &target_collection.reference,
                        aggregate_expr,
                        relationship,
                    )?);

                    // Build (what will be) a RowSet with both fields.
                    Ok(sql::helpers::SelectSet::RowsAndAggregates(
                        row_select,
                        aggregate_select,
                    ))
                }
            }?;

            // form a single JSON item shaped `{ rows: [], aggregates: {} }`
            // that matches the ndc_models::RowSet type
            let json_select = sql::helpers::select_rowset(
                join_field.table_alias.clone(),
                state.make_table_alias("rows".to_string()),
                sql::helpers::make_column_alias("row_json".to_string()),
                state.make_table_alias("aggregates".to_string()),
                sql::helpers::make_column_alias("agg_json".to_string()),
                final_select_set,
            );

            let json_column_alias = sql::helpers::make_json_column_alias();

            Ok(sql::ast::Join::OuterApply(sql::ast::OuterApply {
                select: Box::new(json_select),
                alias: join_field.table_alias,
                alias_path: sql::ast::AliasPath {
                    elements: vec![json_column_alias],
                },
            }))
        })
        .collect::<Result<Vec<sql::ast::Join>, Error>>()
}

/// Given a relationship, turn it into a Where clause for a Join.
pub fn translate_column_mapping(
    env: &Env,
    current_table: &TableNameAndReference,
    target_collection_alias_reference: &sql::ast::TableReference,
    expr: sql::ast::Expression,
    relationship: &ndc_models::Relationship,
) -> Result<sql::ast::Expression, Error> {
    let table_info = env.lookup_collection(&current_table.name.clone().into())?;

    let target_collection_info = env.lookup_collection(&relationship.target_collection)?;

    relationship
        .column_mapping
        .iter()
        .map(|(source_col, target_col)| {
            let source_column_info = table_info.lookup_column(source_col)?;
            let target_column_info = target_collection_info.lookup_column(target_col)?;
            Ok(sql::ast::Expression::BinaryOperation {
                left: Box::new(sql::ast::Expression::ColumnReference(
                    sql::ast::ColumnReference::TableColumn {
                        table: current_table.reference.clone(),
                        name: source_column_info.name,
                    },
                )),
                operator: sql::ast::BinaryOperator("=".to_string()),
                right: Box::new(sql::ast::Expression::ColumnReference(
                    sql::ast::ColumnReference::TableColumn {
                        table: target_collection_alias_reference.clone(),
                        name: target_column_info.name,
                    },
                )),
            })
        })
        .try_fold(expr, |expr, op| {
            let op = op?;
            Ok(sql::ast::Expression::And {
                left: Box::new(expr),
                right: Box::new(op),
            })
        })
}

#[derive(Debug)]
/// Used in `make_relationship_arguments()` below.
pub struct MakeRelationshipArguments {
    pub relationship_arguments:
        BTreeMap<ndc_models::ArgumentName, ndc_models::RelationshipArgument>,
    pub caller_arguments: BTreeMap<ndc_models::ArgumentName, ndc_models::RelationshipArgument>,
}

/// Combine the caller arguments and the relationship arguments into a single map.
///
/// We don't support relationships column arguments yet, so for now we convert to a regular argument
/// and throw an error on the column case. Will be fixed in the future.
pub fn make_relationship_arguments(
    arguments: MakeRelationshipArguments,
) -> Result<BTreeMap<ndc_models::ArgumentName, ndc_models::Argument>, Error> {
    // these are arguments defined in the relationship definition.
    let relationship_arguments: BTreeMap<ndc_models::ArgumentName, ndc_models::Argument> =
        arguments
            .relationship_arguments
            .into_iter()
            .map(|(key, argument)| Ok((key, relationship_argument_to_argument(argument)?)))
            .collect::<Result<BTreeMap<ndc_models::ArgumentName, ndc_models::Argument>, Error>>()?;

    // these are arguments defined when calling the relationship.
    let caller_arguments: BTreeMap<ndc_models::ArgumentName, ndc_models::Argument> = arguments
        .caller_arguments
        .into_iter()
        .map(|(key, argument)| Ok((key, relationship_argument_to_argument(argument)?)))
        .collect::<Result<BTreeMap<ndc_models::ArgumentName, ndc_models::Argument>, Error>>()?;

    let mut arguments = relationship_arguments;

    // We do not allow caller arguments to override relationship defined arguments,
    // because those might be specified as permissions.
    // We don't expect the engine to return such queries, but add this as a precaution.
    for (key, value) in caller_arguments {
        match arguments.insert(key.clone(), value) {
            None => Ok(()),
            Some(_) => Err(Error::RelationshipArgumentWasOverriden(key.to_string())),
        }?;
    }

    Ok(arguments)
}

/// We don't support relationships column arguments yet, so for now we convert to a regular argument
/// and throw an error on the column case. Will be fixed in the future.
fn relationship_argument_to_argument(
    argument: ndc_models::RelationshipArgument,
) -> Result<ndc_models::Argument, Error> {
    match argument {
        ndc_models::RelationshipArgument::Literal { value } => {
            Ok(ndc_models::Argument::Literal { value })
        }
        ndc_models::RelationshipArgument::Variable { name } => {
            Ok(ndc_models::Argument::Variable { name })
        }
        ndc_models::RelationshipArgument::Column { .. } => Err(Error::NotSupported(
            "relationship column arguments".to_string(),
        )),
    }
}
