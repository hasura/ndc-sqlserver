//! Configuration and state for our connector.

use super::introspection;
use crate::environment::Environment;
use crate::error::Error;
use crate::secret::Secret;
use crate::{uri, ConnectionUri};

use ndc_models::{AggregateFunctionName, CollectionName, ComparisonOperatorName, FieldName};
use query_engine_execution::query::execute_query;
use query_engine_metadata::metadata;
use query_engine_metadata::metadata::stored_procedures::{
    StoredProcedureArgumentInfo, StoredProcedureInfo, StoredProcedures,
};
use query_engine_metadata::metadata::{database, Nullable};
use query_engine_metrics::metrics;
use query_engine_sql::sql::{ast::RawSql, string::SQL};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use thiserror::Error;

// TODO(KC): Move the `table_configuration.sql` to the `static` folder present
// in the root of this repo.
const TABLE_CONFIGURATION_QUERY: &str = include_str!("table_configuration.sql");

const STORED_PROCS_CONFIGURATION_QUERY: &str =
    include_str!("../../../static/introspect_stored_procedures.sql");

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
    pub mssql_connection_string: ConnectionUri,
    pub metadata: query_engine_metadata::metadata::Metadata,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema, Clone)]
pub struct StoredProceduresConfigurationOptions {
    pub overwrite_existing_stored_procedures: bool,
}

impl RawConfiguration {
    pub fn empty() -> Self {
        Self {
            version: CURRENT_VERSION,
            mssql_connection_string: uri::ConnectionUri(Secret::FromEnvironment {
                variable: crate::DEFAULT_CONNECTION_URI_VARIABLE.into(),
            }),
            metadata: query_engine_metadata::metadata::Metadata::default(),
        }
    }

