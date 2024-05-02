//! Implement the `/schema` endpoint to return the connector's schema.
//! See the Hasura
//! [Native Data Connector Specification](https://hasura.github.io/ndc-spec/specification/schema/index.html)
//! for further details.

use std::collections::BTreeMap;

use ndc_sdk::connector;
use ndc_sdk::models;
use ndc_sdk::models::TypeRepresentation;
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
/// also have to include the `int` scalar type (if it isn't included for another reason elsewhere
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

    // If int doesn't exist anywhere else in the schema, we need to add it here. However, a user
    // can't filter or aggregate based on the affected rows of a procedure, so we don't need to add
    // any aggregate functions or comparison operators. However, if int exists elsewhere in the
    // schema and has already been added, it will also already contain these functions and
    // operators.
    scalar_types
        .entry("int".to_string())
        .or_insert(models::ScalarType {
            aggregate_functions: BTreeMap::new(),
            comparison_operators: BTreeMap::new(),
            representation: Some(TypeRepresentation::Int32),
        });

    fields.insert(
        "affected_rows".to_string(),
        models::ObjectField {
            description: Some("The number of rows affected by the mutation".to_string()),
            r#type: models::Type::Named {
                name: "int".to_string(),
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

/// Gets the schema of the native mutations.

/// Each native mutation creates two objects:
/// 1. Object with name `{native_mutation_name}_response`, this object
///    will contain two fields:
///         a. affected_rows: int - The rows affected by the native mutation.
///         b. returning: `{native_mutation_name}` - Data returned by the native mutation.
/// 2. Object with name `{native_mutation_name}` whose fields will
///    contain the fields specified in the `columns`.
fn get_native_mutations_schema(
    native_mutations_metadata: &query_engine_metadata::metadata::NativeMutations,
    object_types: &mut BTreeMap<String, models::ObjectType>,
    scalar_types: &mut BTreeMap<String, models::ScalarType>,
) -> Result<Vec<models::ProcedureInfo>, connector::SchemaError> {
    let mut native_mutations = Vec::new();

    native_mutations_metadata.0.iter().for_each(|(name, info)| {
        let native_query_object_type = models::ObjectType {
            description: info.description.clone(),
            fields: BTreeMap::from_iter(info.columns.iter().map(|(column_name, column)| {
                (
                    column_name.clone(),
                    models::ObjectField {
                        description: column.description.clone(),
                        r#type: column_to_type(column),
                    },
                )
            })),
        };
        object_types.insert(name.clone(), native_query_object_type);

        let procedure_info = make_procedure_type(
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
            object_types,
            scalar_types,
        );
        native_mutations.push(procedure_info);
    });

    Ok(native_mutations)
}

/// Gets the schema of the native queries.
/// Each native query creates creates an object named as the name
/// of the native query with the fields of the object corresponding
/// to the `columns` field.
fn get_native_queries_schema(
    native_queries: &query_engine_metadata::metadata::NativeQueries,
    object_types: &mut BTreeMap<String, models::ObjectType>,
) -> Result<Vec<models::CollectionInfo>, connector::SchemaError> {
    let mut read_only_native_queries = Vec::new();

    native_queries.0.iter().for_each(|(name, info)| {
        let native_query_object_type = models::ObjectType {
            description: info.description.clone(),
            fields: BTreeMap::from_iter(info.columns.iter().map(|(column_name, column)| {
                (
                    column_name.clone(),
                    models::ObjectField {
                        description: column.description.clone(),
                        r#type: column_to_type(column),
                    },
                )
            })),
        };
        object_types.insert(name.clone(), native_query_object_type);

        let native_query_collection_info = models::CollectionInfo {
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
        };
        read_only_native_queries.push(native_query_collection_info);
    });

    Ok(read_only_native_queries)
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
        configuration::occurring_scalar_types(metadata)
            .iter()
            .map(|scalar_type| {
                (
                    scalar_type.0.clone(),
                    models::ScalarType {
                        // TODO(PY): Add representation for beta
                        representation: None,
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
                                    match op_def.operator_kind {
                                        metadata::OperatorKind::Equal => {
                                            models::ComparisonOperatorDefinition::Equal
                                        }
                                        metadata::OperatorKind::In => {
                                            models::ComparisonOperatorDefinition::In
                                        }
                                        metadata::OperatorKind::Custom => {
                                            models::ComparisonOperatorDefinition::Custom {
                                                argument_type: models::Type::Named {
                                                    name: op_def.argument_type.0.clone(),
                                                },
                                            }
                                        }
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

    let mut object_types = table_types;

    let native_queries = get_native_queries_schema(&metadata.native_queries, &mut object_types)?;

    let native_mutations = get_native_mutations_schema(
        &metadata.native_mutations,
        &mut object_types,
        &mut scalar_types,
    )?;

    let mut collections = tables;
    collections.extend(native_queries);

    let procedures = native_mutations;

    Ok(models::SchemaResponse {
        collections,
        procedures,
        functions: vec![],
        object_types,
        scalar_types,
    })
}

#[cfg(test)]
mod tests {
    use ndc_sdk::models::{ObjectField, ObjectType, ProcedureInfo};
    use query_engine_metadata::metadata::{
        parse_native_query, ColumnInfo, NativeMutations, NativeQueryInfo, NativeQuerySql,
        ScalarType,
    };

    use super::*;

    #[test]
    fn test_native_mutation_schema() {
        let parsed_sql = parse_native_query("INSERT INTO Authors(Id, Name) OUTPUT inserted.Id as id, inserted.Name as name VALUES ({{id}}, {{name}})");
        let sql = NativeQuerySql(parsed_sql);

        let id_col_info = ColumnInfo {
            name: "Id".to_string(),
            r#type: ScalarType("int".to_string()),
            nullable: metadata::Nullable::NonNullable,
            description: None,
        };

        let name_col_info = ColumnInfo {
            name: "Name".to_string(),
            r#type: ScalarType("varchar".to_string()),
            nullable: metadata::Nullable::NonNullable,
            description: None,
        };

        let mut columns = BTreeMap::new();

        columns.insert("id".to_owned(), id_col_info);
        columns.insert("name".to_owned(), name_col_info);

        let native_mutation_info = NativeQueryInfo {
            arguments: BTreeMap::new(),
            sql,
            columns,
            description: None,
        };

        let mut native_mutations = BTreeMap::new();

        native_mutations.insert(
            "insert_user_native_mutation".to_string(),
            native_mutation_info,
        );

        let native_mutations = NativeMutations(native_mutations);

        let mut object_types = BTreeMap::new();
        let mut scalar_types = BTreeMap::new();

        let native_mutation_procedure_info =
            get_native_mutations_schema(&native_mutations, &mut object_types, &mut scalar_types)
                .unwrap();

        let expected_mutation_procedure_info = ProcedureInfo {
            name: "insert_user_native_mutation".to_string(),
            description: None,
            arguments: BTreeMap::new(),
            result_type: ndc_sdk::models::Type::Named {
                name: "insert_user_native_mutation_response".into(),
            },
        };

        assert_eq!(
            native_mutation_procedure_info,
            vec![expected_mutation_procedure_info]
        );

        let expected_object_field_id = ObjectField {
            description: None,
            r#type: models::Type::Named {
                name: "int".to_string(),
            },
        };

        let expected_object_field_name = ObjectField {
            description: None,
            r#type: models::Type::Named {
                name: "varchar".to_string(),
            },
        };

        let expected_object_field_affected_rows = ObjectField {
            description: Some("The number of rows affected by the mutation".into()),
            r#type: models::Type::Named {
                name: "int".to_string(),
            },
        };

        let expected_native_mutation_object_type = ObjectType {
            description: None,
            fields: BTreeMap::from([
                ("id".to_owned(), expected_object_field_id),
                ("name".to_owned(), expected_object_field_name),
            ]),
        };

        let expected_object_field_returning = ObjectField {
            description: Some("Data from rows affected by the mutation".into()),
            r#type: models::Type::Array {
                element_type: Box::new(models::Type::Named {
                    name: "insert_user_native_mutation".into(),
                }),
            },
        };

        let expected_native_mutation_response_object_type = ObjectType {
            description: Some("Responses from the 'insert_user_native_mutation' procedure".into()),
            fields: BTreeMap::from([
                (
                    "affected_rows".to_owned(),
                    expected_object_field_affected_rows,
                ),
                ("returning".to_owned(), expected_object_field_returning),
            ]),
        };

        let mut expected_object_types = BTreeMap::new();
        expected_object_types.insert(
            "insert_user_native_mutation".into(),
            expected_native_mutation_object_type,
        );
        expected_object_types.insert(
            "insert_user_native_mutation_response".to_string(),
            expected_native_mutation_response_object_type,
        );

        assert_eq!(object_types, expected_object_types);
    }
}
