//! Handle 'rows' and 'aggregates' translation.

use std::collections::BTreeMap;

use indexmap::IndexMap;

use ndc_sdk::models;

use super::aggregates;
use super::filtering;
use crate::translation::error::Error;
use crate::translation::helpers::CollectionOrProcedureInfo;
use crate::translation::helpers::{
    CollectionInfo, Env, RootAndCurrentTables, State, TableNameAndReference,
};

use super::relationships;
use super::sorting;
use query_engine_sql::sql;

/// Translate aggregates query to sql ast.
pub fn translate_aggregate_query(
    env: &Env,
    state: &mut State,
    current_table: &TableNameAndReference,
    from_clause: &sql::ast::From,
    query: &models::Query,
) -> Result<Option<sql::ast::Select>, Error> {
    match &query.aggregates {
        None => Ok(None),
        Some(aggregate_fields) => {
            // create all aggregate columns
            let aggregate_columns =
                aggregates::translate(&current_table.reference, aggregate_fields)?;

            // create the select clause and the joins, order by, where clauses.
            // We don't add the limit afterwards.
            let mut select =
                translate_query_part(env, state, current_table, query, aggregate_columns, vec![])?;

            // we remove the order by part though because it is only relevant for group by clauses,
            // which we don't support at the moment.
            select.order_by = sql::helpers::empty_order_by();

            // turn these into a single JSON result
            select.for_json = sql::ast::ForJson::ForJsonPathWithoutArrayWrapper;

            select.from = Some(from_clause.clone());

            Ok(Some(select))
        }
    }
}

/*

SELECT
  JSON_QUERY(
    CONCAT('[', STRING_AGG(cast('{}' as nvarchar(MAX)), ','), ']')
  ) AS [no_fields]
FROM
  ( SELECT * FROM <table-name> ORDER BY 1 ASC OFFSET <offset> ROWS FETCH NEXT <limit> ROWS ONLY
    WHERE <where-clause>
  )
*/
pub fn make_no_fields_select_query(
    env: &Env,
    state: &mut State,
    from_clause: &sql::ast::From,
    query: &models::Query,
    table_alias: &sql::ast::TableAlias,
    current_table: &TableNameAndReference,
) -> Result<sql::ast::Select, Error> {
    // Root table and current table are same at this point.
    let root_and_current_table = RootAndCurrentTables {
        root_table: current_table.clone(),
        current_table: current_table.clone(),
    };
    let where_expression = match &query.predicate {
        Some(predicate) => {
            let expression =
                filtering::translate_expression(env, state, &root_and_current_table, predicate)?.0;
            sql::ast::Where(expression)
        }
        None => sql::ast::Where(sql::helpers::empty_where()),
    };
    let select = sql::ast::Select {
        with: sql::helpers::empty_with(),
        select_list: sql::ast::SelectList::SelectList(vec![(
            sql::ast::ColumnAlias {
                name: "no_fields".to_string(),
            },
            sql::ast::Expression::FunctionCall {
                function: sql::ast::Function::Unknown("JSON_QUERY".to_string()),
                args: vec![sql::ast::Expression::FunctionCall {
                    function: sql::ast::Function::Unknown("CONCAT".to_string()),
                    args: vec![
                        sql::ast::Expression::Value(sql::ast::Value::String("[".to_string())),
                        sql::ast::Expression::FunctionCall {
                            function: sql::ast::Function::Unknown("STRING_AGG".to_string()),
                            args: vec![
                                sql::ast::Expression::Cast {
                                    expression: Box::new(sql::ast::Expression::Value(
                                        sql::ast::Value::String("{}".to_string()),
                                    )),
                                    r#type: sql::ast::ScalarType("nvarchar(MAX)".to_string()),
                                },
                                sql::ast::Expression::Value(sql::ast::Value::String(
                                    ",".to_string(),
                                )),
                            ],
                        },
                        sql::ast::Expression::Value(sql::ast::Value::String("]".to_string())),
                    ],
                }],
            },
        )]),
        from: Some(sql::ast::From::Select {
            select: Box::new(sql::ast::Select {
                with: sql::helpers::empty_with(),
                select_list: sql::ast::SelectList::SelectStar,
                from: Some(from_clause.clone()),
                joins: vec![],
                where_: where_expression,
                group_by: sql::helpers::empty_group_by(),
                order_by: sql::ast::OrderBy {
                    elements: vec![sql::ast::OrderByElement {
                        target: sql::ast::Expression::Value(sql::ast::Value::Int8(1)),
                        direction: sql::ast::OrderByDirection::Asc,
                    }],
                },
                limit: match (query.limit, query.offset) {
                    (None, None) => None,
                    (limit, Some(offset)) => Some(sql::ast::Limit { limit, offset }),
                    (limit, None) => Some(sql::ast::Limit { limit, offset: 0 }),
                },
                for_json: sql::ast::ForJson::NoJson,
            }),
            alias: table_alias.clone(),
            alias_path: sql::ast::AliasPath {
                elements: Vec::new(),
            },
        }),
        joins: Vec::new(),
        where_: sql::ast::Where(sql::helpers::empty_where()),
        group_by: sql::helpers::empty_group_by(),
        order_by: sql::helpers::empty_order_by(),
        for_json: sql::ast::ForJson::NoJson,
        limit: None,
    };

    Ok(select)
}

