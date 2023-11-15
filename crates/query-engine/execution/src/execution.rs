//! Execute an execution plan against the database.

use crate::metrics;
use ndc_sdk::models;
use query_engine_sql::sql;
use serde_json;
use std::collections::BTreeMap;
use tiberius::QueryItem;
use tokio_stream::StreamExt;

/// Execute a query against sqlserver.
pub async fn mssql_execute(
    mssql_pool: &bb8::Pool<bb8_tiberius::ConnectionManager>,
    metrics: &metrics::Metrics,
    plan: sql::execution_plan::ExecutionPlan,
) -> Result<models::QueryResponse, Error> {
    let query = plan.query();

    tracing::info!(
        "\nGenerated SQL: {}\nWith params: {:?}\nAnd variables: {:?}",
        query.sql,
        &query.params,
        &plan.variables,
    );

    let acquisition_timer = metrics.time_connection_acquisition_wait();
    let connection_result = mssql_pool.get().await.map_err(Error::ConnectionPool);
    let mut connection = acquisition_timer.complete_with(connection_result)?;

    let query_timer = metrics.time_query_execution();
    let rows_result = execute_query(&mut connection, plan).await;
    let rows = query_timer.complete_with(rows_result)?;

    tracing::info!("Database rows result: {:?}", rows);

    // Hack a response from the query results. See the 'response_hack' for more details.
    let response = rows_to_response(rows);

    // tracing::info!(
    //     "Query response: {}",
    //     serde_json::to_string(&response).unwrap()
    // );

    Ok(response)
}

async fn execute_query(
    connection: &mut bb8::PooledConnection<'_, bb8_tiberius::ConnectionManager>,
    plan: sql::execution_plan::ExecutionPlan,
) -> Result<Vec<serde_json::Value>, Error> {
    let query = plan.query();

    // run the query on each set of variables. The result is a vector of rows each
    // element in the vector is the result of running the query on one set of variables.
    let rows: Vec<serde_json::Value> = match plan.variables {
        None => {
            let empty_map = BTreeMap::new();
            let rows = execute_mssql_query(connection, &query, &empty_map).await?;
            vec![rows]
        }
        Some(variable_sets) => {
            let mut sets_of_rows = vec![];
            for vars in &variable_sets {
                let rows = execute_mssql_query(connection, &query, vars).await?;
                sets_of_rows.push(rows);
            }
            sets_of_rows
        }
    };
    Ok(rows)
}

/// Take the sqlserver results and return them as a QueryResponse.
fn rows_to_response(results: Vec<serde_json::Value>) -> models::QueryResponse {
    let rowsets = results
        .into_iter()
        .map(|raw_rowset| serde_json::from_value(raw_rowset).unwrap())
        .collect();

    models::QueryResponse(rowsets)
}

/// Execute the query on one set of variables.
async fn execute_mssql_query(
    connection: &mut bb8::PooledConnection<'_, bb8_tiberius::ConnectionManager>,
    query: &sql::string::SQL,
    variables: &BTreeMap<String, serde_json::Value>,
) -> Result<serde_json::Value, Error> {
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

    // collect big lump of json here
    let mut result_str = String::new();

    // stream it out and collect it here:
    while let Some(item) = stream.try_next().await.unwrap() {
        match item {
            // ignore these
            QueryItem::Metadata(_meta) => {
                // .. handling
            }
            // ...concatenate these
            QueryItem::Row(row) => {
                let item = row.get(0).unwrap();
                result_str.push_str(item)
            }
        }
    }

    // once we're happy this is stable, we should stream the JSON string straight out
    let json_value = serde_json::from_str(&result_str).unwrap();

    Ok(json_value)
}

pub enum Error {
    Query(String),
    ConnectionPool(bb8::RunError<bb8_tiberius::Error>),
}
