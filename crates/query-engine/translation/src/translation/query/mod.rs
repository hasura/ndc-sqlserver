//! Translate an incoming `QueryRequest`.

pub mod error;

mod aggregates;
mod filtering;
mod helpers;
mod native_queries;
mod operators;
mod relationships;
mod root;
mod sorting;
mod values;

use ndc_sdk::models;

use error::Error;
use helpers::{Env, State, TableNameAndReference};
use query_engine_metadata::metadata;
use query_engine_sql::sql::{self, ast::From};

/// Translate the incoming QueryRequest to an ExecutionPlan (SQL) to be run against the database.
pub fn translate(
    metadata: &metadata::Metadata,
    query_request: models::QueryRequest,
) -> Result<sql::execution_plan::ExecutionPlan, Error> {
    let env = Env::new(metadata, query_request.collection_relationships);
    let mut state = State::new();
    let (current_table, from_clause) = root::make_from_clause_and_reference(
        &query_request.collection,
        &query_request.arguments,
        &env,
        &mut state,
        None,
    )?;

    let select_set = translate_query(
        &env,
        &mut state,
        &current_table,
        &from_clause,
        query_request.query,
    )?;

    // form a single JSON item shaped `{ rows: [], aggregates: {} }`
    // that matches the models::RowSet type
    let mut json_select = sql::helpers::select_rowset(
        state.make_table_alias("universe".to_string()),
        state.make_table_alias("rows".to_string()),
        sql::helpers::make_column_alias("rows".to_string()),
        state.make_table_alias("aggregates".to_string()),
        sql::helpers::make_column_alias("aggregates".to_string()),
        select_set,
    );

    // dbg!("json_select", &json_select);

    // add native queries if there are any
    json_select.with = sql::ast::With {
        common_table_expressions: native_queries::translate(state)?,
    };

    // normalize ast
    let json_select = sql::rewrites::constant_folding::normalize_select(json_select);

    Ok(sql::execution_plan::simple_exec_plan(
        query_request.variables,
        query_request.collection,
        json_select,
    ))
}

