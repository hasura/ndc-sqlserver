//! Configuration and state for our connector.

use super::introspection;
use crate::environment::Environment;
use crate::error::Error;
use crate::secret::Secret;
use query_engine_execution::metrics;
use query_engine_metadata::metadata;
use query_engine_metadata::metadata::{database, Nullable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use thiserror::Error;
use tiberius::Query;

const TABLE_CONFIGURATION_QUERY: &str = include_str!("table_configuration.sql");

const TYPES_QUERY: &str = "SELECT name FROM sys.types FOR JSON PATH";

const CURRENT_VERSION: u32 = 1;

const CHARACTER_STRINGS: [&str; 3] = ["char", "text", "varchar"];
const UNICODE_CHARACTER_STRINGS: [&str; 3] = ["nchar", "ntext", "nvarchar"];
const CANNOT_COMPARE: [&str; 3] = ["text", "ntext", "image"];
const EXACT_NUMERICS: [&str; 9] = [
    "bigint",
    "bit",
    "decimal",
    "int",
    "money",
    "numeric",
    "smallint",
    "smallmoney",
    "tinyint",
];
const APPROX_NUMERICS: [&str; 2] = ["float", "real"];
const NOT_COUNTABLE: [&str; 3] = ["image", "ntext", "text"];
const NOT_APPROX_COUNTABLE: [&str; 4] = ["image", "sql_variant", "ntext", "text"];

/// User configuration.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema, Clone)]
pub struct RawConfiguration {
    pub version: u32,
    pub mssql_connection_string: Secret,
    pub metadata: query_engine_metadata::metadata::Metadata,
}

impl RawConfiguration {
    pub fn empty() -> Self {
        Self {
            version: CURRENT_VERSION,
            mssql_connection_string: Secret::Plain("".into()),
            metadata: query_engine_metadata::metadata::Metadata::default(),
        }
    }

    pub fn with_mssql_connection_string(mssql_connection_string: Secret) -> Self {
        Self {
            version: CURRENT_VERSION,
            mssql_connection_string,
            metadata: query_engine_metadata::metadata::Metadata::default(),
        }
    }
}

/// User configuration, elaborated from a 'RawConfiguration'.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub struct Configuration {
    pub version: u32,
    pub mssql_connection_string: String,
    pub metadata: query_engine_metadata::metadata::Metadata,
}

/// State for our connector.
#[derive(Debug)]
pub struct State {
    pub mssql_pool: bb8::Pool<bb8_tiberius::ConnectionManager>,
    pub metrics: query_engine_execution::metrics::Metrics,
}

/// Validate the user configuration.
pub async fn validate_raw_configuration(
    file_path: &std::path::Path,
    config: RawConfiguration,
    environment: impl Environment,
) -> Result<Configuration, Error> {
    if config.version != 1 {
        return Err(Error::InvalidConfigVersion {
            version: config.version,
            file_path: file_path.to_path_buf(),
        });
    }
    let mssql_connection_string = match config.mssql_connection_string {
        Secret::Plain(s) => s,
        Secret::FromEnvironment { variable } => {
            environment
                .read(&variable)
                .map_err(|err| Error::MissingEnvironmentVariable {
                    file_path: file_path.to_path_buf(),
                    message: format!("{err}"),
                })?
        }
    };

    Ok(Configuration {
        version: config.version,
        mssql_connection_string,
        metadata: config.metadata,
    })
}

/// Create a connection pool and wrap it inside a connector State.
pub async fn create_state(
    configuration: &Configuration,
    metrics_registry: &mut prometheus::Registry,
) -> Result<State, InitializationError> {
    let mssql_pool = create_mssql_pool(configuration)
        .await
        .map_err(InitializationError::UnableToCreateMSSQLPool)?;
    let metrics = query_engine_execution::metrics::Metrics::initialize(metrics_registry)
        .map_err(InitializationError::MetricsError)?;
    Ok(State {
        mssql_pool,
        metrics,
    })
}

/// Create a connection pool with default settings.
async fn create_mssql_pool(
    configuration: &Configuration,
) -> Result<bb8::Pool<bb8_tiberius::ConnectionManager>, bb8_tiberius::Error> {
    let connection_string = configuration.mssql_connection_string.clone();
    let config = tiberius::Config::from_ado_string(&connection_string)?;

    let mgr = bb8_tiberius::ConnectionManager::new(config);

    bb8::Pool::builder().max_size(2).build(mgr).await
}

async fn select_first_row(
    mssql_pool: &bb8::Pool<bb8_tiberius::ConnectionManager>,
    query: &str,
) -> tiberius::Row {
    let mut connection = mssql_pool.get().await.unwrap();

    // let's do a query to check everything is ok
    let select = Query::new(query);

    // go!
    let stream = select.query(&mut connection).await.unwrap();

    // Nothing is fetched, the first result set starts.
    stream.into_row().await.unwrap().unwrap()
}

