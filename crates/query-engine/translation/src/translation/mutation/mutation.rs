use indexmap::indexmap;
use indexmap::IndexMap;
use ndc_sdk::models::{self};
use query_engine_metadata::metadata;
use query_engine_sql::sql::execution_plan::{
    MutationExecutionPlan, MutationOperationExecutionPlan, NativeMutationOperationExecutionPlan,
    NativeMutationResponseSelection,
};
use query_engine_sql::sql::{
    self,
    ast::{
        ColumnAlias, ColumnName, CommonTableExpression, Expression, RawSQLStatement, ScalarType,
        Select, TableAlias, Where, WithJSONSchema,
    },
    helpers::{empty_group_by, empty_order_by, empty_where, empty_with, make_column_alias},
    string::SQL,
};

use crate::translation::helpers::generate_native_query_sql;
use crate::translation::{
    error::Error,
    helpers::{
        Env, MutationOperation, NativeMutationInfo, ProcedureInfo, State, TableNameAndReference,
    },
    query,
};

pub fn translate(
    metadata: &metadata::Metadata,
    mutation_request: models::MutationRequest,
) -> Result<sql::execution_plan::MutationExecutionPlan, Error> {
    let env = Env::new(metadata, mutation_request.collection_relationships);
    let state = State::new();

    let mut translated_mutation_operations = Vec::new();

    for mutation_operation in mutation_request.operations {
        translated_mutation_operations
            .push(translate_mutation_operation(&env, mutation_operation)?);
    }

    generate_mutation_execution_plan(&env, state, translated_mutation_operations)
}

fn translate_mutation_operation(
    env: &Env,

    mutation_operation: ndc_sdk::models::MutationOperation,
) -> Result<MutationOperation, Error> {
    match mutation_operation {
        ndc_sdk::models::MutationOperation::Procedure {
            name,
            arguments,
            fields,
        } => {
            let procedure_info: ProcedureInfo = env.lookup_procedure(&name)?;
            match procedure_info {
                ProcedureInfo::NativeMutation { info, .. } => {
                    Ok(MutationOperation::NativeMutation(NativeMutationInfo {
                        name: name.to_string(),
                        info,
                        arguments,
                        fields,
                    }))
                }
            }
        }
    }
}

/// A procedure expects an object with two fields:
///     * affected_rows, the integer number of rows affected by the operation
///     * returning, the nested array object of rows returned
///
/// The user must supply at least one of these two structures, and otherwise we'll throw an error.
#[allow(clippy::type_complexity)]
pub fn parse_procedure_fields(
    fields: Option<models::NestedField>,
) -> Result<
    (
        (String, Option<IndexMap<String, models::Aggregate>>), // Contains "affected_rows"
        (String, Option<IndexMap<String, models::Field>>),     // Contains "returning"
    ),
    Error,
> {
    match fields {
        Some(models::NestedField::Object(models::NestedObject { fields })) => {
            let mut affected_rows = ("affected_rows".to_string(), None);
            let mut returning = ("returning".to_string(), None);

            for (alias, field) in fields {
                match field {
                    models::Field::Column { column, fields: _ } if column == "affected_rows" => {
                        affected_rows = (
                            alias.clone(),
                            Some(indexmap!(alias => models::Aggregate::StarCount {})),
                        );
                    }
                    models::Field::Column { column, fields } if column == "returning" => {
                        returning = match fields {
                            Some(nested_fields) => match nested_fields {
                                models::NestedField::Object(models::NestedObject { .. }) => {
                                    Err(Error::UnexpectedStructure(
                                        "single object in 'returning' clause".to_string(),
                                    ))?
                                }
                                models::NestedField::Array(models::NestedArray { fields }) => {
                                    match &*fields {
                                        models::NestedField::Object(models::NestedObject {
                                            fields,
                                        }) => (alias, Some(fields.clone())),
                                        models::NestedField::Array(_) => {
                                            Err(Error::UnexpectedStructure(
                                                "multi-dimensional array in 'returning' clause"
                                                    .to_string(),
                                            ))?
                                        }
                                    }
                                }
                            },
                            None => returning,
                        };
                    }
                    _ => Err(Error::UnexpectedStructure(
                        "single object in 'returning' clause".to_string(),
                    ))?,
                }
            }

            if affected_rows.1.is_none() && returning.1.is_none() {
                Err(Error::NoProcedureResultFieldsRequested)?
            }

            Ok((affected_rows, returning))
        }

        Some(models::NestedField::Array(_)) => {
            Err(Error::NotImplementedYet("nested array fields".to_string()))
        }
        None => Err(Error::NoProcedureResultFieldsRequested)?,
    }
}

