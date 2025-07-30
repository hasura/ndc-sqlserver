//! Implement the `/mutation` endpoint to run a mutation statement against SQL Server.
//! See the Hasura
//! [Native Data Connector Specification](https://hasura.github.io/ndc-spec/specification/mutations/index.html)
//! for further details.

use ndc_sdk::{connector, json_response::JsonResponse, models};
use ndc_sqlserver_configuration as configuration;
use query_engine_execution::mutation;
use query_engine_sql::sql;
use query_engine_translation::translation;
use tracing::info_span;
use tracing::Instrument;

use crate::error::convert;
use crate::error::record;

/// Execute a mutation
///
/// This function implements the [mutation endpoint](https://hasura.github.io/ndc-spec/specification/mutations/index.html)
/// from the NDC specification.
pub async fn mutation(
    configuration: &configuration::Configuration,
    state: &configuration::State,
    request: models::MutationRequest,
) -> Result<JsonResponse<models::MutationResponse>, connector::ErrorResponse> {
    let timer = state.metrics.time_mutation_total();

    let result = async move {
        tracing::info!(
            request_json = serde_json::to_string(&request).unwrap(),
            request = ?request
        );
        let plan = async {
            plan_mutation(configuration, state, request).map_err(|err| {
                record::translation_error(&err, &state.metrics);
                convert::translation_error_to_response(&err)
            })
        }
        .instrument(info_span!("Execute Mutation"))
        .await?;

        let result = async {
            execute_mutation_plan(state, plan).await.map_err(|err| {
                record::execution_error(&err, &state.metrics);
                convert::execution_error_to_response(err)
            })
        }
        .instrument(info_span!("Execute mutation"))
        .await?;

        Ok(result)
    }
    .instrument(info_span!("Execute mutation"))
    .await;

    timer.complete_with(result)
}

/// Given the `configuration`, `state` and `mutation_request`, construct
/// the `MutationExecutionPlan`.
fn plan_mutation(
    configuration: &configuration::Configuration,
    state: &configuration::State,
    mutation_request: models::MutationRequest,
) -> Result<sql::execution_plan::MutationExecutionPlan, translation::error::Error> {
    let timer = state.metrics.time_mutation_plan();
    let result = translation::mutation::translate(&configuration.metadata, mutation_request);
    timer.complete_with(result)
}

/// Execute the `MutationExecutionPlan` and return the response back.
async fn execute_mutation_plan(
    state: &configuration::State,
    plan: sql::execution_plan::MutationExecutionPlan,
) -> Result<JsonResponse<models::MutationResponse>, query_engine_execution::error::Error> {
    mutation::execute_mutations(&state.mssql_pool, &state.metrics, plan)
        .await
        .map(JsonResponse::Serialized)
}
