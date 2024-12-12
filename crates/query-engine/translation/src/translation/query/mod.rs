//! Translate an incoming `QueryRequest`.

mod aggregates;
mod filtering;
mod native_queries;
mod relationships;
mod root;
mod sorting;

use super::error::Error;
use super::helpers::{CollectionOrProcedureInfo, Env, State, TableNameAndReference};

use query_engine_metadata::metadata;
use query_engine_sql::sql;

/// Translate the incoming QueryRequest to an ExecutionPlan (SQL) to be run against the database.
pub fn translate(
    metadata: &metadata::Metadata,
    query_request: ndc_models::QueryRequest,
) -> Result<sql::execution_plan::QueryExecutionPlan, Error> {
    let env = Env::new(metadata, query_request.collection_relationships);
    let mut state = State::new();
    let table_alias = state.make_table_alias(query_request.collection.to_string());
    let (current_table, from_clause) = root::make_from_clause_and_reference(
        &query_request.collection,
        &query_request.arguments,
        &env,
        &mut state,
        &table_alias,
    )?;

    let select_set = translate_query(
        &env,
        &mut state,
        &env.lookup_collection(&query_request.collection)?,
        &current_table,
        &from_clause,
        &query_request.query,
        &table_alias,
    )?;

    // form a single JSON item shaped `{ rows: [], aggregates: {} }`
    // that matches the ndc_models::RowSet type
    let mut json_select = sql::helpers::select_rowset(
        state.make_table_alias("universe".to_string()),
        state.make_table_alias("rows".to_string()),
        sql::helpers::make_column_alias("rows".to_string()),
        state.make_table_alias("aggregates".to_string()),
        sql::helpers::make_column_alias("aggregates".to_string()),
        select_set,
    );

    // add native queries if there are any
    json_select.with = sql::ast::With {
        common_table_expressions: native_queries::translate_native_queries(state)?,
    };

    // normalize ast
    let json_select = sql::rewrites::constant_folding::normalize_select(json_select);

    Ok(sql::execution_plan::simple_exec_plan(
        query_request.variables,
        query_request.collection.to_string(),
        json_select,
    ))
}

/// Translate a query to sql ast.
/// We return a SELECT for the 'rows' field and a SELECT for the 'aggregates' field.
pub fn translate_query(
    env: &Env,
    state: &mut State,
    collection_info: &CollectionOrProcedureInfo,

    current_table: &TableNameAndReference,
    from_clause: &sql::ast::From,
    query: &ndc_models::Query,
    table_alias: &sql::ast::TableAlias,
) -> Result<sql::helpers::SelectSet, Error> {
    // translate rows query. if there are no fields, make this a None
    let row_select = root::translate_rows_query(
        env,
        state,
        collection_info,
        current_table,
        from_clause,
        query,
        table_alias,
    )?;

    // translate aggregate select. if there are no fields, make this a None
    let aggregate_select =
        root::translate_aggregate_query(env, state, collection_info, current_table, from_clause, query)?;

    match (row_select, aggregate_select) {
        (Some(rows), None) => Ok(sql::helpers::SelectSet::Rows(rows)),
        (None, Some(aggregates)) => Ok(sql::helpers::SelectSet::Aggregates(aggregates)),
        (Some(rows), Some(aggregates)) => {
            Ok(sql::helpers::SelectSet::RowsAndAggregates(rows, aggregates))
        }
        (None, None) => Err(Error::NoFieldsAndAggregates),
    }
}
