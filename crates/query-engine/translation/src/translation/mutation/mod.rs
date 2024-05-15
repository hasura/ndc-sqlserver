use ndc_sdk::models::{self, MutationOperation};
use query_engine_metadata::metadata;
use query_engine_sql::sql;
use query_engine_sql::sql::ast::{RawSQLQuery, RawSQLStatement};

use crate::translation::error;
use crate::translation::helpers::{Env, ProcedureInfo, State};
use crate::translation::values;

use super::helpers::NativeMutationInfo;

pub fn translate(
    metadata: &metadata::Metadata,
    mutation_request: models::MutationRequest,
) -> Result<sql::execution_plan::MutationExecutionPlan, error::Error> {
    let env = Env::new(metadata, mutation_request.collection_relationships);
    let mut state = State::new();

    for mutation_operation in mutation_request.operations {
        translate_mutation_operation(&env, &mut state, mutation_operation)?;
    }

    let sql = translate_mutation(state)?; // This function call doesn't make sense, move the logic within it, to the for loop above.
    Ok(sql::execution_plan::MutationExecutionPlan {
        mutation_query: sql,
    })
}

fn translate_mutation_operation(
    env: &Env,
    state: &mut State,
    mutation_operation: MutationOperation,
) -> Result<(), error::Error> {
    match mutation_operation {
        MutationOperation::Procedure {
            name,
            arguments,
            fields,
        } => {
            println!("arguments recieved are {:#?}", arguments);
            let procedure_info: ProcedureInfo = env.lookup_procedure(&name)?;
            match procedure_info {
                ProcedureInfo::NativeMutation { info, .. } => {
                    state.insert_native_mutation(name.as_str(), info, arguments)
                }
            }
        }
    }
    Ok(())
}

// FIXME: This function already exists in `translation/query.rs`, please refactor to
// deduplicate it.
fn generate_sql(
    native_mutation: &NativeMutationInfo,
) -> Result<Vec<sql::ast::RawSql>, error::Error> {
    native_mutation
        .info
        .sql
        .0
        .iter()
        .map(|part| match part {
            metadata::NativeQueryPart::Text(text) => Ok(sql::ast::RawSql::RawText(text.clone())),
            metadata::NativeQueryPart::Parameter(param) => {
                let typ = match native_mutation.info.arguments.get(param) {
                    None => Err(error::Error::ArgumentNotFound(param.clone())),
                    Some(argument) => Ok(argument.r#type.clone()),
                }?;
                let exp = match native_mutation.arguments.get(param) {
                    None => Err(error::Error::ArgumentNotFound(param.clone())),
                    Some(argument) => values::translate_json_value(argument, &typ),
                }?;

                println!("Exp is {exp:#?}");

                Ok(sql::ast::RawSql::Expression(exp))
            }
        })
        .collect()
}

fn translate_mutation(state: State) -> Result<sql::string::SQL, error::Error> {
    let mut sql = sql::string::SQL::new();
    let mut mutation_sql = vec![];

    for native_mutation in state.get_native_mutations() {
        let raw_sql_statement = generate_sql(&native_mutation).map(RawSQLStatement)?;
        mutation_sql.push(raw_sql_statement);
    }

    let raw_sql = RawSQLQuery(mutation_sql);

    raw_sql.to_sql(&mut sql);

    println!("SQL is {:#?}", sql);

    Ok(sql)
}