/// Construct the deployment configuration by introspecting the database.
pub async fn configure(configuration: &Configuration) -> Result<Configuration, Error> {
    let mssql_pool = create_mssql_pool(configuration).await.unwrap();

    let mut metadata = query_engine_metadata::metadata::Metadata::default();

    metadata.native_queries = configuration.metadata.native_queries.clone();

    let tables_row = select_first_row(&mssql_pool, TABLE_CONFIGURATION_QUERY).await;

    let tables: Vec<introspection::IntrospectionTable> =
        serde_json::from_str(tables_row.get(0).unwrap()).unwrap();

    metadata.tables = get_tables_info(tables);

    let types_row = select_first_row(&mssql_pool, TYPES_QUERY).await;

    let type_names: Vec<TypeItem> = serde_json::from_str(types_row.get(0).unwrap()).unwrap();

    metadata.comparison_operators = get_comparison_operators(&type_names).await;

    metadata.aggregate_functions = get_aggregate_functions(&type_names).await;

    Ok(Configuration {
        version: 1,
        mssql_connection_string: configuration.mssql_connection_string.clone(),
        metadata,
    })
}

#[derive(Deserialize, Debug)]
struct TypeItem {
    name: database::ScalarType,
}

// we lookup all types in sys.types, then use our hardcoded ideas about each one to attach
// aggregate functions
async fn get_aggregate_functions(type_names: &Vec<TypeItem>) -> database::AggregateFunctions {
    let mut aggregate_functions = BTreeMap::new();

    for type_name in type_names {
        aggregate_functions.insert(
            type_name.name.clone(),
            get_aggregate_functions_for_type(&type_name.name),
        );
    }
    database::AggregateFunctions(aggregate_functions)
}

// we hard code these, essentially
// we look up available types in `sys.types` but hard code their behaviour by looking them up below
// taken from https://learn.microsoft.com/en-us/sql/t-sql/functions/aggregate-functions-transact-sql?view=sql-server-ver16
fn get_aggregate_functions_for_type(
    type_name: &database::ScalarType,
) -> BTreeMap<String, database::AggregateFunction> {
    let mut aggregate_functions = BTreeMap::new();

    if !NOT_APPROX_COUNTABLE.contains(&type_name.0.as_str()) {
        aggregate_functions.insert(
            "APPROX_COUNT_DISTINCT".to_string(),
            database::AggregateFunction {
                return_type: metadata::ScalarType("bigint".to_string()),
            },
        );
    }

    if !NOT_COUNTABLE.contains(&type_name.0.as_str()) {
        aggregate_functions.insert(
            "COUNT".to_string(),
            database::AggregateFunction {
                return_type: metadata::ScalarType("int".to_string()),
            },
        );
    }

    if type_name.0.as_str() != "bit"
        && (EXACT_NUMERICS.contains(&type_name.0.as_str())
            || APPROX_NUMERICS.contains(&type_name.0.as_str())
            || CHARACTER_STRINGS.contains(&type_name.0.as_str())
            || type_name.0.as_str() == "datetime"
            || type_name.0.as_str() == "uniqueidentifier")
    {
        aggregate_functions.insert(
            "MIN".to_string(),
            database::AggregateFunction {
                return_type: type_name.clone(),
            },
        );
        aggregate_functions.insert(
            "MAX".to_string(),
            database::AggregateFunction {
                return_type: type_name.clone(),
            },
        );
    }

    if type_name.0.as_str() != "bit"
        && (EXACT_NUMERICS.contains(&type_name.0.as_str())
            || APPROX_NUMERICS.contains(&type_name.0.as_str()))
    {
        aggregate_functions.insert(
            "STDEV".to_string(),
            database::AggregateFunction {
                return_type: database::ScalarType("float".to_string()),
            },
        );
        aggregate_functions.insert(
            "STDEVP".to_string(),
            database::AggregateFunction {
                return_type: database::ScalarType("float".to_string()),
            },
        );
        aggregate_functions.insert(
            "VAR".to_string(),
            database::AggregateFunction {
                return_type: database::ScalarType("float".to_string()),
            },
        );
        aggregate_functions.insert(
            "VARP".to_string(),
            database::AggregateFunction {
                return_type: database::ScalarType("float".to_string()),
            },
        );
    }

    if let Some(precise_return_type) = match type_name.0.as_str() {
        "tinyint" => Some("int"),
        "smallint" => Some("int"),
        "int" => Some("int"),
        "bigint" => Some("bigint"),
        "decimal" => Some("decimal"),
        "money" => Some("money"),
        "smallmoney" => Some("money"),
        "float" => Some("float"),
        "real" => Some("float"),
        _ => None,
    } {
        aggregate_functions.insert(
            "AVG".to_string(),
            database::AggregateFunction {
                return_type: metadata::ScalarType(precise_return_type.to_string()),
            },
        );
        aggregate_functions.insert(
            "SUM".to_string(),
            database::AggregateFunction {
                return_type: metadata::ScalarType(precise_return_type.to_string()),
            },
        );
    };

    aggregate_functions.insert(
        "COUNT_BIG".to_string(),
        database::AggregateFunction {
            return_type: metadata::ScalarType("bigint".to_string()),
        },
    );

    aggregate_functions
}

