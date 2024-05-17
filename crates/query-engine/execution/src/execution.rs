//! Execute an execution plan against the database.

use crate::metrics;
use bytes::{BufMut, Bytes, BytesMut};

use query_engine_sql::sql::{
    self,
    ast::With,
    execution_plan::{MutationExecutionPlan, NativeMutationExecutionPlan},
    string::SQL,
};
use query_engine_translation::translation::{
    helpers::{Env, State, TableNameAndReference},
    mutation::mutation::generate_native_mutation_response_cte,
    query::translate_query,
};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use thiserror::Error;
use tiberius::QueryItem;
use tokio_stream::StreamExt;
use tracing::{info_span, Instrument};

/// Execute a query against sqlserver.
pub async fn mssql_execute_query_plan(
    mssql_pool: &bb8::Pool<bb8_tiberius::ConnectionManager>,
    metrics: &metrics::Metrics,
    plan: sql::execution_plan::QueryExecutionPlan,
) -> Result<Bytes, Error> {
    let query = plan.query();

    tracing::info!(
        "\nGenerated SQL: {}\nWith params: {:?}\nAnd variables: {:?}",
        query.sql,
        &query.params,
        &plan.variables,
    );

    let acquisition_timer = metrics.time_connection_acquisition_wait();
    let connection_result = mssql_pool
        .get()
        .instrument(info_span!("Acquire connection"))
        .await
        .map_err(Error::ConnectionPool);
    let mut connection = acquisition_timer.complete_with(connection_result)?;

    let query_timer = metrics.time_query_execution();
    let bytes_result = execute_queries(&mut connection, plan)
        .instrument(info_span!("Database request"))
        .await;
    query_timer.complete_with(bytes_result)
}

pub async fn execute_mutations(
    mssql_pool: &bb8::Pool<bb8_tiberius::ConnectionManager>,
    metrics: &metrics::Metrics,
    plan: sql::execution_plan::MutationsExecutionPlan,
) -> Result<Bytes, Error> {
    // TODO: Run this in a transaction.

    let acquisition_timer = metrics.time_connection_acquisition_wait();
    let connection_result = mssql_pool
        .get()
        .instrument(info_span!("Acquire connection"))
        .await
        .map_err(Error::ConnectionPool);
    let mut connection = acquisition_timer.complete_with(connection_result)?;

    let query_timer = metrics.time_query_execution();

    // this buffer represents the JSON response
    let mut buffer = BytesMut::new();
    buffer.put(&[b'{'][..]); // we start by opening an object
    buffer.put(&b"\"operation_results\":"[..]); // specify the key for MutationResponse
    buffer.put(&[b'['][..]); // open the responses array

    let mut i = plan.mutations.into_iter();

    if let Some(mutation) = i.next() {
        let mutation_response = execute_mutation(&mut connection, mutation, &mut buffer).await;
    }

    buffer.put(&[b']'][..]); // we end by closing the array

    query_timer.complete_with(Ok(buffer.freeze()))
}

async fn execute_queries(
    connection: &mut bb8::PooledConnection<'_, bb8_tiberius::ConnectionManager>,
    plan: sql::execution_plan::QueryExecutionPlan,
) -> Result<Bytes, Error> {
    let query = plan.query();

    // this buffer represents the JSON response
    let mut buffer = BytesMut::new();
    buffer.put(&[b'['][..]); // we start by opening the array
    match plan.variables {
        None => {
            let empty_map = BTreeMap::new();
            execute_query(connection, &query, &empty_map, &mut buffer).await?;
        }
        Some(variable_sets) => {
            let mut i = variable_sets.iter();
            if let Some(first) = i.next() {
                execute_query(connection, &query, first, &mut buffer).await?;
                for vars in i {
                    buffer.put(&[b','][..]); // each result, except the first, is prefixed by a ','
                    execute_query(connection, &query, vars, &mut buffer).await?;
                }
            }
        }
    }
    buffer.put(&[b']'][..]); // we end by closing the array
    Ok(buffer.freeze())
}