/// This function constructs a CTE that represents the return type of the
/// Native Mutation and then this CTE will be the source for selecting
/// fields from it and return it into the `MutationResponse` format.
fn get_native_mutation_response_selection(
    native_mutation_info: &NativeMutationInfo,
    response_select: Select,
    json_cte_table_alias: TableAlias,
) -> Result<NativeMutationResponseSelection, Error> {
    let mut select_list = Vec::new();

    // Collects information about the columns
    // mentioned in the native mutation info
    let mut with_json_schema = Vec::new();

    for (col_name, col_info) in native_mutation_info.info.columns.iter() {
        let column_alias = ColumnAlias {
            name: col_name.to_string(),
        };
        select_list.push((
            column_alias.clone(),
            Expression::ColumnReference(sql::ast::ColumnReference::TableColumn {
                table: sql::ast::TableReference::AliasedTable(TableAlias {
                    unique_index: 0,
                    name: "open_json".to_string(),
                }),
                name: ColumnName(col_name.to_string()),
            }),
        ));

        with_json_schema.push((column_alias, ScalarType(col_info.r#type.0.clone())))
    }

    Ok(NativeMutationResponseSelection {
        response_json_schema: WithJSONSchema(with_json_schema),
        response_select,
        response_cte_table_alias: json_cte_table_alias,
    })
}

pub fn generate_native_mutation_response_cte(
    response_json: String,
    json_schema: WithJSONSchema,
    cte_alias: TableAlias,
) -> CommonTableExpression {
    let from_clause = sql::ast::From::OpenJSON {
        alias: cte_alias.clone(),
        json_value_param: sql::string::Param::String(response_json),
        with_json_schema: json_schema.clone(),
    };

    let cte_select = Select {
        with: empty_with(),
        select_list: sql::ast::SelectList::SelectStar,
        from: Some(from_clause),
        joins: Vec::new(),
        where_: Where(empty_where()),
        group_by: empty_group_by(),
        order_by: empty_order_by(),
        limit: None,
        for_json: sql::ast::ForJson::NoJson,
    };

    CommonTableExpression {
        alias: cte_alias,
        column_names: Some(json_schema.0.into_iter().map(|c| c.0).collect()),
        select: sql::ast::CTExpr::Select(cte_select),
    }
}

fn generate_mutation_execution_plan(
    env: &Env,
    mut state: State,
    mutation_operations: Vec<MutationOperation>,
) -> Result<MutationExecutionPlan, Error> {
    let mut mutations: Vec<MutationOperationExecutionPlan> = Vec::new();

    // Traverse over the mutation operations and compute the SQL statements that need to
    // be run.
    for mutation_operation in mutation_operations {
        match mutation_operation {
            crate::translation::helpers::MutationOperation::NativeMutation(
                native_mutation_info,
            ) => {
                let raw_sql = generate_native_query_sql(
                    native_mutation_info.info.arguments.clone(),
                    native_mutation_info
                        .arguments
                        .clone()
                        .into_iter()
                        .map(|(arg_name, arg_value)| {
                            (arg_name, models::Argument::Literal { value: arg_value })
                        })
                        .collect(),
                    native_mutation_info.info.sql.clone(),
                )
                .map(RawSQLStatement)?;

                let mut mutation_sql_query = SQL::new();

                raw_sql.to_sql(&mut mutation_sql_query);

                let (affected_rows, (returning_alias, returning)) =
                    parse_procedure_fields(native_mutation_info.fields.clone())?;

                let query = ndc_sdk::models::Query {
                    aggregates: affected_rows.1,
                    fields: returning,
                    limit: None,
                    offset: None,
                    order_by: None,
                    predicate: None,
                };

                let json_response_cte_alias = TableAlias {
                    unique_index: 0,
                    name: "json_response_cte_alias".to_string(),
                };

                let json_response_table_alias = TableNameAndReference {
                    name: native_mutation_info.name.clone(),
                    reference: sql::ast::TableReference::AliasedTable(
                        json_response_cte_alias.clone(),
                    ),
                };

                let from_clause = sql::ast::From::Table {
                    reference: sql::ast::TableReference::AliasedTable(
                        json_response_cte_alias.clone(),
                    ),
                    alias: json_response_cte_alias.clone(),
                };

                let select_set = query::translate_query(
                    env,
                    &mut state,
                    &json_response_table_alias,
                    &from_clause,
                    &query,
                    &json_response_cte_alias,
                )?;

                // form a single JSON item shaped `{ type: "procedure", result: "<mutation_operation_result>" }`
                // that matches the models::RowSet type
                let json_select = sql::helpers::select_mutation_rowset(
                    (
                        state.make_table_alias("universe".to_string()),
                        sql::helpers::make_column_alias("universe".to_string()),
                    ),
                    (
                        state.make_table_alias("rows".to_string()),
                        sql::helpers::make_column_alias(returning_alias),
                    ),
                    state.make_table_alias("aggregates".to_string()),
                    make_column_alias(affected_rows.0),
                    select_set,
                );

                let mut response_selection_sql = SQL::new();

                json_select.to_sql(&mut response_selection_sql);

                let response_selection = get_native_mutation_response_selection(
                    &native_mutation_info,
                    json_select,
                    json_response_cte_alias,
                )?;

                let native_mutation_exec_plan = NativeMutationOperationExecutionPlan {
                    mutation_sql_query,
                    response_selection,
                    native_mutation_name: native_mutation_info.name.clone(),
                };

                mutations.push(MutationOperationExecutionPlan::NativeMutation(
                    native_mutation_exec_plan,
                ));
            }
        }
    }

    Ok(MutationExecutionPlan { mutations })
}
