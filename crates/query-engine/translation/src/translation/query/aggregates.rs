//! Handle aggregates translation.

use indexmap::IndexMap;

use ndc_sdk::models;

use crate::translation::error::Error;
use query_engine_sql::sql;

/// Translate any aggregates we should include in the query into our SQL AST.
pub fn translate(
    table: &sql::ast::TableReference,
    aggregates: &IndexMap<models::FieldName, models::Aggregate>,
) -> Result<Vec<(sql::ast::ColumnAlias, sql::ast::Expression)>, Error> {
    aggregates
        .into_iter()
        .map(|(alias, aggregation)| {
            let expression = match aggregation {
                models::Aggregate::ColumnCount {
                    column, distinct, ..
                } => {
                    let count_column_alias = sql::helpers::make_column_alias(column.to_string());
                    if *distinct {
                        sql::ast::Expression::Count(sql::ast::CountType::Distinct(
                            sql::ast::ColumnReference::AliasedColumn {
                                table: table.clone(),
                                column: count_column_alias,
                            },
                        ))
                    } else {
                        sql::ast::Expression::Count(sql::ast::CountType::Simple(
                            sql::ast::ColumnReference::AliasedColumn {
                                table: table.clone(),
                                column: count_column_alias,
                            },
                        ))
                    }
                }
                models::Aggregate::SingleColumn {
                    column,
                    function,
                    field_path: _,
                } => {
                    let column_ref_expression = sql::ast::Expression::ColumnReference(
                        sql::ast::ColumnReference::AliasedColumn {
                            table: table.clone(),
                            column: sql::helpers::make_column_alias(column.to_string()),
                        },
                    );
                    match function.as_str() {
                        "SUM" | "AVG" => sql::ast::Expression::FunctionCall {
                            function: sql::ast::Function::Unknown(function.to_string()),
                            args: vec![sql::ast::Expression::Cast {
                                expression: Box::new(column_ref_expression),
                                r#type: sql::ast::ScalarType("BIGINT".to_string()),
                            }],
                        },
                        _ => sql::ast::Expression::FunctionCall {
                            function: sql::ast::Function::Unknown(function.to_string()),
                            args: vec![column_ref_expression],
                        },
                    }
                }
                models::Aggregate::StarCount {} => {
                    sql::ast::Expression::Count(sql::ast::CountType::Star)
                }
            };
            Ok((
                sql::helpers::make_column_alias(alias.to_string()),
                expression,
            ))
        })
        .collect::<Result<Vec<_>, Error>>()
}
