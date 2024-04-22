//! Implement the `/schema` endpoint to return the connector's schema.
//! See the Hasura
//! [Native Data Connector Specification](https://hasura.github.io/ndc-spec/specification/schema/index.html)
//! for further details.

use std::collections::BTreeMap;

use ndc_sdk::connector;
use ndc_sdk::models;
use query_engine_metadata::metadata;

use super::configuration;

/// Extract the models::Type representation of a readonly column.
fn column_to_type(column: &metadata::ColumnInfo) -> models::Type {
    match &column.nullable {
        metadata::Nullable::NonNullable => models::Type::Named {
            name: column.r#type.0.clone(),
        },
        metadata::Nullable::Nullable => models::Type::Nullable {
            underlying_type: Box::new(models::Type::Named {
                name: column.r#type.0.clone(),
            }),
        },
    }
}

/// Build a `ProcedureInfo` type from the given parameters.
///
/// Because procedures return an `affected_rows` count alongside the result type that it's
/// `returning`, we have to generate a separate object type for its result. As part of that, we may
/// also have to include the `int4` scalar type (if it isn't included for another reason elsewhere
/// in the schema). So, this function creates that object type, optionally adds that scalar type,
/// and then returns a `ProcedureInfo` that points to the correct object type.
fn make_procedure_type(
    name: String,
    description: Option<String>,
    arguments: BTreeMap<String, models::ArgumentInfo>,
    result_type: models::Type,
    object_types: &mut BTreeMap<String, models::ObjectType>,
    scalar_types: &mut BTreeMap<String, models::ScalarType>,
) -> models::ProcedureInfo {
    let mut fields = BTreeMap::new();
    let object_type_name = format!("{name}_response");

    // If int4 doesn't exist anywhere else in the schema, we need to add it here. However, a user
    // can't filter or aggregate based on the affected rows of a procedure, so we don't need to add
    // any aggregate functions or comparison operators. However, if int4 exists elsewhere in the
    // schema and has already been added, it will also already contain these functions and
    // operators.
    scalar_types
        .entry("Int".to_string())
        .or_insert(models::ScalarType {
            //representation: Some(models::TypeRepresentation::Int32),
            aggregate_functions: BTreeMap::new(),
            comparison_operators: BTreeMap::new(),
        });

    fields.insert(
        "affected_rows".to_string(),
        models::ObjectField {
            description: Some("The number of rows affected by the mutation".to_string()),
            r#type: models::Type::Named {
                name: "Int".to_string(),
            },
        },
    );

    fields.insert(
        "returning".to_string(),
        models::ObjectField {
            description: Some("Data from rows affected by the mutation".to_string()),
            r#type: models::Type::Array {
                element_type: Box::from(result_type),
            },
        },
    );

    object_types.insert(
        object_type_name.clone(),
        models::ObjectType {
            description: Some(format!("Responses from the '{name}' procedure")),
            fields,
        },
    );

    models::ProcedureInfo {
        name,
        description,
        arguments,
        result_type: models::Type::Named {
            name: object_type_name,
        },
    }
}

