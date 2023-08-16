//! Configuration and state for our connector.

use super::metrics;
use ndc_hub::connector;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgConnection, PgPool, PgPoolOptions};
use sqlx::{Connection, Executor, Row};
use thiserror::Error;

const CURRENT_VERSION: u32 = 1;
const CONFIGURATION_QUERY: &str = include_str!("configuration.sql");

/// User configuration.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
pub struct DeploymentConfiguration {
    pub version: u32,
    pub postgres_database_url: String,
    pub tables: query_engine::metadata::TablesInfo,
    pub aggregate_functions: query_engine::metadata::AggregateFunctions,
}

impl DeploymentConfiguration {
    pub fn empty() -> Self {
        Self {
            version: CURRENT_VERSION,
            postgres_database_url: "".into(),
            tables: query_engine::metadata::TablesInfo::default(),
            aggregate_functions: query_engine::metadata::AggregateFunctions::default(),
        }
    }
}

/// State for our connector.
#[derive(Debug, Clone)]
pub struct State {
    pub pool: PgPool,
    pub mssql_pool: bb8::Pool<bb8_tiberius::ConnectionManager>,
    pub metrics: metrics::Metrics,
}

/// Validate the user configuration.
pub async fn validate_raw_configuration(
    configuration: &DeploymentConfiguration,
) -> Result<DeploymentConfiguration, connector::ValidateError> {
    if configuration.version != 1 {
        return Err(connector::ValidateError::ValidateError(vec![
            connector::InvalidRange {
                path: vec![connector::KeyOrIndex::Key("version".into())],
                message: format!(
                    "invalid configuration version, expected 1, got {0}",
                    configuration.version
                ),
            },
        ]));
    }
    Ok(configuration.clone())
}

/// Create a connection pool and wrap it inside a connector State.
pub async fn create_state(
    configuration: &DeploymentConfiguration,
    metrics_registry: &mut prometheus::Registry,
) -> Result<State, connector::InitializationError> {
    let pool = create_pool(configuration).await.map_err(|e| {
        connector::InitializationError::Other(InitializationError::UnableToCreatePool(e).into())
    })?;
    let mssql_pool = create_mssql_pool(configuration).await.map_err(|e| {
        connector::InitializationError::Other(
            InitializationError::UnableToCreateMSSQLPool(e).into(),
        )
    })?;
    let metrics = metrics::initialise_metrics(metrics_registry).await?;
    Ok(State {
        pool,
        mssql_pool,
        metrics,
    })
}

/// Create a connection pool with default settings.
async fn create_mssql_pool(
    _configuration: &DeploymentConfiguration,
) -> Result<bb8::Pool<bb8_tiberius::ConnectionManager>, bb8_tiberius::Error> {
    let connection_string =
        "DRIVER={ODBC Driver 18 for SQL Server};SERVER=127.0.0.1,64003;Uid=SA;Pwd=Password!";

    let mut config = tiberius::Config::from_ado_string(connection_string)?;

    // TODO: this is bad and we need to make TLS work properly before releasing this
    config.trust_cert();
    // TODO: LOOK UP LOOK UP

    let mgr = bb8_tiberius::ConnectionManager::new(config);

    bb8::Pool::builder().max_size(2).build(mgr).await
}

/// Create a connection pool with default settings.
async fn create_pool(configuration: &DeploymentConfiguration) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(50)
        .connect(&configuration.postgres_database_url)
        .await
}

/// Construct the deployment configuration by introspecting the database.
pub async fn configure(
    args: &DeploymentConfiguration,
) -> Result<DeploymentConfiguration, connector::UpdateConfigurationError> {
    let mut connection = PgConnection::connect(&args.postgres_database_url)
        .await
        .map_err(|e| connector::UpdateConfigurationError::Other(e.into()))?;

    let row = connection
        .fetch_one(CONFIGURATION_QUERY)
        .await
        .map_err(|e| connector::UpdateConfigurationError::Other(e.into()))?;

    let tables: query_engine::metadata::TablesInfo = serde_json::from_value(row.get(0))
        .map_err(|e| connector::UpdateConfigurationError::Other(e.into()))?;
    let aggregate_functions: query_engine::metadata::AggregateFunctions =
        serde_json::from_value(row.get(1))
            .map_err(|e| connector::UpdateConfigurationError::Other(e.into()))?;

    Ok(DeploymentConfiguration {
        version: 1,
        postgres_database_url: args.postgres_database_url.clone(),
        tables,
        aggregate_functions,
    })
}

/// State initialization error.
#[derive(Debug, Error)]
pub enum InitializationError {
    #[error("unable to initialize connection pool: {0}")]
    UnableToCreatePool(sqlx::Error),
    #[error("unable to initialize mssql connection pool: {0}")]
    UnableToCreateMSSQLPool(bb8_tiberius::Error),
    #[error("error initializing Prometheus metrics: {0}")]
    PrometheusError(prometheus::Error),
}