/// Translate rows part of query to sql ast.
pub fn translate_rows_query(
    env: &Env,
    state: &mut State,
    collection_info: &CollectionOrProcedureInfo,
    current_table: &TableNameAndReference,
    from_clause: &sql::ast::From,
    query: &models::Query,
    table_alias: &sql::ast::TableAlias,
) -> Result<Option<sql::ast::Select>, Error> {
    // join aliases
    let mut join_fields: Vec<relationships::JoinFieldInfo> = vec![];

    // translate fields to select list
    if let Some(fields) = query.fields.clone() {
        // remember whether we fields were requested or not.
        // The case were fields were not requested, and also no aggregates were requested,
        // can be used for `__typename` queries.
        if IndexMap::is_empty(&fields) && query.aggregates.is_none() {
            Some(make_no_fields_select_query(
                env,
                state,
                from_clause,
                query,
                table_alias,
                current_table,
            ))
            .transpose()
        } else {
            // translate fields to columns or relationships.
            let columns: Vec<(sql::ast::ColumnAlias, sql::ast::Expression)> = fields
                .into_iter()
                .map(|(alias, field)| match field {
                    models::Field::Column { column, .. } => {
                        let column_info = collection_info.lookup_column(&column)?;
                        Ok(sql::helpers::make_column(
                            current_table.reference.clone(),
                            column_info.name.clone(),
                            sql::helpers::make_column_alias(alias.to_string()),
                        ))
                    }
                    models::Field::Relationship {
                        query,
                        relationship,
                        arguments,
                    } => {
                        let table_alias =
                            state.make_relationship_table_alias(&alias.clone().into());
                        let column_alias = sql::helpers::make_column_alias(alias.to_string());
                        let json_column_alias = sql::helpers::make_json_column_alias();
                        let column_name = sql::ast::ColumnReference::AliasedColumn {
                            table: sql::ast::TableReference::AliasedTable(table_alias.clone()),
                            column: json_column_alias.clone(),
                        };
                        join_fields.push(relationships::JoinFieldInfo {
                            table_alias,
                            column_alias: column_alias.clone(),
                            relationship_name: relationship.to_string(),
                            arguments,
                            query: *query,
                        });
                        Ok((
                            column_alias,
                            sql::ast::Expression::JsonQuery(
                                Box::new(sql::ast::Expression::ColumnReference(column_name)),
                                sql::helpers::empty_json_path(),
                            ),
                        ))
                    }
                })
                .collect::<Result<Vec<_>, Error>>()?;

            // create the select clause and the joins, order by, where clauses.
            // We'll add the limit afterwards.
            let mut select =
                translate_query_part(env, state, current_table, query, columns, join_fields)?;

            // turn these into an array of JSON results
            select.for_json = sql::ast::ForJson::ForJsonPath;

            select.from = Some(from_clause.clone());

            // if query has limit or offset, and no order_by, then create a default
            let has_limit_or_offset: bool =
                Option::is_some(&query.limit) || Option::is_some(&query.offset);

            if has_limit_or_offset && select.order_by.elements.is_empty() {
                match collection_info {
                    crate::translation::helpers::CollectionOrProcedureInfo::Collection(
                        collection_info,
                    ) => {
                        match collection_info {
                    CollectionInfo::Table { info, .. } => {
                        select.order_by = sorting::default_table_order_by(
                            info,
                            current_table.reference.clone(),
                        )?;
                    }
                    CollectionInfo::NativeQuery { info, .. } => {
                        select.order_by = sorting::default_native_query_order_by(
                            info.clone(),
                            current_table.reference.clone(),
                        )?;
                    }
                    // Native mutations will not have limit or offset
                    CollectionInfo::NativeMutation { .. } => {
                        return Err(Error::UnexpectedInternalError(
                            "Unexpected: found native mutation query with a limit/offset clause"
                                .to_string(),
                        ))
                    }
                }
                    }
                    crate::translation::helpers::CollectionOrProcedureInfo::Procedure(_) => {
                        return Err(Error::UnexpectedInternalError(
                            "Unexpected: found procedure with limit/offset".to_string(),
                        ))
                    }
                }
            }

            // Add the limit.
            select.limit = match (query.limit, query.offset) {
                (None, None) => None,
                (limit, Some(offset)) => Some(sql::ast::Limit { limit, offset }),
                (limit, None) => Some(sql::ast::Limit { limit, offset: 0 }),
            };

            Ok(Some(select))
        }
    } else {
        Ok(None)
    }
}