// we lookup all types in sys.types, then use our hardcoded ideas about each one to attach
// comparison operators
async fn get_comparison_operators(type_names: &Vec<TypeItem>) -> database::ComparisonOperators {
    let mut comparison_operators = BTreeMap::new();

    for type_name in type_names {
        comparison_operators.insert(
            type_name.name.clone(),
            get_comparison_operators_for_type(&type_name.name),
        );
    }

    database::ComparisonOperators(comparison_operators)
}

// we hard code these, essentially
// we look up available types in `sys.types` but hard code their behaviour by looking them up below
// categories taken from https://learn.microsoft.com/en-us/sql/t-sql/data-types/data-types-transact-sql
fn get_comparison_operators_for_type(
    type_name: &database::ScalarType,
) -> BTreeMap<String, database::ComparisonOperator> {
    let mut comparison_operators = BTreeMap::new();

    // in ndc-spec, all things can be `==`
    comparison_operators.insert(
        "_eq".to_string(),
        database::ComparisonOperator {
            operator_name: "=".to_string(),
            argument_type: type_name.clone(),
            operator_kind: database::OperatorKind::Equal,
        },
    );

    comparison_operators.insert(
        "_in".to_string(),
        database::ComparisonOperator {
            operator_name: "IN".to_string(),
            argument_type: type_name.clone(),
            operator_kind: database::OperatorKind::In,
        },
    );

    // include LIKE and NOT LIKE for string-ish types
    if CHARACTER_STRINGS.contains(&type_name.0.as_str())
        || UNICODE_CHARACTER_STRINGS.contains(&type_name.0.as_str())
    {
        comparison_operators.insert(
            "_like".to_string(),
            database::ComparisonOperator {
                operator_name: "LIKE".to_string(),
                argument_type: type_name.clone(),
                operator_kind: database::OperatorKind::Custom,
            },
        );
        comparison_operators.insert(
            "_nlike".to_string(),
            database::ComparisonOperator {
                operator_name: "NOT LIKE".to_string(),
                argument_type: type_name.clone(),
                operator_kind: database::OperatorKind::Custom,
            },
        );
    }

    // include comparison operators for types that are comparable, according to
    // https://learn.microsoft.com/en-us/sql/t-sql/language-elements/comparison-operators-transact-sql?view=sql-server-ver16
    if !CANNOT_COMPARE.contains(&type_name.0.as_str()) {
        comparison_operators.insert(
            "_neq".to_string(),
            database::ComparisonOperator {
                operator_name: "!=".to_string(),
                argument_type: type_name.clone(),
                operator_kind: database::OperatorKind::Custom,
            },
        );
        comparison_operators.insert(
            "_lt".to_string(),
            database::ComparisonOperator {
                operator_name: "<".to_string(),
                argument_type: type_name.clone(),
                operator_kind: database::OperatorKind::Custom,
            },
        );
        comparison_operators.insert(
            "_gt".to_string(),
            database::ComparisonOperator {
                operator_name: ">".to_string(),
                argument_type: type_name.clone(),
                operator_kind: database::OperatorKind::Custom,
            },
        );

        comparison_operators.insert(
            "_gte".to_string(),
            database::ComparisonOperator {
                operator_name: ">=".to_string(),
                argument_type: type_name.clone(),
                operator_kind: database::OperatorKind::Custom,
            },
        );
        comparison_operators.insert(
            "_lte".to_string(),
            database::ComparisonOperator {
                operator_name: "<=".to_string(),
                argument_type: type_name.clone(),
                operator_kind: database::OperatorKind::Custom,
            },
        );
    }
    comparison_operators
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
    #[error("error initializing metrics: {0}")]
    MetricsError(metrics::Error),
}

/// Collect all the types that can occur in the metadata. This is a bit circumstantial. A better
/// approach is likely to record scalar type names directly in the metadata via configuration.sql.
pub fn occurring_scalar_types(metadata: &metadata::Metadata) -> BTreeSet<metadata::ScalarType> {
    let tables = &metadata.tables;
    let native_queries = &metadata.native_queries;
    let tables_column_types = tables
        .0
        .values()
        .flat_map(|v| v.columns.values().map(|c| c.r#type.clone()));

    let native_queries_column_types = native_queries
        .0
        .values()
        .flat_map(|v| v.columns.values().map(|c| c.r#type.clone()));

    let native_queries_arguments_types = native_queries
        .0
        .values()
        .flat_map(|v| v.arguments.values().map(|c| c.r#type.clone()));

    let aggregate_types = metadata
        .aggregate_functions
        .0
        .values()
        .flat_map(|v| v.values().map(|c| c.return_type.clone()));

    let comparison_operator_types = metadata
        .comparison_operators
        .0
        .values()
        .flat_map(|v| v.values().map(|c| c.argument_type.clone()));

    tables_column_types
        .chain(native_queries_column_types)
        .chain(native_queries_arguments_types)
        .chain(aggregate_types)
        .chain(comparison_operator_types)
        .collect::<BTreeSet<metadata::ScalarType>>()
}