/// Translate a query to sql ast.
/// We return a SELECT for the 'rows' field and a SELECT for the 'aggregates' field.
pub fn translate_query(
    env: &Env,
    state: &mut State,
    current_table: &TableNameAndReference,
    from_clause: &sql::ast::From,
    query: models::Query,
) -> Result<sql::helpers::SelectSet, Error> {
    // // Error::NoFields becomes Ok(None)
    // // everything stays Err
    // let map_no_fields_error_to_none = |err| match err {
    //     Error::NoFields => Ok(None),
    //     other_error => Err(other_error),
    // };

    // wrap valid result in Some
    // let wrap_ok = |a| Ok(Some(a));

    // translate rows query. if there are no fields, make this a None
    let row_select =
        root::translate_rows_query(env, state, current_table, from_clause, &query)?;
            // .map_or_else(map_no_fields_error_to_none, wrap_ok)?;

    // translate aggregate select. if there are no fields, make this a None
    let aggregate_select = root::translate_aggregate_query(env, state, current_table, from_clause, &query)?;

    match (row_select, aggregate_select) {
        ((root::ReturnsFields::FieldsWereRequested, rows), None) => Ok(sql::helpers::SelectSet::Rows(rows)),
        ((root::ReturnsFields::NoFieldsWereRequested, _), Some(aggregates)) => Ok(sql::helpers::SelectSet::Aggregates(aggregates)),
        ((root::ReturnsFields::FieldsWereRequested, rows), Some(aggregates)) => {
            Ok(sql::helpers::SelectSet::RowsAndAggregates(rows, aggregates))
        }
        ((root::ReturnsFields::NoFieldsWereRequested, _), None) => {
            let select = sql::ast::Select {
                with: sql::helpers::empty_with(),
                select_list: sql::ast::SelectList::SelectList(
                    vec![(
                        sql::ast::ColumnAlias{
                            name: "no_fields".to_string(),
                        },
                        sql::ast::Expression::FunctionCall { function: sql::ast::Function::Unknown("JSON_QUERY".to_string()), args: vec![
                            sql::ast::Expression::FunctionCall { function: sql::ast::Function::Unknown("CONCAT".to_string()), args: vec![
                                sql::ast::Expression::Value(sql::ast::Value::String("[".to_string())),
                                sql::ast::Expression::FunctionCall { function: sql::ast::Function::Unknown("STRING_AGG".to_string()), args: vec![
                                    sql::ast::Expression::Cast { expression: Box::new(sql::ast::Expression::Value(sql::ast::Value::String("{}".to_string()))),
                                        r#type: sql::ast::ScalarType("nvarchar(MAX)".to_string()) },
                                    // sql::ast::Expression::Value(sql::ast::Value::String("{}".to_string())),
                                    // sql::ast::Expression::FunctionCall { function: sql::ast::Function::Unknown("CAST".to_string()), args: vec![
                                    //     // sql::ast::Expression::Value(sql::ast::Value::String("'{}' as nvarchar(MAX)".to_string()))
                                    //     sql::ast::Expression::FunctionCall { function: sql::ast::Function::Unknown("AS".to_string()), args: vec![
                                    //         sql::ast::Expression::Value(sql::ast::Value::String("'{}'".to_string())),
                                    //         sql::ast::Expression::Value(sql::ast::Value::String("nvarchar(MAX)".to_string())),
                                    //     ] }
                                    // ] },
                                    sql::ast::Expression::Value(sql::ast::Value::String(",".to_string())),
                                ] },
                                sql::ast::Expression::Value(sql::ast::Value::String("]".to_string())),
                                ] }
                            ] }
                    )],
                ),
                // from: Some(from_clause.clone()),
                from: Some(sql::ast::From::Select { 
                                    select: Box::new(
                                        sql::ast::Select {
                                            with: sql::helpers::empty_with(),
                                            select_list: sql::ast::SelectList::SelectStar,
                                            from: Some(from_clause.clone()),
                                            joins: vec![],
                                            where_: sql::ast::Where(sql::helpers::empty_where()),
                                            group_by: sql::helpers::empty_group_by(),
                                            // order_by: sql::helpers::empty_order_by(),
                                            order_by: sql::ast::OrderBy{
                                                elements: vec![
                                                    sql::ast::OrderByElement{
                                                        target: sql::ast::Expression::Value(sql::ast::Value::Int8(1)),
                                                        direction: sql::ast::OrderByDirection::Asc,
                                                    }
                                                ]
                                            },
                                            // limit: None,
                                            limit: 
                                                // Add the limit.
                                                match (query.limit, query.offset) {
                                                    (None, None) => None,
                                                    (limit, Some(offset)) => Some(sql::ast::Limit { limit, offset }),
                                                    (limit, None) => Some(sql::ast::Limit { limit, offset: 0 }),
                                                },
                                            for_json: sql::ast::ForJson::NoJson,
                                        }
                                    ),
                                    alias: state.make_table_alias("foo".to_string()),
                                    alias_path: sql::ast::AliasPath{
                                        elements: Vec::new(),
                                    } }),
                joins: Vec::new(),
                where_: sql::ast::Where(sql::helpers::empty_where()),
                group_by: sql::helpers::empty_group_by(),
                // order_by: sql::ast::OrderBy{
                //     elements: vec![
                //         sql::ast::OrderByElement{
                //             target: sql::ast::Expression::Value(sql::ast::Value::Int8(1)),
                //             direction: sql::ast::OrderByDirection::Asc,
                //         }
                //     ]
                // },
                order_by: sql::helpers::empty_order_by(),
                // order_by: sql::helpers::empty_order_by(),
                for_json: sql::ast::ForJson::NoJson,
                // limit: 
                //     // Add the limit.
                //     match (query.limit, query.offset) {
                //         (None, None) => None,
                //         (limit, Some(offset)) => Some(sql::ast::Limit { limit, offset }),
                //         (limit, None) => Some(sql::ast::Limit { limit, offset: 0 }),
                //     }
                limit: None

            };
            Ok(sql::helpers::SelectSet::Rows(select))
        }
    }
}