/// Translate the lion (or common) part of 'rows' or 'aggregates' part of a query.
/// Specifically, from, joins, order bys, and where clauses.
///
/// This expects to get the relevant information about tables, relationships, the root table,
/// and the query, as well as the columns and join fields after processing.
///
/// One thing that this doesn't do that you want to do for 'rows' and not 'aggregates' is
/// set the limit and offset so you want to do that after calling this function.
fn translate_query_part(
    env: &Env,
    state: &mut State,
    current_table: &TableNameAndReference,
    query: &models::Query,
    columns: Vec<(sql::ast::ColumnAlias, sql::ast::Expression)>,
    join_fields: Vec<relationships::JoinFieldInfo>,
) -> Result<sql::ast::Select, Error> {
    let root_table = current_table.clone();

    // the root table and the current table are the same at this point
    let root_and_current_tables = RootAndCurrentTables {
        root_table,
        current_table: current_table.clone(),
    };

    // construct a simple select with the table name, alias, and selected columns.
    let mut select = sql::helpers::simple_select(columns);

    // collect any joins for relationships
    let mut relationship_joins =
        relationships::translate_joins(env, state, &root_and_current_tables, join_fields)?;

    // translate order_by
    let (order_by, order_by_joins) =
        sorting::translate_order_by(env, state, &root_and_current_tables, &query.order_by)?;

    relationship_joins.extend(order_by_joins);
    // translate where
    let (filter, filter_joins) = match query.clone().predicate {
        None => Ok((
            sql::ast::Expression::Value(sql::ast::Value::Bool(true)),
            vec![],
        )),
        Some(predicate) => {
            filtering::translate_expression(env, state, &root_and_current_tables, &predicate)
        }
    }?;

    select.where_ = sql::ast::Where(filter);

    relationship_joins.extend(filter_joins);

    select.joins = relationship_joins;

    select.order_by = order_by;

    Ok(select)
}

/// Create a from clause from a collection name and its reference.
pub fn make_from_clause_and_reference(
    collection_name: &models::CollectionName,
    arguments: &BTreeMap<models::ArgumentName, models::Argument>,
    env: &Env,
    state: &mut State,
    collection_alias: &sql::ast::TableAlias,
) -> Result<(TableNameAndReference, sql::ast::From), Error> {
    let collection_alias_name = sql::ast::TableReference::AliasedTable(collection_alias.clone());

    // find the table according to the metadata.
    let collection_info = env.lookup_collection(collection_name)?;
    let from_clause = make_from_clause(state, collection_alias, &collection_info, arguments)?;

    let current_table = TableNameAndReference {
        name: collection_name.to_string(),
        reference: collection_alias_name.clone(),
    };
    Ok((current_table, from_clause))
}

/// Build a FROM clause from a collection info and an alias.
/// Will add a Native Query to the 'State' if the collection is a native query.
fn make_from_clause(
    state: &mut State,
    current_table_alias: &sql::ast::TableAlias,
    collection_info: &CollectionOrProcedureInfo,
    arguments: &BTreeMap<models::ArgumentName, models::Argument>,
) -> Result<sql::ast::From, Error> {
    match &collection_info {
        CollectionOrProcedureInfo::Collection(collection_info) => match collection_info {
            CollectionInfo::Table { info, .. } => {
                let db_table = sql::ast::TableReference::DBTable {
                    schema: sql::ast::SchemaName(info.schema_name.clone()),
                    table: sql::ast::TableName(info.table_name.clone()),
                };

                Ok(sql::ast::From::Table {
                    reference: db_table,
                    alias: current_table_alias.clone(),
                })
            }

            CollectionInfo::NativeQuery { name, info } => {
                let aliased_table =
                    state.insert_native_query(name, info.clone(), arguments.clone());
                Ok(sql::ast::From::Table {
                    reference: aliased_table,
                    alias: current_table_alias.clone(),
                })
            }
            CollectionInfo::NativeMutation { .. } => Err(Error::UnexpectedInternalError(
                "Native mutations can't have a `FROM` clause attached with them".into(),
            )),
        },
        CollectionOrProcedureInfo::Procedure(_procedure_info) => {
            Err(Error::UnexpectedInternalError(
                "Procedures can't have a `FROM` clause attached with them".into(),
            ))
        }
    }
}