/// Execute the query on one set of variables.
async fn execute_query(
    connection: &mut bb8::PooledConnection<'_, bb8_tiberius::ConnectionManager>,
    query: &sql::string::SQL,
    variables: &BTreeMap<String, serde_json::Value>,
    buffer: &mut (impl BufMut + Send),
) -> Result<(), Error> {
    let query_text = query.sql.as_str();

    let mut mssql_query = tiberius::Query::new(query_text);

    // bind parameters....
    for param in query.params.clone().into_iter() {
        match param {
            sql::string::Param::String(string) => {
                mssql_query.bind(string);
                Ok(())
            }
            sql::string::Param::Variable(var) => match variables.get(&var) {
                Some(value) => match value {
                    serde_json::Value::String(s) => {
                        mssql_query.bind(s);
                        Ok(())
                    }
                    serde_json::Value::Number(n) => {
                        mssql_query.bind(n.as_f64());
                        Ok(())
                    }
                    serde_json::Value::Bool(b) => {
                        mssql_query.bind(*b);
                        Ok(())
                    }
                    // this is a problem - we don't know the type of the value!
                    serde_json::Value::Null => Err(Error::Query(
                        "null variable not currently supported".to_string(),
                    )),
                    serde_json::Value::Array(_array) => Err(Error::Query(
                        "array variable not currently supported".to_string(),
                    )),
                    serde_json::Value::Object(_object) => Err(Error::Query(
                        "object variable not currently supported".to_string(),
                    )),
                },
                None => Err(Error::Query(format!("Variable not found '{}'", var))),
            },
        }?
    }

    // go!
    let mut stream = mssql_query.query(connection).await.unwrap();

    // stream it out and collect it here:
    while let Some(item) = stream.try_next().await.unwrap() {
        match item {
            // ignore these
            QueryItem::Metadata(_meta) => {
                // .. handling
            }
            // ...concatenate these
            QueryItem::Row(row) => {
                let item: &[u8] = row
                    .try_get(0)
                    .map_err(Error::TiberiusError)
                    .map(|item: Option<&str>| item.unwrap().as_bytes())?;
                buffer.put(item);
            }
        }
    }

    Ok(())
}

fn convert_mutation_response_to_json(db_results: &Vec<tiberius::Row>) -> Result<String, Error> {
    // Each element of the vector corresponds to the results of a single SQL statement.
    let mut db_results_json: Vec<HashMap<String, Option<serde_json::Value>>> = Vec::new();

    for row in db_results.into_iter() {
        let mut row_result_json = HashMap::new();

        let cols = row.columns().to_owned();

        for col in cols.into_iter() {
            match col.column_type() {
                tiberius::ColumnType::Int1 => {
                    let int_val = row
                        .try_get::<u8, &str>(col.name())
                        .map_err(Error::TiberiusError)?;
                    row_result_json.insert(
                        col.name().to_owned(),
                        int_val.map(|i| serde_json::Value::Number(i.into())),
                    );
                }

                tiberius::ColumnType::Int2 => {
                    let int_val = row
                        .try_get::<i16, &str>(col.name())
                        .map_err(Error::TiberiusError)?;
                    row_result_json.insert(
                        col.name().to_owned(),
                        int_val.map(|i| serde_json::Value::Number(i.into())),
                    );
                }

                tiberius::ColumnType::Int4 => {
                    let int_val = row
                        .try_get::<i32, &str>(col.name())
                        .map_err(Error::TiberiusError)?;
                    row_result_json.insert(
                        col.name().to_owned(),
                        int_val.map(|i| serde_json::Value::Number(i.into())),
                    );
                }

                tiberius::ColumnType::Int8 | tiberius::ColumnType::Intn => {
                    let int_val = row
                        .try_get::<i64, &str>(col.name())
                        .map_err(Error::TiberiusError)?;
                    row_result_json.insert(
                        col.name().to_owned(),
                        int_val.map(|i| serde_json::Value::Number(i.into())),
                    );
                }

                tiberius::ColumnType::Float4
                | tiberius::ColumnType::Float8
                | tiberius::ColumnType::Floatn => {
                    let float_val = row
                        .try_get::<f64, &str>(col.name())
                        .map_err(Error::TiberiusError)?;
                    let json_float_val = match float_val {
                        Some(f) => Some(serde_json::Number::from_f64(f).ok_or(Error::Mutation(
                            MutationError::NativeMutation {
                                column_name: col.name().to_string(),
                                column_type: col.column_type(),
                                error: NativeMutationResponseParseError::InvalidJSONFloatNumber,
                            },
                        ))?),
                        None => None,
                    };

                    row_result_json.insert(
                        col.name().to_owned(),
                        json_float_val.map(|f| serde_json::Value::Number(f)),
                    );
                }

                tiberius::ColumnType::NChar
                | tiberius::ColumnType::NVarchar
                | tiberius::ColumnType::NText
                | tiberius::ColumnType::BigVarChar => {
                    let string_val = row
                        .try_get::<&str, &str>(col.name())
                        .map_err(Error::TiberiusError)?;

                    row_result_json.insert(
                        col.name().to_owned(),
                        string_val.map(|s| serde_json::Value::String(s.to_string())),
                    );
                }

                tiberius::ColumnType::Datetime
                | tiberius::ColumnType::Daten
                | tiberius::ColumnType::Datetime2
                | tiberius::ColumnType::Datetime4
                | tiberius::ColumnType::DatetimeOffsetn
                | tiberius::ColumnType::Datetimen => {
                    let date_time_str = row
                        .try_get::<&str, &str>(col.name())
                        .map_err(Error::TiberiusError)?;
                    row_result_json.insert(
                        col.name().to_owned(),
                        date_time_str.map(|d| serde_json::Value::String(d.to_string())),
                    );
                }

                _ => {
                    return Err(Error::Mutation(MutationError::NativeMutation {
                        column_name: col.name().to_string(),
                        column_type: col.column_type(),
                        error: NativeMutationResponseParseError::UnknownType(col.column_type()),
                    }))
                }
            }
        }

        db_results_json.push(row_result_json);
    }

    let db_results_json = serde_json::to_string_pretty(&db_results_json)
        .map_err(|e| Error::Mutation(MutationError::JSONSerializationError(e)))?;

    println!("DB Results of the mutation are {}", db_results_json);

    Ok(db_results_json)
}

