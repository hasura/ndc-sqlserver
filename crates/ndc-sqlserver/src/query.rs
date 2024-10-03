//! Implement the `/query` endpoint to run a query against SQLServer.
//! See the Hasura
//! [Native Data Connector Specification](https://hasura.github.io/ndc-spec/specification/queries/index.html)
//! for further details.
use crate::error::convert;
use crate::error::record;
use ndc_sdk::connector;
use ndc_sdk::json_response::JsonResponse;
use ndc_sdk::models;
use ndc_sqlserver_configuration as configuration;
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
) -> Result<JsonResponse<models::QueryResponse>, connector::ErrorResponse> {
    tracing::info!("{}", serde_json::to_string(&query_request).unwrap());
    tracing::info!("{:?}", query_request);

    let timer = state.metrics.time_query_total();
    let result = async move {
        // Plan the query
        let plan = async {
            plan_query(configuration, state, query_request).map_err(|err| {
                record::translation_error(&err, &state.metrics);
                convert::translation_error_to_response(&err)
            })
        }
        .instrument(info_span!("Plan query"))
        .await?;

        // Execute the query.
        let result = async {
            execute_query(state, plan).await.map_err(|err| {
                record::execution_error(&err, &state.metrics);
                convert::execution_error_to_response(err)
            })
        }
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
) -> Result<sql::execution_plan::QueryExecutionPlan, translation::error::Error> {
    let timer = state.metrics.time_query_plan();
    let result = translation::query::translate(&configuration.metadata, query_request);
    timer.complete_with(result)
}

async fn execute_query(
    state: &configuration::State,
    plan: sql::execution_plan::QueryExecutionPlan,
) -> Result<JsonResponse<models::QueryResponse>, query_engine_execution::error::Error> {
    query::mssql_execute_query_plan(&state.mssql_pool, &state.metrics, plan)
        .await
        .map(JsonResponse::Serialized)
}
