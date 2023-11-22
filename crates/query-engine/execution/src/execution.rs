//! Execute an execution plan against the database.

use crate::metrics;
use bytes::{BufMut, Bytes, BytesMut};
use query_engine_sql::sql;
use serde_json;
use std::collections::BTreeMap;
use tiberius::QueryItem;
use tokio_stream::StreamExt;
use tracing::{info_span, Instrument};

/// Execute a query against sqlserver.
pub async fn mssql_execute(
    mssql_pool: &bb8::Pool<bb8_tiberius::ConnectionManager>,
    metrics: &metrics::Metrics,
    plan: sql::execution_plan::ExecutionPlan,
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
    plan: sql::execution_plan::ExecutionPlan,
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
    // let's do a query to check everything is ok
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

pub enum Error {
    Query(String),
    ConnectionPool(bb8::RunError<bb8_tiberius::Error>),
    TiberiusError(tiberius::error::Error),
}
