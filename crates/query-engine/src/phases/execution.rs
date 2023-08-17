//! Execute an execution plan against the database.

use std::collections::BTreeMap;

use serde_json;

use ndc_hub::models;

use super::translation::sql;

/// Execute a query against postgres.
pub async fn mssql_execute(
    mssql_pool: &bb8::Pool<bb8_tiberius::ConnectionManager>,
    plan: sql::execution_plan::ExecutionPlan,
) -> Result<models::QueryResponse, Error> {
    let query = plan.query();

    tracing::info!(
        "\nGenerated SQL: {}\nWith params: {:?}\nAnd variables: {:?}",
        query.sql,
        &query.params,
        &plan.variables,
    );

    // run the query on each set of variables. The result is a vector of rows each
    // element in the vector is the result of running the query on one set of variables.
    let rows: Vec<serde_json::Value> = match plan.variables {
        None => {
            let empty_map = BTreeMap::new();
            let rows = execute_mssql_query(mssql_pool, &query, &empty_map).await?;
            vec![rows]
        }
        Some(variable_sets) => {
            let mut sets_of_rows = vec![];
            for vars in &variable_sets {
                let rows = execute_mssql_query(mssql_pool, &query, vars).await?;
                sets_of_rows.push(rows);
            }
            sets_of_rows
        }
    };

    tracing::info!("Database rows result: {:?}", rows);

    // Hack a response from the query results. See the 'response_hack' for more details.
    let response = rows_to_response(rows);

    // tracing::info!(
    //     "Query response: {}",
    //     serde_json::to_string(&response).unwrap()
    // );

    Ok(response)
}

/// Take the postgres results and return them as a QueryResponse.
fn rows_to_response(results: Vec<serde_json::Value>) -> models::QueryResponse {
    let rowsets = results
        .into_iter()
        .map(|raw_rowset| serde_json::from_value(raw_rowset).unwrap())
        .collect();

    models::QueryResponse(rowsets)
}

/// Execute the query on one set of variables.
async fn execute_mssql_query(
    mssql_pool: &bb8::Pool<bb8_tiberius::ConnectionManager>,
    query: &sql::string::SQL,
    _variables: &BTreeMap<String, serde_json::Value>,
) -> Result<serde_json::Value, Error> {
    let mut connection = mssql_pool.get().await.unwrap();

    // let's do a query to check everything is ok
    let query_text = query.sql.as_str();

    let mut mssql_query = tiberius::Query::new(query_text);

    // bind parameters....
    for param in query.params.clone().into_iter() {
        mssql_query.bind(param);
    }

    // go!
    let stream = mssql_query.query(&mut connection).await.unwrap();

    // Nothing is fetched, the first result set starts.
    let rows = stream.into_row().await.unwrap().unwrap();

    let single_item: &str = rows.get(0).unwrap();

    let json_value = serde_json::from_str(single_item).unwrap();

    // build query
    //    let tiberius_query = build_mssql_query_with_params(query, variables).await?;

    Ok(json_value)
}

impl tiberius::IntoSql<'_> for sql::string::Param {
    fn into_sql(self) -> tiberius::ColumnData<'static> {
        match self {
            sql::string::Param::String(string) => string.into_sql(),
            sql::string::Param::Variable(var) => var.into_sql(),
        }
    }
}

pub enum Error {
    Query(String),
}
