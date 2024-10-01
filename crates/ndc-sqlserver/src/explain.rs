//! Implement the `/explain` endpoint to return a query execution plan.
//! See the Hasura
//! [Native Data Connector Specification](https://hasura.github.io/ndc-spec/specification/explain.html)
//! for further details.

use std::collections::BTreeMap;

use tracing::{info_span, Instrument};

use ndc_sdk::connector;
use ndc_sdk::models;
use ndc_sqlserver_configuration as configuration;
use query_engine_execution::query;
use query_engine_sql::sql;
use query_engine_translation::translation;

use crate::error::convert;
use crate::error::record;

/// Explain a query by creating an execution plan
///
/// This function implements the [explain endpoint](https://hasura.github.io/ndc-spec/specification/explain.html)
/// from the NDC specification.
pub async fn explain(
    configuration: &configuration::Configuration,
    state: &configuration::State,
    query_request: models::QueryRequest,
) -> Result<models::ExplainResponse, connector::ErrorResponse> {
    async move {
        tracing::info!(
            query_request_json = serde_json::to_string(&query_request).unwrap(),
            query_request = ?query_request
        );

        // Compile the query.
        let plan = async {
            plan_query(configuration, state, query_request).map_err(|err| {
                record::translation_error(&err, &state.metrics);
                convert::translation_error_to_response(&err)
            })
        }
        .instrument(info_span!("Plan query"))
        .await?;

        // Execute an explain query.
        let (query, plan) = async {
            query::explain(&state.mssql_pool, plan)
                .await
                .map_err(|err| {
                    record::execution_error(&err, &state.metrics);
                    convert::execution_error_to_response(err)
                })
        }
        .instrument(info_span!("Explain query"))
        .await?;

        state.metrics.record_successful_explain();

        let details =
            BTreeMap::from_iter([("SQL Query".into(), query), ("Execution Plan".into(), plan)]);

        let response = models::ExplainResponse { details };

        Ok(response)
    }
    .instrument(info_span!("/explain"))
    .await
}

fn plan_query(
    configuration: &configuration::Configuration,
    state: &configuration::State,
    query_request: models::QueryRequest,
) -> Result<sql::execution_plan::QueryExecutionPlan, translation::error::Error> {
    let timer = state.metrics.time_query_plan();
    let result = translation::query::translate(&configuration.metadata, query_request);
    timer.complete_with(result)
}
