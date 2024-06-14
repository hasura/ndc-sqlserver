//! Implement the `/query` endpoint to run a query against SQLServer.
//! See the Hasura
//! [Native Data Connector Specification](https://hasura.github.io/ndc-spec/specification/queries/index.html)
//! for further details.
use ndc_sdk::connector;
use ndc_sdk::json_response::JsonResponse;
use ndc_sdk::models;
use ndc_sqlserver_configuration as configuration;
use query_engine_execution::error;
use query_engine_execution::query;
use query_engine_sql::sql;
use query_engine_translation::translation;
use tracing::{info_span, Instrument};

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

    let timer = state.metrics.time_query_total();
    let result = async move {
        // Plan the query
        let plan = async { plan_query(configuration, state, query_request) }
            .instrument(info_span!("Plan query"))
            .await?;

        // Execute the query.
        let result = execute_query(state, plan)
            .instrument(info_span!("Execute query"))
            .await?;

        // assuming query succeeded, increment counter
        state.metrics.record_successful_query();

        Ok(result)
    }
    .instrument(info_span!("Execute query"))
    .await;

    timer.complete_with(result)
}

// Compile the query.
fn plan_query(
    configuration: &configuration::Configuration,
    state: &configuration::State,
    query_request: models::QueryRequest,
) -> Result<sql::execution_plan::QueryExecutionPlan, connector::QueryError> {
    let timer = state.metrics.time_query_plan();
    let result =
        translation::query::translate(&configuration.metadata, query_request).map_err(|err| {
            tracing::error!("{}", err);
            match err {
                translation::error::Error::NotSupported(_) => {
                    connector::QueryError::UnsupportedOperation(err.to_string())
                }
                _ => connector::QueryError::InvalidRequest(err.to_string()),
            }
        });
    timer.complete_with(result)
}

async fn execute_query(
    state: &configuration::State,
    plan: sql::execution_plan::QueryExecutionPlan,
) -> Result<JsonResponse<models::QueryResponse>, connector::QueryError> {
    query::mssql_execute_query_plan(&state.mssql_pool, &state.metrics, plan)
        .await
        .map(JsonResponse::Serialized)
        .map_err(|err| match err {
            error::Error::Query(err) => {
                tracing::error!("{}", err);
                connector::QueryError::Other(err.into())
            }
            error::Error::ConnectionPool(err) => {
                tracing::error!("{}", err);
                connector::QueryError::Other(err.into())
            }
            error::Error::TiberiusError(err) => {
                tracing::error!("{}", err);
                connector::QueryError::Other(err.into())
            }
            error::Error::Mutation(err) => {
                tracing::error!("{}", err);
                connector::QueryError::Other(err.into())
            }
        })
}
