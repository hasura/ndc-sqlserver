//! Handle native queries translation after building the query.

use ndc_sdk::models;
use query_engine_sql::sql::ast::TableAlias;

use super::error::Error;
use super::helpers::NativeQueryInfo;
use super::helpers::State;
use super::values;
use query_engine_metadata::metadata::{self};
use query_engine_sql::sql;

/// Given a `NativeQueryInfo` substitutes the value of the arguments
/// and constructs an AST from which the Raw SQL can be derived.
fn generate_sql(native_query: &NativeQueryInfo) -> Result<Vec<sql::ast::RawSql>, Error> {
    native_query
        .info
        .sql
        .0
        .iter()
        .map(|part| match part {
            metadata::NativeQueryPart::Text(text) => Ok(sql::ast::RawSql::RawText(text.clone())),
            metadata::NativeQueryPart::Parameter(param) => {
                let typ = match native_query.info.arguments.get(param) {
                    None => Err(Error::ArgumentNotFound(param.clone())),
                    Some(argument) => Ok(argument.r#type.clone()),
                }?;
                let exp = match native_query.arguments.get(param) {
                    None => Err(Error::ArgumentNotFound(param.clone())),
                    Some(argument) => match argument {
                        models::Argument::Literal { value } => {
                            values::translate_json_value(value, &typ)
                        }
                        models::Argument::Variable { name } => {
                            Ok(values::translate_variable(name.clone(), &typ))
                        }
                    },
                }?;
                Ok(sql::ast::RawSql::Expression(exp))
            }
        })
        .collect()
}

/// Translate native queries collected in State by the translation proccess into CTEs.
pub fn translate_native_queries(
    state: State,
) -> Result<Vec<sql::ast::CommonTableExpression>, Error> {
    let mut ctes = vec![];
    let native_queries = state.get_native_queries();

    // for each found table expression
    for native_query in native_queries {
        // convert metadata representation to sql::ast representation
        let sql: Vec<sql::ast::RawSql> = generate_sql(&native_query)?;

        // add a cte
        ctes.push(sql::ast::CommonTableExpression {
            alias: native_query.alias,
            column_names: None,
            select: sql::ast::CTExpr::RawSql(sql),
        });
    }

    Ok(ctes)
}

/// Translate native mutations collected in State by the translation process into
/// a vector of Raw SQL statements
pub fn translate_native_mutations(
    state: State,
) -> Result<Vec<(TableAlias, Vec<sql::ast::RawSql>)>, Error> {
    let native_mutations = state.get_native_mutations();

    let mut native_mutations_sql_queries = vec![];

    for native_mutation in native_mutations {
        let sql = generate_sql(&native_mutation)?;
        native_mutations_sql_queries.push((native_mutation.alias, sql));
    }
    Ok(native_mutations_sql_queries)
}
