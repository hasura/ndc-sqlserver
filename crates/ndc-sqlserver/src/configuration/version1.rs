//! Configuration and state for our connector.
use crate::metrics;

use crate::configuration::introspection;
use ndc_sdk::connector;
use query_engine_metadata::metadata::{database, Nullable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
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

    let decoded: Vec<introspection::IntrospectionTable> =
        serde_json::from_str(row.get(0).unwrap()).unwrap();

    let mut metadata = query_engine_metadata::metadata::Metadata::default();

    metadata.tables = get_tables_info(decoded);

    metadata.native_queries = configuration.metadata.native_queries.clone();

    Ok(RawConfiguration {
        version: 1,
        mssql_connection_string: configuration.mssql_connection_string.clone(),
        metadata,
    })
}

fn get_tables_info(
    introspection_tables: Vec<introspection::IntrospectionTable>,
) -> database::TablesInfo {
    let mut tables = BTreeMap::new();

    for introspection_table in introspection_tables {
        let table_name = introspection_table.name;

        let mut columns = BTreeMap::new();
        let mut foreign_relations_inner = BTreeMap::new();

        for introspection_column in introspection_table.joined_sys_column {
            let column_name = introspection_column.name.clone();
            let (column, new_foreign_relations) = get_column_info(introspection_column);
            columns.insert(column_name, column);
            foreign_relations_inner.extend(new_foreign_relations);
        }

        let table_info = database::TableInfo {
            columns,
            description: None,
            foreign_relations: database::ForeignRelations(foreign_relations_inner),
            table_name: table_name.clone(),
            schema_name: introspection_table.joined_sys_schema.name,
            uniqueness_constraints: get_uniqueness_constraints(
                introspection_table.joined_sys_primary_key,
            ),
        };

        tables.insert(table_name, table_info);
    }

    database::TablesInfo(tables)
}

fn get_foreign_relation(
    local_column: String,
    foreign_key: introspection::IntrospectionForeignKeyColumn,
) -> database::ForeignRelation {
    let mut column_mapping = BTreeMap::new();
    column_mapping.insert(local_column, foreign_key.joined_referenced_column_name);

    database::ForeignRelation {
        foreign_table: foreign_key.joined_referenced_table_name,
        column_mapping,
    }
}

fn get_uniqueness_constraints(
    opt_primary_key: Option<introspection::IntrospectionPrimaryKey>,
) -> database::UniquenessConstraints {
    let mut uniqueness_constraints_inner = BTreeMap::new();

    if let Some(primary_key) = opt_primary_key {
        let constraint_name = primary_key.name;

        let keys_set = primary_key
            .columns
            .iter()
            .fold(BTreeSet::new(), |mut set, part| {
                set.insert(part.name.clone());
                set
            });

        uniqueness_constraints_inner
            .insert(constraint_name, database::UniquenessConstraint(keys_set));
    }

    database::UniquenessConstraints(uniqueness_constraints_inner)
}

fn get_column_info(
    introspection_column: introspection::IntrospectionColumn,
) -> (
    database::ColumnInfo,
    BTreeMap<String, database::ForeignRelation>,
) {
    let mut foreign_relations = BTreeMap::new();
    introspection_column
        .joined_foreign_key_columns
        .into_iter()
        .enumerate()
        .for_each(|(index, foreign_key)| {
            let fancy_key = format!("FK_{}{}", introspection_column.name, index);
            foreign_relations.insert(
                fancy_key,
                get_foreign_relation(introspection_column.name.clone(), foreign_key),
            );
        });

    let column_info = database::ColumnInfo {
        description: None,
        name: introspection_column.name,
        nullable: if introspection_column.is_nullable {
            Nullable::Nullable
        } else {
            Nullable::NonNullable
        },
        r#type: database::ScalarType(introspection_column.joined_sys_type.name),
    };
    (column_info, foreign_relations)
}

/// State initialization error.
#[derive(Debug, Error)]
pub enum InitializationError {
    #[error("unable to initialize mssql connection pool: {0}")]
    UnableToCreateMSSQLPool(bb8_tiberius::Error),
    #[error("error initializing Prometheus metrics: {0}")]
    PrometheusError(prometheus::Error),
}
