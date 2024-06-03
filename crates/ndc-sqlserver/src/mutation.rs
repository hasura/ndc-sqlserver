//! Implement the `/mutation` endpoint to run a mutation statement against SQL Server.
//! See the Hasura
//! [Native Data Connector Specification](https://hasura.github.io/ndc-spec/specification/mutations/index.html)
//! for further details.

use ndc_sdk::{connector, json_response::JsonResponse, models};
use ndc_sqlserver_configuration as configuration;
use query_engine_execution::error;
use query_engine_execution::mutation;
use query_engine_sql::sql;
use query_engine_translation::translation;
use tracing::info_span;
use tracing::Instrument;

/// Execute a mutation
///
/// This function implements the [mutation endpoint](https://hasura.github.io/ndc-spec/specification/mutations/index.html)
/// from the NDC specification.
pub async fn mutation(
    configuration: &configuration::Configuration,
    state: &configuration::State,
    request: models::MutationRequest,
) -> Result<JsonResponse<models::MutationResponse>, connector::MutationError> {
    let timer = state.metrics.time_mutation_total();

    let result = async move {
        tracing::info!(
            request_json = serde_json::to_string(&request).unwrap(),
            request = ?request
        );
        let plan = async { plan_mutation(configuration, state, request) }
            .instrument(info_span!("Execute Mutation"))
            .await?;

        let result = async {
            execute_mutations_plan(state, plan)
                .instrument(info_span!("Execute mutation"))
                .await
        }
        .await?;

        Ok(result)
    }
    .instrument(info_span!("/mutation"))
    .await;

    timer.complete_with(result)
}

fn plan_mutation(
    configuration: &configuration::Configuration,
    state: &configuration::State,
    mutation_request: models::MutationRequest,
) -> Result<sql::execution_plan::MutationExecutionPlan, connector::MutationError> {
    let timer = state.metrics.time_mutation_plan();
    let result = translation::mutation::translate(&configuration.config.metadata, mutation_request)
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
    plan: sql::execution_plan::MutationExecutionPlan,
) -> Result<JsonResponse<models::MutationResponse>, connector::MutationError> {
    mutation::execute_mutations(&state.mssql_pool, &state.metrics, plan)
        .await
        .map(JsonResponse::Serialized)
        .map_err(|err| match err {
            error::Error::Query(err) => {
                tracing::error!("{}", err);
                connector::MutationError::Other(err.into())
            }
            error::Error::ConnectionPool(err) => {
                tracing::error!("{}", err);
                connector::MutationError::Other(err.into())
            }
            error::Error::TiberiusError(err) => {
                tracing::error!("{}", err);
                connector::MutationError::Other(err.into())
            }
            error::Error::Mutation(err) => {
                tracing::error!("{}", err);
                connector::MutationError::Other(err.into())
            }
        })
}
