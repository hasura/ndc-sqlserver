//! This defines a `Connector` implementation for PostgreSQL.
//!
//! The routes are defined here.
//!
//! The relevant types for configuration and state are defined in
//! `super::configuration`.

use ndc_sdk::connector::LocatedError;
// use hyper::client::connect::Connect;
// use ndc_client::apis::configuration::Configuration;
use tiberius::Query;

use async_trait::async_trait;
use ndc_sdk::connector;
use ndc_sdk::json_response::JsonResponse;
use ndc_sdk::models;
use tokio::fs;

use super::configuration;
use super::explain;
use super::query;
use super::schema;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct SQLServer {}

pub const CONFIGURATION_FILENAME: &str = "configuration.json";
pub const CONFIGURATION_JSONSCHEMA_FILENAME: &str = "schema.json";

// pub struct SqlServerSetup<Env: Environment> {
//     environment: Env,
// }

// impl<Env: Environment> SqlServerSetup<Env> {
//     pub fn new(environment: Env) -> Self {
//         Self { environment }
//     }
// }

#[async_trait]
impl connector::ConnectorSetup for SQLServer {
    type Connector = SQLServer;

    /// Validate the raw configuration provided by the user,
    /// returning a configuration error or a validated [`Connector::Configuration`].
    async fn parse_configuration(
        &self,
        configuration_dir: impl AsRef<Path> + Send,
    ) -> Result<<Self::Connector as connector::Connector>::Configuration, connector::ParseError>
    {
        let configuration_file = configuration_dir.as_ref().join(CONFIGURATION_FILENAME);
        let configuration_file_contents =
            fs::read_to_string(&configuration_file)
                .await
                .map_err(|err| {
                    connector::ParseError::Other(
                        format!("{}: {}", &configuration_file.display(), err).into(),
                    )
                })?;
        let configuration: configuration::RawConfiguration =
            serde_json::from_str(&configuration_file_contents).map_err(|error| {
                connector::ParseError::ParseError(LocatedError {
                    file_path: configuration_file.clone(),
                    line: error.line(),
                    column: error.column(),
                    message: error.to_string(),
                })
            })?;

        configuration::validate_raw_configuration(configuration)
            .await
            .map(Arc::new)
    }

    /// Initialize the connector's in-memory state.
    ///
    /// For example, any connection pools, prepared queries,
    /// or other managed resources would be allocated here.
    ///
    /// In addition, this function should register any
    /// connector-specific metrics with the metrics registry.
    async fn try_init_state(
        &self,
        configuration: &<Self::Connector as connector::Connector>::Configuration,
        metrics: &mut prometheus::Registry,
    ) -> Result<<Self::Connector as connector::Connector>::State, connector::InitializationError>
    {
        configuration::create_state(configuration, metrics)
            .await
            .map(Arc::new)
            .map_err(|err| connector::InitializationError::Other(err.into()))
    }
}

#[async_trait]
impl connector::Connector for SQLServer {
    /// The type of validated configuration
    type Configuration = Arc<configuration::Configuration>;
    /// The type of unserializable state
    type State = Arc<configuration::State>;
    /// Update any metrics from the state
    ///
    /// Note: some metrics can be updated directly, and do not
    /// need to be updated here. This function can be useful to
    /// query metrics which cannot be updated directly, e.g.
    /// the number of idle connections in a connection pool
    /// can be polled but not updated directly.
    fn fetch_metrics(
        _configuration: &Self::Configuration,
        _state: &Self::State,
    ) -> Result<(), connector::FetchMetricsError> {
        // We'd call something `update_pool_metrics` here ideally, see SQLServer NDC

        Ok(())
    }

    /// Check the health of the connector.
    ///
    /// For example, this function should check that the connector
    /// is able to reach its data source over the network.
    async fn health_check(
        _configuration: &Self::Configuration,
        state: &Self::State,
    ) -> Result<(), connector::HealthError> {
        health_check_connect(state)
            .await
            .map_err(|e| connector::HealthError::Other(Box::new(e)))
    }

