//! This defines a `Connector` implementation for PostgreSQL.
//!
//! The routes are defined here.
//!
//! The relevant types for configuration and state are defined in
//! `super::configuration`.

use tiberius::Query;

use async_trait::async_trait;
use ndc_hub::connector;
use ndc_hub::models;

use query_engine::phases;

use super::configuration;
use super::metrics;

#[derive(Clone, Default)]
pub struct Postgres {}

#[async_trait]
impl connector::Connector for Postgres {
    /// RawConfiguration is what the user specifies as JSON
    type RawConfiguration = configuration::DeploymentConfiguration;
    /// The type of validated configuration
    type Configuration = configuration::DeploymentConfiguration;
    /// The type of unserializable state
    type State = configuration::State;

    fn make_empty_configuration() -> Self::RawConfiguration {
        configuration::DeploymentConfiguration::empty()
    }

    /// Configure a configuration maybe?
    async fn update_configuration(
        args: &Self::RawConfiguration,
    ) -> Result<configuration::DeploymentConfiguration, connector::UpdateConfigurationError> {
        configuration::configure(args).await
    }

    /// Validate the raw configuration provided by the user,
    /// returning a configuration error or a validated [`Connector::Configuration`].
    async fn validate_raw_configuration(
        configuration: &Self::RawConfiguration,
    ) -> Result<Self::Configuration, connector::ValidateError> {
        configuration::validate_raw_configuration(configuration).await
    }

    /// Initialize the connector's in-memory state.
    ///
    /// For example, any connection pools, prepared queries,
    /// or other managed resources would be allocated here.
    ///
    /// In addition, this function should register any
    /// connector-specific metrics with the metrics registry.
    async fn try_init_state(
        configuration: &Self::Configuration,
        metrics: &mut prometheus::Registry,
    ) -> Result<Self::State, connector::InitializationError> {
        configuration::create_state(configuration, metrics).await
    }

    /// Update any metrics from the state
    ///
    /// Note: some metrics can be updated directly, and do not
    /// need to be updated here. This function can be useful to
    /// query metrics which cannot be updated directly, e.g.
    /// the number of idle connections in a connection pool
    /// can be polled but not updated directly.
    fn fetch_metrics(
        _configuration: &configuration::DeploymentConfiguration,
        state: &configuration::State,
    ) -> Result<(), connector::FetchMetricsError> {
        metrics::update_pool_metrics(&state.pool, &state.metrics);

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
    async fn get_capabilities() -> models::CapabilitiesResponse {
        let empty = serde_json::to_value(()).unwrap();
        models::CapabilitiesResponse {
            versions: "^0.0.0".into(),
            capabilities: models::Capabilities {
                explain: Some(empty.clone()),
                query: Some(models::QueryCapabilities {
                    foreach: Some(empty.clone()),
                    order_by_aggregate: None,
                    relation_comparisons: None,
                }),
                relationships: Some(empty),
                mutations: None,
            },
        }
    }

    /// Get the connector's schema.
    ///
    /// This function implements the [schema endpoint](https://hasura.github.io/ndc-spec/specification/schema/index.html)
    /// from the NDC specification.
    async fn get_schema(
        _configuration: &Self::Configuration,
    ) -> Result<models::SchemaResponse, connector::SchemaError> {
        todo!("get_schema")
    }

    /// Explain a query by creating an execution plan
    ///
    /// This function implements the [explain endpoint](https://hasura.github.io/ndc-spec/specification/explain.html)
    /// from the NDC specification.
    async fn explain(
        _configuration: &Self::Configuration,
        _state: &Self::State,
        _query_request: models::QueryRequest,
    ) -> Result<models::ExplainResponse, connector::ExplainError> {
        todo!("explain!")
    }

    /// Execute a mutation
    ///
    /// This function implements the [mutation endpoint](https://hasura.github.io/ndc-spec/specification/mutations/index.html)
    /// from the NDC specification.
    async fn mutation(
        _configuration: &Self::Configuration,
        _state: &Self::State,
        _request: models::MutationRequest,
    ) -> Result<models::MutationResponse, connector::MutationError> {
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
    ) -> Result<models::QueryResponse, connector::QueryError> {
        tracing::info!("{}", serde_json::to_string(&query_request).unwrap());
        tracing::info!("{:?}", query_request);

        // Compile the query.
        let plan = match phases::translation::query::translate(&configuration.tables, query_request)
        {
            Ok(plan) => Ok(plan),
            Err(err) => {
                tracing::error!("{}", err);
                match err {
                    phases::translation::query::error::Error::NotSupported(_) => {
                        Err(connector::QueryError::UnsupportedOperation(err.to_string()))
                    }
                    _ => Err(connector::QueryError::InvalidRequest(err.to_string())),
                }
            }
        }?;

        // Execute the query.
        let result = phases::execution::mssql_execute(&state.mssql_pool, plan)
            .await
            .map_err(|err| match err {
                phases::execution::Error::Query(err) => {
                    tracing::error!("{}", err);
                    connector::QueryError::Other(err.into())
                }
                phases::execution::Error::DB(err) => {
                    tracing::error!("{}", err);
                    connector::QueryError::Other(err.to_string().into())
                }
            })?;

        // assuming query succeeded, increment counter
        state.metrics.query_total.inc();

        Ok(result)
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