async fn execute_mutation(
    connection: &mut bb8::PooledConnection<'_, bb8_tiberius::ConnectionManager>,
    mutation_plan: MutationExecutionPlan,
    buffer: &mut (impl BufMut + Send),
) -> Result<(), Error> {
    match mutation_plan {
        MutationExecutionPlan::NativeMutation(native_mutation_plan) => {
            execute_native_mutation(connection, native_mutation_plan, buffer).await
        }
    }
}

/// Execute the mutation query.
async fn execute_native_mutation(
    connection: &mut bb8::PooledConnection<'_, bb8_tiberius::ConnectionManager>,
    native_mutation_plan: NativeMutationExecutionPlan,
    buffer: &mut (impl BufMut + Send),
) -> Result<(), Error> {
    let mutation_query = &native_mutation_plan.mutation_sql_query;

    let mut mssql_query = tiberius::Query::new(mutation_query.sql.as_str());

    // bind parameters....
    for param in mutation_query.params.clone().into_iter() {
        match param {
            sql::string::Param::String(string) => {
                mssql_query.bind(string);
            }
            // Variables are not used with mutations.
            sql::string::Param::Variable(_) => {}
        }
    }

    // go!
    let stream = mssql_query
        .query(connection)
        .await
        .map_err(Error::TiberiusError)?;

    let native_mutation_response = stream.into_results().await.map_err(Error::TiberiusError)?;

    // We expect each Native mutation to return exactly one row set.
    if native_mutation_response.len() > 1 {
        return Err(Error::Mutation(
            MutationError::NativeMutationMoreThanOneRowSet {
                native_mutation_name: native_mutation_plan.native_mutation_name.clone(),
            },
        ));
    }

    // Parse the response received from the DB and convert it into JSON to prepare
    // it for the next stage i.e. to select fields from the response obtained.
    let response_json =
        convert_mutation_response_to_json(native_mutation_response.first().unwrap_or(&Vec::new()))?;

    // TODO: Construct the response selection AST with the help of `native_mutation_response_json`
    // TODO: Execute the response selection query now.

    let mutation_response_cte = generate_native_mutation_response_cte(
        response_json,
        native_mutation_plan.response_selection.response_json_schema,
        native_mutation_plan
            .response_selection
            .response_cte_table_alias,
    );

    let mut response_selection_select = native_mutation_plan.response_selection.response_select;

    response_selection_select.with = With {
        common_table_expressions: vec![mutation_response_cte],
    };

    let mut response_selection_sql = SQL::new();

    response_selection_select.to_sql(&mut response_selection_sql);

    execute_query(
        connection,
        &response_selection_sql,
        &BTreeMap::new(),
        buffer,
    )
    .await?;

    Ok(())
}

/// Convert a query to an EXPLAIN query and execute it against postgres.
pub async fn explain(
    mssql_pool: &bb8::Pool<bb8_tiberius::ConnectionManager>,
    plan: sql::execution_plan::QueryExecutionPlan,
) -> Result<(String, String), Error> {
    let query = plan.query();

    tracing::info!(
        generated_sql = query.sql,
        params = ?&query.params,
        variables = ?&plan.variables,
    );

    let query_text = get_query_text(&query, plan.variables)?;

    let mut connection = mssql_pool.get().await.map_err(Error::ConnectionPool)?;

    let maybe_results: Result<Vec<String>, Error> =
        execute_explain(&mut connection, &query_text).await;

    // if we fail, make sure we turn off explain mode before giving up
    let results = match maybe_results {
        Ok(results) => Ok(results),
        Err(e) => {
            let _ = connection.simple_query("SET SHOWPLAN_TEXT OFF").await;
            Err(e)
        }
    }?;

    let pretty = sqlformat::format(
        &query.sql,
        &sqlformat::QueryParams::None,
        sqlformat::FormatOptions::default(),
    );

    Ok((pretty, results.join("\n")))
}

