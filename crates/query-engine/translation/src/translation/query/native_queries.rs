//! Handle native queries translation after building the query.

use crate::translation::helpers::State;
use crate::translation::{error::Error, helpers::generate_native_query_sql};

use query_engine_sql::sql;

/// Translate native queries collected in State by the translation proccess into CTEs.
pub fn translate_native_queries(
    state: State,
) -> Result<Vec<sql::ast::CommonTableExpression>, Error> {
    let mut ctes = vec![];
    let native_queries = state.get_native_queries();

    // for each found table expression
    for native_query in native_queries {
        // convert metadata representation to sql::ast representation
        let sql = generate_native_query_sql(
            &native_query.info.arguments,
            &native_query.arguments,
            &native_query.info.sql,
        )?;

        // add a cte
        ctes.push(sql::ast::CommonTableExpression {
            alias: native_query.alias,
            column_names: None,
            select: sql::ast::CTExpr::RawSql(sql),
        });
    }

    Ok(ctes)
}
