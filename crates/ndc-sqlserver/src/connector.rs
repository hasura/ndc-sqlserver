//! This defines a `Connector` implementation for PostgreSQL.
//!
//! The routes are defined here.
//!
//! The relevant types for configuration and state are defined in
//! `super::configuration`.

use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use configuration::environment::Environment;
use ndc_sdk::connector;
use ndc_sdk::connector::Result;
use ndc_sdk::json_response::JsonResponse;
use ndc_sdk::models;
use tokio::fs;

use super::explain;
use super::mutation;
use super::query;
use super::schema;
use ndc_sqlserver_configuration as configuration;

#[derive(Clone, Default)]
pub struct SQLServer {}

pub struct SQLServerSetup<Env: Environment> {
    environment: Env,
}

impl<Env: Environment> SQLServerSetup<Env> {
    pub fn new(environment: Env) -> Self {
        Self { environment }
    }
}

#[async_trait]
impl<Env: Environment + Send + Sync + 'static> connector::ConnectorSetup for SQLServerSetup<Env> {
    type Connector = SQLServer;

    /// Validate the raw configuration provided by the user,
    /// returning a configuration error or a validated [`Connector::Configuration`].
    async fn parse_configuration(
        &self,
        configuration_dir: &Path,
    ) -> Result<<Self::Connector as connector::Connector>::Configuration> {
        let configuration_file = configuration_dir.join(configuration::CONFIGURATION_FILENAME);
        let configuration_file_contents =
            fs::read_to_string(&configuration_file).await.map_err(|_| {
                connector::ParseError::CouldNotFindConfiguration(configuration_file.clone())
            })?;
        let configuration: configuration::RawConfiguration =
            serde_json::from_str(&configuration_file_contents).map_err(|error| {
                connector::ParseError::ParseError(connector::LocatedError {
                    file_path: configuration_file.clone(),
                    line: error.line(),
                    column: error.column(),
                    message: error.to_string(),
                })
            })?;

        let configuration = configuration::validate_raw_configuration(
            &configuration_file,
            configuration,
            &self.environment,
        )
        .await
        .map(Arc::new)
        .map_err(|error| match error {
            configuration::Error::ParseError {
                file_path,
                line,
                column,
                message,
            } => connector::ParseError::ParseError(connector::LocatedError {
                file_path,
                line,
                column,
                message,
            }),
            configuration::Error::InvalidConfigVersion { version, file_path } => {
                connector::ParseError::ValidateError(connector::InvalidNodes(vec![
                    connector::InvalidNode {
                        file_path,
                        node_path: vec![connector::KeyOrIndex::Key("version".into())],
                        message: format!(
                            "invalid configuration version, expected 1, got {version}",
                        ),
                    },
                ]))
            }
            configuration::Error::MissingEnvironmentVariable { file_path, message } => {
                connector::ParseError::ValidateError(connector::InvalidNodes(vec![
                    connector::InvalidNode {
                        file_path,
                        node_path: vec![connector::KeyOrIndex::Key("connectionUri".into())],
                        message,
                    },
                ]))
            }
            configuration::Error::IoError(inner) => connector::ParseError::IoError(inner),
            configuration::Error::IoErrorButStringified(inner) => {
                std::io::Error::other(inner).into()
            }
            configuration::Error::ConnectionPoolError(inner) => {
                std::io::Error::other(inner).into()
            }
            configuration::Error::GetConnectionFromPool(inner) => {
                std::io::Error::other(inner).into()
            }
            configuration::Error::JsonDeserializationError(inner) => {
                connector::ParseError::from(std::io::Error::other(inner))
            }
            configuration::Error::IntrospectionQueryExecutionError(inner) => {
                connector::ParseError::from(std::io::Error::other(inner))
            }
            configuration::Error::StoredProcedureIntrospectionError(inner) => {
                connector::ParseError::from(std::io::Error::other(inner))
            }
        })?;

        Ok(configuration)
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
    ) -> Result<<Self::Connector as connector::Connector>::State> {
        configuration::create_state(configuration, metrics)
            .await
            .map(Arc::new)
            .map_err(connector::ErrorResponse::from_error)
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
    fn fetch_metrics(_configuration: &Self::Configuration, _state: &Self::State) -> Result<()> {
        // We'd call something `update_pool_metrics` here ideally, see SQLServer NDC

        Ok(())
    }

    /// Get the connector's capabilities.
    ///
    /// This function implements the [capabilities endpoint](https://hasura.github.io/ndc-spec/specification/capabilities.html)
    /// from the NDC specification.
    async fn get_capabilities() -> models::Capabilities {
        crate::capabilities::get_capabilities()
    }

    /// Get the connector's schema.
    ///
    /// This function implements the [schema endpoint](https://hasura.github.io/ndc-spec/specification/schema/index.html)
    /// from the NDC specification.
    async fn get_schema(
        configuration: &Self::Configuration,
    ) -> Result<JsonResponse<models::SchemaResponse>> {
        schema::get_schema(configuration)
            .map_err(|err| {
                tracing::error!(
                    meta.signal_type = "log",
                    event.domain = "ndc",
                    event.name = "Schema error",
                    name = "Schema error",
                    body = %err,
                    error = true,
                    "Schema error",
                );
                err
            })
            .map(Into::into)
    }

    /// Explain a query by creating an execution plan
    ///
    /// This function implements the [explain endpoint](https://hasura.github.io/ndc-spec/specification/explain.html)
    /// from the NDC specification.
    async fn query_explain(
        configuration: &Self::Configuration,
        state: &Self::State,
        query_request: models::QueryRequest,
    ) -> Result<JsonResponse<models::ExplainResponse>> {
        explain::explain(configuration, state, query_request)
            .await
            .map(JsonResponse::Value)
    }

    async fn mutation_explain(
        _configuration: &Self::Configuration,
        _state: &Self::State,
        _mutation_request: models::MutationRequest,
    ) -> Result<JsonResponse<models::ExplainResponse>> {
        //TODO(PY): Implement mutation explain
        todo!("mutation explain is currently not implemented")
    }

    /// Execute a mutation
    ///
    /// This function implements the [mutation endpoint](https://hasura.github.io/ndc-spec/specification/mutations/index.html)
    /// from the NDC specification.
    async fn mutation(
        configuration: &Self::Configuration,
        state: &Self::State,
        request: models::MutationRequest,
    ) -> Result<JsonResponse<models::MutationResponse>> {
        mutation::mutation(configuration, state, request)
            .await
            .map_err(|err| {
                tracing::error!(
                    meta.signal_type = "log",
                    event.domain = "ndc",
                    event.name = "Mutation error",
                    name = "Mutation error",
                    body = %err,
                    error = true,
                    "Mutation error",
                );
                err
            })
    }

    /// Execute a query
    ///
    /// This function implements the [query endpoint](https://hasura.github.io/ndc-spec/specification/queries/index.html)
    /// from the NDC specification.
    async fn query(
        configuration: &Self::Configuration,
        state: &Self::State,
        query_request: models::QueryRequest,
    ) -> Result<JsonResponse<models::QueryResponse>> {
        query::query(configuration, state, query_request)
            .await
            .map_err(|err| {
                tracing::error!(
                    meta.signal_type = "log",
                    event.domain = "ndc",
                    event.name = "Query error",
                    name = "Query error",
                    body = %err,
                    error = true,
                    "Query error",
                );
                err
            })
    }
}