    /// Get the connector's capabilities.
    ///
    /// This function implements the [capabilities endpoint](https://hasura.github.io/ndc-spec/specification/capabilities.html)
    /// from the NDC specification.
    async fn get_capabilities() -> JsonResponse<models::CapabilitiesResponse> {
        JsonResponse::Value(models::CapabilitiesResponse {
            version: "^0.1.0".into(),
            capabilities: models::Capabilities {
                query: models::QueryCapabilities {
                    aggregates: Some(models::LeafCapability {}),
                    variables: Some(models::LeafCapability {}),
                    explain: Some(models::LeafCapability {}),
                },
                mutation: models::MutationCapabilities {
                    transactional: Some(models::LeafCapability {}),
                    explain: Some(models::LeafCapability {}),
                },
                relationships: Some(models::RelationshipCapabilities {
                    relation_comparisons: Some(models::LeafCapability {}),
                    order_by_aggregate: Some(models::LeafCapability {}),
                }),
            },
        })
    }

    /// Get the connector's schema.
    ///
    /// This function implements the [schema endpoint](https://hasura.github.io/ndc-spec/specification/schema/index.html)
    /// from the NDC specification.
    async fn get_schema(
        configuration: &Self::Configuration,
    ) -> Result<JsonResponse<models::SchemaResponse>, connector::SchemaError> {
        schema::get_schema(configuration).await.map(Into::into)
    }

    /// Explain a query by creating an execution plan
    ///
    /// This function implements the [explain endpoint](https://hasura.github.io/ndc-spec/specification/explain.html)
    /// from the NDC specification.
    async fn query_explain(
        configuration: &Self::Configuration,
        state: &Self::State,
        query_request: models::QueryRequest,
    ) -> Result<JsonResponse<models::ExplainResponse>, connector::ExplainError> {
        explain::explain(configuration, state, query_request)
            .await
            .map(JsonResponse::Value)
    }

    async fn mutation_explain(
        _configuration: &Self::Configuration,
        _state: &Self::State,
        _mutation_request: models::MutationRequest,
    ) -> Result<JsonResponse<models::ExplainResponse>, connector::ExplainError> {
        //TODO(PY): Implement mutation explain
        todo!("mutation explain is currently not implemented")
    }

    /// Execute a mutation
    ///
    /// This function implements the [mutation endpoint](https://hasura.github.io/ndc-spec/specification/mutations/index.html)
    /// from the NDC specification.
    async fn mutation(
        _configuration: &Self::Configuration,
        _state: &Self::State,
        _request: models::MutationRequest,
    ) -> Result<JsonResponse<models::MutationResponse>, connector::MutationError> {
        todo!("mutations are currently not implemented")
    }

    /// Execute a query
    ///
    /// This function implements the [query endpoint](https://hasura.github.io/ndc-spec/specification/queries/index.html)
    /// from the NDC specification.
    async fn query(
        configuration: &Self::Configuration,
        state: &Self::State,
        query_request: models::QueryRequest,
    ) -> Result<JsonResponse<models::QueryResponse>, connector::QueryError> {
        query::query(configuration, state, query_request).await
    }
}

// let's connect to our sql server and get the party started
async fn health_check_connect(state: &configuration::State) -> Result<(), tiberius::error::Error> {
    let mut connection = state.mssql_pool.get().await.unwrap();

    // let's do a query to check everything is ok
    let params = vec![String::from("hello"), String::from("world")];
    let mut select = Query::new("SELECT @P1, @P2");

    // bind parameters....
    for param in params.into_iter() {
        select.bind(param);
    }

    // go!
    let stream = select.query(&mut connection).await?;

    // Nothing is fetched, the first result set starts.
    let row = stream.into_row().await?.unwrap();

    let inner_result: Vec<Option<&str>> = vec![row.get(0), row.get(1)];

    // check we got the result we expected, panic if not!
    assert!(inner_result == vec![Some("hello"), Some("world")]);

    Ok(())
}
