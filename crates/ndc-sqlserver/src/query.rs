//! Implement the `/query` endpoint to run a query against SQLServer.
//! See the Hasura
//! [Native Data Connector Specification](https://hasura.github.io/ndc-spec/specification/queries/index.html)
//! for further details.

use super::configuration;
use ndc_sdk::connector;
use ndc_sdk::json_response::JsonResponse;
use ndc_sdk::models;
use query_engine_execution::execution;
use query_engine_translation::translation;

/// Execute a query
///
/// This function implements the [query endpoint](https://hasura.github.io/ndc-spec/specification/queries/index.html)
/// from the NDC specification.
pub async fn query(
    configuration: &configuration::Configuration,
    state: &configuration::State,
    query_request: models::QueryRequest,
) -> Result<JsonResponse<models::QueryResponse>, connector::QueryError> {
    tracing::info!("{}", serde_json::to_string(&query_request).unwrap());
    tracing::info!("{:?}", query_request);

    // Compile the query.
    let plan = match translation::query::translate(&configuration.config.metadata, query_request) {
        Ok(plan) => Ok(plan),
        Err(err) => {
            tracing::error!("{}", err);
            match err {
                translation::query::error::Error::NotSupported(_) => {
                    Err(connector::QueryError::UnsupportedOperation(err.to_string()))
                }
                _ => Err(connector::QueryError::InvalidRequest(err.to_string())),
            }
        }
    }?;

    // Execute the query.
    let result = execution::mssql_execute(&state.mssql_pool, &state.metrics, plan)
        .await
        .map_err(|err| match err {
            execution::Error::Query(err) => {
                tracing::error!("{}", err);
                connector::QueryError::Other(err.into())
            }
            execution::Error::ConnectionPool(err) => {
                tracing::error!("{}", err);
                connector::QueryError::Other(err.into())
            }
        })?;

    // assuming query succeeded, increment counter
    state.metrics.record_successful_query();

    // TODO: return raw JSON
    Ok(JsonResponse::Value(result))
}