fn get_query_text(
    query: &sql::string::SQL,
    variables: Option<Vec<BTreeMap<String, Value>>>,
) -> Result<String, Error> {
    let empty_map = BTreeMap::new();
    let variable_sets = variables.unwrap_or_default();
    let variables = variable_sets.first().unwrap_or(&empty_map);

    let declarations =
        query
            .params
            .iter()
            .enumerate()
            .try_fold(String::new(), |str, (i, param)| {
                let type_name: String = match param {
                    sql::string::Param::String(_string) => Ok("VARCHAR(MAX)".to_string()),
                    sql::string::Param::Variable(var) => match &variables.get(var) {
                        Some(value) => match value {
                            serde_json::Value::String(_s) => Ok("VARCHAR(MAX)".to_string()),
                            serde_json::Value::Number(_n) => Ok("NUMERIC".to_string()),
                            serde_json::Value::Bool(_b) => Ok("TINYINT".to_string()),
                            // this is a problem - we don't know the type of the value!
                            serde_json::Value::Null => Err(Error::Query(
                                "null variable not currently supported".to_string(),
                            )),
                            serde_json::Value::Array(_array) => Err(Error::Query(
                                "array variable not currently supported".to_string(),
                            )),
                            serde_json::Value::Object(_object) => Err(Error::Query(
                                "object variable not currently supported".to_string(),
                            )),
                        },
                        None => Err(Error::Query(format!("Variable not found '{}'", var))),
                    },
                }?;

                Ok(format!("{} DECLARE @P{} {}; ", str, i + 1, type_name))
            })?;

    Ok(format!("{} {}", declarations, query.sql.as_str()))
}

// this is separated so we know where any ?s fall through to
async fn execute_explain(
    connection: &mut bb8::PooledConnection<'_, bb8_tiberius::ConnectionManager>,
    query_text: &str,
) -> Result<Vec<String>, Error> {
    let _ = connection.simple_query("SET SHOWPLAN_TEXT ON").await;

    let results = {
        let mut results: Vec<String> = vec![];

        // go!
        let mut stream = connection
            .simple_query(query_text)
            .await
            .map_err(Error::TiberiusError)?;

        // stream it out and collect it here:
        while let Some(item) = stream.try_next().await.map_err(Error::TiberiusError)? {
            match item {
                // ignore these
                QueryItem::Metadata(_meta) => {
                    // .. handling
                }
                // ...concatenate these
                QueryItem::Row(row) => {
                    let item: &str = row
                        .try_get(0)
                        .map_err(Error::TiberiusError)
                        .map(|item: Option<&str>| item.unwrap())?;
                    results.push(item.to_string());
                }
            }
        }
        results
    };

    let _ = connection.simple_query("SET SHOWPLAN_TEXT OFF").await;

    Ok(results)
}

#[derive(Debug, Error)]
pub enum NativeMutationResponseParseError {
    #[error("Unable to parse the float number, because it is not a valid JSON float number.")]
    InvalidJSONFloatNumber,
    #[error("Unable to parse response of type {0:#?}. HINT: Try casting the output of the column as as string in the native mutation SQL query.")]
    UnknownType(tiberius::ColumnType),
    #[error("Unable to parse response: {0}. HINT: Try casting the output of the column as a string in the native mutation SQL query.")]
    UnableToParseResponse(tiberius::error::Error),
}

#[derive(Debug, Error)]
pub enum MutationError {
    #[error("Error executing native mutation, column name: {column_name}, column type: {column_type:#?}, error: {error}")]
    NativeMutation {
        column_name: String,
        column_type: tiberius::ColumnType,
        error: NativeMutationResponseParseError,
    },
    #[error("The native mutation {native_mutation_name} is returning more than one set of rows. A native mutation statement is expected to return exactly one set of row set")]
    NativeMutationMoreThanOneRowSet { native_mutation_name: String },
    #[error("Error in serializing the native mutation response to JSON. Error: {0}")]
    JSONSerializationError(serde_json::Error),
    #[error("Error in translating the response selection query: {0}")]
    NativeMutationResponseSelectionError(query_engine_translation::translation::error::Error),
}

#[derive(Debug)]
pub enum Error {
    Query(String),
    ConnectionPool(bb8::RunError<bb8_tiberius::Error>),
    TiberiusError(tiberius::error::Error),
    Mutation(MutationError),
}
