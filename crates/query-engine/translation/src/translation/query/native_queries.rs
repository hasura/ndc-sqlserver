//! Handle native queries translation after building the query.

use ndc_sdk::models;

use crate::translation::error::Error;
use crate::translation::helpers::{NativeQueryInfo, State};
use crate::translation::values;

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