    pub fn with_mssql_connection_string(mssql_connection_string: Secret) -> Self {
        Self {
            version: CURRENT_VERSION,
            mssql_connection_string: uri::ConnectionUri(mssql_connection_string),
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
    pub metrics: query_engine_metrics::metrics::Metrics,
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
        uri::ConnectionUri(Secret::Plain(s)) => s,
        uri::ConnectionUri(Secret::FromEnvironment { variable }) => environment
            .read(&variable)
            .map_err(|err| Error::MissingEnvironmentVariable {
                file_path: file_path.to_path_buf(),
                message: format!("{err}"),
            })?,
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
    let mssql_pool = create_mssql_pool(&configuration.mssql_connection_string)
        .await
        .map_err(InitializationError::UnableToCreateMSSQLPool)?;
    let metrics = query_engine_metrics::metrics::Metrics::initialize(metrics_registry)
        .map_err(InitializationError::MetricsError)?;
    Ok(State {
        mssql_pool,
        metrics,
    })
}

// If the connection string is ODBC we want to throw an error.
fn is_odbc_connection_string(conn_str: &str) -> Result<(), bb8_tiberius::Error> {
    if conn_str.contains("Driver=") || conn_str.contains("DSN=") {
        Err(bb8_tiberius::Error::Tiberius(tiberius::error::Error::Io {
            kind: std::io::ErrorKind::Other,
            message: "ODBC connection strings are not supported. ADO.NET is the supported format."
                .into(),
        }))
    } else {
        Ok(())
    }
}

/// Create a connection pool with default settings.
async fn create_mssql_pool(
    configuration: &str,
) -> Result<bb8::Pool<bb8_tiberius::ConnectionManager>, bb8_tiberius::Error> {
    let connection_string = configuration.to_owned();
    // Lets check the string and error early if it is an ODBC connection string
    is_odbc_connection_string(&connection_string)?;
    let config = tiberius::Config::from_ado_string(&connection_string)?;
    let mgr = bb8_tiberius::ConnectionManager::new(config);

    bb8::Pool::builder().max_size(2).build(mgr).await
}

// get_stored_procedures fetches the stored procedures from the database and returns them as a
// vector of introspection::IntrospectStoredProcedure.
async fn configure_stored_procedures(
    mssql_pool: &bb8::Pool<bb8_tiberius::ConnectionManager>,
    existing_stored_procedures: StoredProcedures,
    config_options: Option<StoredProceduresConfigurationOptions>,
) -> Result<StoredProcedures, Error> {
    match config_options {
        Some(config_options) => {
            let mut connection = mssql_pool
                .get()
                .await
                .map_err(Error::GetConnectionFromPool)?;
            // Let's do some stored procedures introspection
            let mut stored_procs_query = SQL::new();
            RawSql::RawText(STORED_PROCS_CONFIGURATION_QUERY.to_string())
                .to_sql(&mut stored_procs_query);
            let mut stored_procs_rows = Vec::new();
            execute_query(
                &mut connection,
                &stored_procs_query,
                &BTreeMap::new(),
                &mut stored_procs_rows,
            )
            .await
            .map_err(|e| Error::IntrospectionQueryExecutionError(format!("{:?}", e)))?;
            let introspected_stored_procedures: Vec<introspection::IntrospectStoredProcedure> =
                serde_json::from_slice(&stored_procs_rows)
                    .map_err(|e| Error::JsonDeserializationError(e.to_string()))?;
            let new_stored_procedures = get_stored_procedures(introspected_stored_procedures);

            // traverse the new stored procedures and add them to the existing stored procedures
            let mut merged_stored_procedures = existing_stored_procedures.0.clone();
            for (name, stored_procedure) in new_stored_procedures.0 {
                // check if the stored procedure already exists
                match merged_stored_procedures.entry(name) {
                    std::collections::btree_map::Entry::Occupied(mut e) => {
                        if config_options.overwrite_existing_stored_procedures {
                            e.insert(stored_procedure);
                        } else {
                            // do not overwrite the existing stored procedure
                            continue;
                        }
                    }
                    std::collections::btree_map::Entry::Vacant(e) => {
                        e.insert(stored_procedure);
                    }
                }
            }

            Ok(StoredProcedures(merged_stored_procedures))
        }
        None => Ok(existing_stored_procedures),
    }
}

/// Construct the deployment configuration by introspecting the database.
pub async fn configure(
    file_path: &std::path::Path,
    configuration: &RawConfiguration,
    environment: impl Environment,
    stored_procedure_configuration_options: Option<StoredProceduresConfigurationOptions>,
) -> Result<RawConfiguration, Error> {
    let connection_string = match &configuration.mssql_connection_string {
        uri::ConnectionUri(Secret::Plain(s)) => s.to_owned(),
        uri::ConnectionUri(Secret::FromEnvironment { variable }) => environment
            .read(variable)
            .map_err(|err| Error::MissingEnvironmentVariable {
                file_path: file_path.to_path_buf(),
                message: format!("{err}"),
            })?,
    };
    let mssql_pool = create_mssql_pool(&connection_string)
        .await
        .map_err(Error::ConnectionPoolError)?;

    let mut connection = mssql_pool
        .get()
        .await
        .map_err(Error::GetConnectionFromPool)?;

    // Let's do some table introspection
    let mut table_query = SQL::new();
    RawSql::RawText(TABLE_CONFIGURATION_QUERY.to_string()).to_sql(&mut table_query);
    let mut table_rows = Vec::new();
    execute_query(
        &mut connection,
        &table_query,
        &BTreeMap::new(),
        &mut table_rows,
    )
    .await
    .map_err(|e| Error::IntrospectionQueryExecutionError(format!("{:?}", e)))?;
    let tables: Vec<introspection::IntrospectionTable> = serde_json::from_slice(&table_rows)
        .map_err(|e| Error::JsonDeserializationError(e.to_string()))?;

    // Let's do some types introspection
    let mut types_query = SQL::new();
    RawSql::RawText(TYPES_QUERY.to_string()).to_sql(&mut types_query);
    let mut types_rows = Vec::new();
    execute_query(
        &mut connection,
        &types_query,
        &BTreeMap::new(),
        &mut types_rows,
    )
    .await
    .map_err(|e| Error::IntrospectionQueryExecutionError(format!("{:?}", e)))?;
    let type_names: Vec<TypeItem> = serde_json::from_slice(&types_rows)
        .map_err(|e| Error::JsonDeserializationError(e.to_string()))?;

    let mut metadata = query_engine_metadata::metadata::Metadata::default();
    metadata.native_queries = configuration.metadata.native_queries.clone();
    metadata.native_mutations = configuration.metadata.native_mutations.clone();
    metadata.tables = get_tables_info(tables);
    metadata.comparison_operators = get_comparison_operators(&type_names);
    metadata.aggregate_functions = get_aggregate_functions(&type_names);
    metadata.stored_procedures = configure_stored_procedures(
        &mssql_pool,
        configuration.metadata.stored_procedures.clone(),
        stored_procedure_configuration_options,
    )
    .await?;

    Ok(RawConfiguration {
        version: 1,
        mssql_connection_string: configuration.mssql_connection_string.clone(),
        metadata,
    })
}

#[derive(Deserialize, Debug)]
struct TypeItem {
    name: database::ScalarType,
}

fn get_stored_procedures(
    introspected_stored_procedures: Vec<introspection::IntrospectStoredProcedure>,
) -> query_engine_metadata::metadata::stored_procedures::StoredProcedures {
    let mut metadata_stored_procs = BTreeMap::new();
    for stored_procedure in introspected_stored_procedures.into_iter() {
        let metadata_stored_procedure = StoredProcedureInfo {
            name: stored_procedure.name.clone(),
            schema: stored_procedure.schema,
            arguments: stored_procedure
                .arguments
                .into_iter()
                .map(|sp| -> (String, StoredProcedureArgumentInfo) {
                    (
                        sp.name.clone(),
                        StoredProcedureArgumentInfo {
                            name: sp.name,
                            r#type: query_engine_metadata::metadata::ScalarType(sp.r#type),
                            nullable: if sp.is_nullable {
                                Nullable::Nullable
                            } else {
                                Nullable::NonNullable
                            },
                            is_output: sp.is_output,
                            description: None,
                        },
                    )
                })
                .collect(),
            returns: None,
            description: None,
        };
        metadata_stored_procs.insert(stored_procedure.name, metadata_stored_procedure);
    }
    StoredProcedures(metadata_stored_procs)
}

// we lookup all types in sys.types, then use our hardcoded ideas about each one to attach
// aggregate functions
fn get_aggregate_functions(type_names: &Vec<TypeItem>) -> database::AggregateFunctions {
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
) -> BTreeMap<AggregateFunctionName, database::AggregateFunction> {
    let mut aggregate_functions = BTreeMap::new();

    if !NOT_APPROX_COUNTABLE.contains(&type_name.0.as_str()) {
        aggregate_functions.insert(
            AggregateFunctionName::new("APPROX_COUNT_DISTINCT".to_string().into()),
            database::AggregateFunction {
                return_type: metadata::ScalarType("bigint".to_string()),
            },
        );
    }

    if !NOT_COUNTABLE.contains(&type_name.0.as_str()) {
        aggregate_functions.insert(
            AggregateFunctionName::new("COUNT".to_string().into()),
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
            AggregateFunctionName::new("MIN".to_string().into()),
            database::AggregateFunction {
                return_type: type_name.clone(),
            },
        );
        aggregate_functions.insert(
            AggregateFunctionName::new("MAX".to_string().into()),
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
            AggregateFunctionName::new("STDEV".to_string().into()),
            database::AggregateFunction {
                return_type: database::ScalarType("float".to_string()),
            },
        );
        aggregate_functions.insert(
            AggregateFunctionName::new("STDEVP".to_string().into()),
            database::AggregateFunction {
                return_type: database::ScalarType("float".to_string()),
            },
        );
        aggregate_functions.insert(
            AggregateFunctionName::new("VAR".to_string().into()),
            database::AggregateFunction {
                return_type: database::ScalarType("float".to_string()),
            },
        );
        aggregate_functions.insert(
            AggregateFunctionName::new("VARP".to_string().into()),
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
            AggregateFunctionName::new("AVG".to_string().into()),
            database::AggregateFunction {
                return_type: metadata::ScalarType(precise_return_type.to_string()),
            },
        );
        aggregate_functions.insert(
            AggregateFunctionName::new("SUM".to_string().into()),
            database::AggregateFunction {
                return_type: metadata::ScalarType(precise_return_type.to_string()),
            },
        );
    };

    aggregate_functions.insert(
        AggregateFunctionName::new("COUNT_BIG".to_string().into()),
        database::AggregateFunction {
            return_type: metadata::ScalarType("bigint".to_string()),
        },
    );

    aggregate_functions
}

// we lookup all types in sys.types, then use our hardcoded ideas about each one to attach
// comparison operators
fn get_comparison_operators(type_names: &Vec<TypeItem>) -> database::ComparisonOperators {
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
) -> BTreeMap<ComparisonOperatorName, database::ComparisonOperator> {
    let mut comparison_operators = BTreeMap::new();

    // in ndc-spec, all things can be `==`
    comparison_operators.insert(
        ComparisonOperatorName::new("_eq".to_string().into()),
        database::ComparisonOperator {
            operator_name: "=".to_string(),
            argument_type: type_name.clone(),
            operator_kind: database::OperatorKind::Equal,
        },
    );

    comparison_operators.insert(
        ComparisonOperatorName::new("_in".to_string().into()),
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
            ComparisonOperatorName::new("_like".to_string().into()),
            database::ComparisonOperator {
                operator_name: "LIKE".to_string(),
                argument_type: type_name.clone(),
                operator_kind: database::OperatorKind::Custom,
            },
        );
        comparison_operators.insert(
            ComparisonOperatorName::new("_nlike".to_string().into()),
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
            ComparisonOperatorName::new("_neq".to_string().into()),
            database::ComparisonOperator {
                operator_name: "!=".to_string(),
                argument_type: type_name.clone(),
                operator_kind: database::OperatorKind::Custom,
            },
        );
        comparison_operators.insert(
            ComparisonOperatorName::new("_lt".to_string().into()),
            database::ComparisonOperator {
                operator_name: "<".to_string(),
                argument_type: type_name.clone(),
                operator_kind: database::OperatorKind::Custom,
            },
        );
        comparison_operators.insert(
            ComparisonOperatorName::new("_gt".to_string().into()),
            database::ComparisonOperator {
                operator_name: ">".to_string(),
                argument_type: type_name.clone(),
                operator_kind: database::OperatorKind::Custom,
            },
        );

        comparison_operators.insert(
            ComparisonOperatorName::new("_gte".to_string().into()),
            database::ComparisonOperator {
                operator_name: ">=".to_string(),
                argument_type: type_name.clone(),
                operator_kind: database::OperatorKind::Custom,
            },
        );
        comparison_operators.insert(
            ComparisonOperatorName::new("_lte".to_string().into()),
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
            columns.insert(FieldName::new(column_name.into()), column);
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

        tables.insert(CollectionName::new(table_name.into()), table_info);
    }

    database::TablesInfo(tables)
}

fn get_foreign_relation(
    local_column: String,
    foreign_key: introspection::IntrospectionForeignKeyColumn,
) -> database::ForeignRelation {
    let mut column_mapping = BTreeMap::new();
    column_mapping.insert(
        local_column.into(),
        foreign_key.joined_referenced_column_name.into(),
    );

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
                set.insert(part.name.clone().into());
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

    // TODO(KC): include types of the native mutations

    let stored_procedures_argument_types = metadata
        .stored_procedures
        .0
        .values()
        .flat_map(|v| v.arguments.values().map(|c| c.r#type.clone()));

    let stored_procedures_column_types = metadata
        .stored_procedures
        .0
        .values()
        .filter_map(|v| v.returns.as_ref())
        .flat_map(|v| v.values().map(|c| c.r#type.clone()));

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
        .chain(stored_procedures_argument_types)
        .chain(stored_procedures_column_types)
        .chain(aggregate_types)
        .chain(comparison_operator_types)
        .collect::<BTreeSet<metadata::ScalarType>>()
}
