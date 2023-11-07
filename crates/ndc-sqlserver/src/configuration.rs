//! Configuration and state for our connector.
use super::metrics;

use ndc_sdk::connector;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tiberius::Query;

const TABLE_CONFIGURATION_QUERY: &str = include_str!("table_configuration.sql");

const CURRENT_VERSION: u32 = 1;

/// User configuration.
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct RawConfiguration {
    pub version: u32,
    pub mssql_connection_string: String,
    pub metadata: query_engine_metadata::metadata::Metadata,
}

impl RawConfiguration {
    pub fn empty() -> Self {
        Self {
            version: CURRENT_VERSION,
            mssql_connection_string: "".into(),
            metadata: query_engine_metadata::metadata::Metadata::default(),
        }
    }
}

/// User configuration, elaborated from a 'RawConfiguration'.
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub config: RawConfiguration,
}

/// State for our connector.
#[derive(Debug, Clone)]
pub struct State {
    pub mssql_pool: bb8::Pool<bb8_tiberius::ConnectionManager>,
    pub metrics: metrics::Metrics,
}

/// Validate the user configuration.
pub async fn validate_raw_configuration(
    config: RawConfiguration,
) -> Result<Configuration, connector::ValidateError> {
    if config.version != 1 {
        return Err(connector::ValidateError::ValidateError(vec![
            connector::InvalidRange {
                path: vec![connector::KeyOrIndex::Key("version".into())],
                message: format!(
                    "invalid configuration version, expected 1, got {0}",
                    config.version
                ),
            },
        ]));
    }
    Ok(Configuration { config })
}

/// Create a connection pool and wrap it inside a connector State.
pub async fn create_state(
    configuration: &Configuration,
    metrics_registry: &mut prometheus::Registry,
) -> Result<State, connector::InitializationError> {
    let mssql_pool = create_mssql_pool(&configuration.config)
        .await
        .map_err(|e| {
            connector::InitializationError::Other(
                InitializationError::UnableToCreateMSSQLPool(e).into(),
            )
        })?;
    let metrics = metrics::initialise_metrics(metrics_registry).await?;
    Ok(State {
        mssql_pool,
        metrics,
    })
}

/// Create a connection pool with default settings.
async fn create_mssql_pool(
    configuration: &RawConfiguration,
) -> Result<bb8::Pool<bb8_tiberius::ConnectionManager>, bb8_tiberius::Error> {
    let mut config = tiberius::Config::from_ado_string(&configuration.mssql_connection_string)?;

    // TODO: this is bad and we need to make TLS work properly before releasing this
    config.trust_cert();
    // TODO: LOOK UP LOOK UP

    let mgr = bb8_tiberius::ConnectionManager::new(config);

    bb8::Pool::builder().max_size(2).build(mgr).await
}

/// Construct the deployment configuration by introspecting the database.
pub async fn configure(
    configuration: &RawConfiguration,
) -> Result<RawConfiguration, connector::UpdateConfigurationError> {
    let mssql_pool = create_mssql_pool(configuration).await.unwrap();

    let mut connection = mssql_pool.get().await.unwrap();

    // let's do a query to check everything is ok
    let select = Query::new(TABLE_CONFIGURATION_QUERY);

    // go!
    let stream = select.query(&mut connection).await.unwrap();

    // Nothing is fetched, the first result set starts.
    let row = stream.into_row().await.unwrap().unwrap();

    let inner_result: Vec<&str> = vec![row.get(0).unwrap()];
    println!();

    println!("{:?}", inner_result);
    println!();

    Ok(RawConfiguration {
        version: 1,
        mssql_connection_string: configuration.mssql_connection_string.clone(),
        metadata: query_engine_metadata::metadata::Metadata::default(),
    })
}

struct IntrospectionTable {
    name: String,
    schema_id: i32,
    type_desc: String,
    joined_sys_schema: IntrospectionSchema,
    joined_sys_column: Vec<IntrospectionColumn>,
    joined_sys_primary_key: IntrospectionPrimaryKey,
}

struct IntrospectionColumn {
    name: String,
    column_id: i32,
    is_nullable: bool,
    is_identity: bool,
    is_computed: bool,
    user_type_id: i32,
    joined_sys_type: IntrospectionType,
    joined_foreign_key_columns: Vec<IntrospectionForeignKeyColumn>,
}

struct IntrospectionForeignKeyColumn {
    constraint_object_id: i32,
    constraint_column_id: i32,
    parent_object_id: i32,
    parent_column_id: i32,
    referenced_object_id: i32,
    referenced_column_id: i32,
    joined_referenced_table_name: String,
    joined_referenced_column_name: String,
    joined_referenced_sys_schema: IntrospectionSchema,
}

struct IntrospectionSchema {
    name: String,
    schema_id: i32,
}

/// State initialization error.
#[derive(Debug, Error)]
pub enum InitializationError {
    #[error("unable to initialize mssql connection pool: {0}")]
    UnableToCreateMSSQLPool(bb8_tiberius::Error),
    #[error("error initializing Prometheus metrics: {0}")]
    PrometheusError(prometheus::Error),
}
