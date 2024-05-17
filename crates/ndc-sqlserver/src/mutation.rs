//! Implement the `/mutation` endpoint to run a mutation against SQLServer.

use super::configuration;
use ndc_sdk::{connector, json_response::JsonResponse, models};
use query_engine_execution::execution;
use query_engine_sql::sql;
use query_engine_translation::translation;

pub async fn mutation(
    configuration: &configuration::Configuration,
    state: &configuration::State,
    mutation_request: models::MutationRequest,
) -> Result<JsonResponse<models::MutationResponse>, connector::MutationError> {
    tracing::info!("{}", serde_json::to_string(&mutation_request).unwrap());
    tracing::info!("{:?}", mutation_request);

    let timer = state.metrics.time_query_total();

    let plan = plan_mutation(configuration, state, mutation_request)?;

    let result = execute_mutations_plan(state, plan).await;

    timer.complete_with(result)
}

fn plan_mutation(
    configuration: &configuration::Configuration,
    state: &configuration::State,
    mutation_request: models::MutationRequest,
) -> Result<sql::execution_plan::MutationsExecutionPlan, connector::MutationError> {
    let timer = state.metrics.time_query_plan();
    let result = translation::mutation::mutation::translate(
        &configuration.config.metadata,
        mutation_request,
    )
    .map_err(|err| {
        tracing::error!("{}", err);
        match err {
            translation::error::Error::NotSupported(_) => {
                connector::MutationError::UnsupportedOperation(err.to_string())
            }
            _ => connector::MutationError::InvalidRequest(err.to_string()),
        }
    });
    timer.complete_with(result)
}

async fn execute_mutations_plan(
    state: &configuration::State,
    plan: sql::execution_plan::MutationsExecutionPlan,
) -> Result<JsonResponse<models::MutationResponse>, connector::MutationError> {
    execution::execute_mutations(&state.mssql_pool, &state.metrics, plan)
        .await
        .map(JsonResponse::Serialized)
        .map_err(|err| match err {
            execution::Error::Query(err) => {
                tracing::error!("{}", err);
                connector::MutationError::Other(err.into())
            }
            execution::Error::ConnectionPool(err) => {
                tracing::error!("{}", err);
                connector::MutationError::Other(err.into())
            }
            execution::Error::TiberiusError(err) => {
                tracing::error!("{}", err);
                connector::MutationError::Other(err.into())
            }
            execution::Error::Mutation(err) => {
                tracing::error!("{}", err);
                connector::MutationError::Other(err.into())
            }
        })
}