/// Get the connector's schema.
///
/// This function implements the [schema endpoint](https://hasura.github.io/ndc-spec/specification/schema/index.html)
/// from the NDC specification.
pub async fn get_schema(
    configuration::Configuration { config, .. }: &configuration::Configuration,
) -> Result<models::SchemaResponse, connector::SchemaError> {
    let configuration::RawConfiguration { metadata, .. } = config;
    let mut scalar_types: BTreeMap<String, models::ScalarType> =
        configuration::occurring_scalar_types(&metadata.tables, &metadata.native_queries)
            .iter()
            .map(|scalar_type| {
                (
                    scalar_type.0.clone(),
                    models::ScalarType {
                        aggregate_functions: metadata
                            .aggregate_functions
                            .0
                            .get(scalar_type)
                            .unwrap_or(&BTreeMap::new())
                            .iter()
                            .map(|(function_name, function_definition)| {
                                (
                                    function_name.clone(),
                                    models::AggregateFunctionDefinition {
                                        result_type: models::Type::Named {
                                            name: function_definition.return_type.0.clone(),
                                        },
                                    },
                                )
                            })
                            .collect(),
                        comparison_operators: metadata
                            .comparison_operators
                            .0
                            .get(scalar_type)
                            .unwrap_or(&BTreeMap::new())
                            .iter()
                            .map(|(op_name, op_def)| {
                                (
                                    op_name.clone(),
                                    models::ComparisonOperatorDefinition {
                                        argument_type: models::Type::Named {
                                            name: op_def.argument_type.0.clone(),
                                        },
                                    },
                                )
                            })
                            .collect(),
                    },
                )
            })
            .collect();

    let tables: Vec<models::CollectionInfo> = metadata
        .tables
        .0
        .iter()
        .map(|(table_name, table)| models::CollectionInfo {
            name: table_name.clone(),
            description: table.description.clone(),
            arguments: BTreeMap::new(),
            collection_type: table_name.clone(),
            uniqueness_constraints: table
                .uniqueness_constraints
                .0
                .iter()
                .map(
                    |(constraint_name, metadata::UniquenessConstraint(constraint_columns))| {
                        (
                            constraint_name.clone(),
                            models::UniquenessConstraint {
                                unique_columns: constraint_columns.iter().cloned().collect(),
                            },
                        )
                    },
                )
                .collect(),
            foreign_keys: table
                .foreign_relations
                .0
                .iter()
                .map(
                    |(
                        constraint_name,
                        metadata::ForeignRelation {
                            foreign_table,
                            column_mapping,
                        },
                    )| {
                        (
                            constraint_name.clone(),
                            models::ForeignKeyConstraint {
                                foreign_collection: foreign_table.clone(),
                                column_mapping: column_mapping.clone(),
                            },
                        )
                    },
                )
                .collect(),
        })
        .collect();

    let native_queries_read_only: Vec<models::CollectionInfo> = metadata
        .native_queries
        .0
        .iter()
        .filter(|(_, info)| !info.is_procedure)
        .map(|(name, info)| models::CollectionInfo {
            name: name.clone(),
            description: info.description.clone(),
            arguments: info
                .arguments
                .iter()
                .map(|(name, column_info)| {
                    (
                        name.clone(),
                        models::ArgumentInfo {
                            description: column_info.description.clone(),
                            argument_type: column_to_type(column_info),
                        },
                    )
                })
                .collect(),
            collection_type: name.clone(),
            uniqueness_constraints: BTreeMap::new(),
            foreign_keys: BTreeMap::new(),
        })
        .collect();

    let table_types = BTreeMap::from_iter(metadata.tables.0.iter().map(|(table_name, table)| {
        let object_type = models::ObjectType {
            description: table.description.clone(),
            fields: BTreeMap::from_iter(table.columns.values().map(|column| {
                (
                    column.name.clone(),
                    models::ObjectField {
                        description: column.description.clone(),
                        r#type: column_to_type(column),
                    },
                )
            })),
        };
        (table_name.clone(), object_type)
    }));

    let native_queries_types =
        BTreeMap::from_iter(metadata.native_queries.0.iter().map(|(name, info)| {
            let object_type = models::ObjectType {
                description: info.description.clone(),
                fields: BTreeMap::from_iter(info.columns.values().map(|column| {
                    (
                        column.name.clone(),
                        models::ObjectField {
                            description: column.description.clone(),
                            r#type: column_to_type(column),
                        },
                    )
                })),
            };
            (name.clone(), object_type)
        }));

    let mut object_types = table_types;
    object_types.extend(native_queries_types);

    let native_queries_tracked_as_procedures: Vec<models::ProcedureInfo> = metadata
        .native_queries
        .0
        .iter()
        .filter(|(_, info)| info.is_procedure)
        .map(|(name, info)| {
            make_procedure_type(
                name.clone(),
                info.description.clone(),
                info.arguments
                    .iter()
                    .map(|(column_name, column_info)| {
                        (
                            column_name.clone(),
                            models::ArgumentInfo {
                                description: column_info.description.clone(),
                                argument_type: column_to_type(column_info),
                            },
                        )
                    })
                    .collect(),
                models::Type::Named { name: name.clone() },
                &mut object_types,
                &mut scalar_types,
            )
        })
        .collect();
    let mut collections = tables;
    collections.extend(native_queries_read_only);

    let procedures = native_queries_tracked_as_procedures;

    Ok(models::SchemaResponse {
        collections,
        procedures,
        functions: vec![],
        object_types,
        scalar_types,
    })
}
