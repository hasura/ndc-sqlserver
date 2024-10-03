use std::collections::BTreeMap;

use super::Error;
use crate::translation::{
    helpers::{CollectionOrProcedureInfo, Env, State, StoredProcedureInfo, TableNameAndReference},
    query::translate_query,
    values::translate_json_value,
};
use indexmap::IndexMap;
use ndc_sdk::models;
use query_engine_metadata::metadata::{ColumnInfo, Nullable};
use query_engine_sql::sql::{
    self,
    ast::{
        ColumnType, ExecProcedure, ExecProcedureInsertIntoTempTable, TableAlias, TemporaryTable,
        TemporaryTableName,
    },
    execution_plan::StoredProcedureExecutionPlan,
    helpers::make_column_alias,
};

fn parse_stored_procedure_fields(
    fields: Option<models::NestedField>,
) -> Result<Option<IndexMap<models::FieldName, models::Field>>, Error> {
    match fields {
        None => Ok(None),
        Some(models::NestedField::Object(_)) => Err(Error::UnexpectedStructure(
            "Stored procedures cannot return a single object".to_string(),
        )),
        Some(models::NestedField::Array(models::NestedArray { fields })) => match *fields {
            models::NestedField::Object(models::NestedObject { fields }) => Ok(Some(fields)),
            models::NestedField::Array(_) => Err(Error::UnexpectedStructure(
                "multi-dimensional array in 'returning' clause".to_string(),
            )),
        },
    }
}

fn get_all_procedure_fields(
    proc_fields: BTreeMap<String, ColumnInfo>,
) -> IndexMap<models::FieldName, models::Field> {
    let mut fields = IndexMap::new();
    for (proc_field_name, proc_field_col_info) in proc_fields {
        fields.insert(
            proc_field_name.into(),
            models::Field::Column {
                arguments: BTreeMap::new(),
                column: proc_field_col_info.name.into(),
                fields: None,
            },
        );
    }
    fields
}

pub(crate) fn generate_execution_plan(
    env: &Env,
    state: &mut State,
    stored_proc_info: StoredProcedureInfo,
    requested_fields: Option<ndc_sdk::models::NestedField>,
    provided_args: &BTreeMap<models::ArgumentName, serde_json::Value>,
) -> Result<StoredProcedureExecutionPlan, Error> {
    // Compute the fields that need to be returned.
    let parsed_fields = parse_stored_procedure_fields(requested_fields)?.unwrap_or(
        get_all_procedure_fields(stored_proc_info.info.returns.clone().ok_or(
            Error::UnexpectedInternalError(
                "Found stored procedure without a return type".to_string(),
            ),
        )?),
    );
    let mut args = BTreeMap::new();

    // Process the arguments provided and convert it into
    // an `Expression`
    for (arg_name, arg_info) in stored_proc_info.info.arguments {
        let arg_val: Option<&serde_json::Value> =
            provided_args.get::<models::ArgumentName>(&arg_name.clone().into());

        match arg_val {
            Some(arg_val) if *arg_val != serde_json::Value::Null => {
                args.insert(arg_name, translate_json_value(arg_val, &arg_info.r#type)?);
            }
            Some(arg_val) => {
                if arg_info.nullable == Nullable::NonNullable {
                    return Err(Error::ArgumentNotFound(arg_name));
                } else {
                    args.insert(arg_name, translate_json_value(arg_val, &arg_info.r#type)?);
                }
            }
            // Throw error if we recieve a `null` or undefined value for a required argument
            None => {
                if arg_info.nullable == Nullable::NonNullable {
                    return Err(Error::ArgumentNotFound(arg_name));
                }
            }
        }
    }

    let temp_table_alias: TableAlias =
        state.make_stored_procedure_table_alias(&stored_proc_info.name);

    let temp_table = TemporaryTable {
        name: TemporaryTableName(temp_table_alias.clone()),
        columns: {
            stored_proc_info
                .info
                .returns
                .unwrap_or_default()
                .values()
                .map(|col_info| (col_info.name.clone(), ColumnType(col_info.r#type.0.clone())))
                .collect()
        },
    };

    // Response selection

    let query = models::Query {
        aggregates: None,
        fields: Some(parsed_fields),
        limit: None,
        offset: None,
        order_by: None,
        predicate: None,
    };

    let table_ref = sql::ast::TableReference::AliasedTable(temp_table_alias.clone());

    let table_name_and_ref = TableNameAndReference {
        name: stored_proc_info.name.clone(),
        reference: sql::ast::TableReference::AliasedTable(temp_table_alias.clone()),
    };

    let from_clause = sql::ast::From::Table {
        reference: table_ref,
        alias: temp_table_alias.clone(),
    };

    let collection_info = env
        .lookup_procedure(&stored_proc_info.name)
        .map(CollectionOrProcedureInfo::Procedure)
        .ok_or(Error::ProcedureNotFound(stored_proc_info.name))?;

    let select_set = translate_query(
        env,
        state,
        &collection_info,
        &table_name_and_ref,
        &from_clause,
        &query,
        &temp_table_alias,
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
            sql::helpers::make_column_alias("rows".to_string()),
        ),
        vec![make_column_alias("rows".to_string())],
        state.make_table_alias("aggregates".to_string()),
        make_column_alias("aggregates".to_string()),
        select_set,
    );

    let exec_proc_sql = ExecProcedureInsertIntoTempTable {
        temp_table,
        exec_procedure: ExecProcedure {
            arguments: args,
            procedure_name: stored_proc_info.info.name,
            procedure_schema: stored_proc_info.info.schema,
        },
        response_selection: json_select,
    };

    Ok(StoredProcedureExecutionPlan {
        temp_table_name: temp_table_alias.name,
        stored_procedure_sql_query: exec_proc_sql,
    })
}
