//! Execute a Query execution plan against the database.

use bytes::{BufMut, Bytes, BytesMut};
use query_engine_metrics::metrics;

use crate::error::Error;
use query_engine_sql::sql;
use serde_json::Value;
use std::collections::BTreeMap;
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
pub(crate) async fn execute_query(
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
