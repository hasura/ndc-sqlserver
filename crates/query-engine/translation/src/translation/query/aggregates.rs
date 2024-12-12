//! Handle aggregates translation.

use indexmap::IndexMap;

use crate::translation::{
    error::Error,
    helpers::{CollectionOrProcedureInfo, Env},
};
use query_engine_sql::sql::{self, ast::ScalarType, helpers::cast_column};

/// Translate any aggregates we should include in the query into our SQL AST.
pub fn translate(
    table: &sql::ast::TableReference,
    aggregates: &IndexMap<ndc_models::FieldName, ndc_models::Aggregate>,
    collection_info: &CollectionOrProcedureInfo,
    env: &Env,
) -> Result<Vec<(sql::ast::ColumnAlias, sql::ast::Expression)>, Error> {
    aggregates
        .into_iter()
        .map(|(alias, aggregation)| {
            let expression = match aggregation {
                ndc_models::Aggregate::ColumnCount {
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
                ndc_models::Aggregate::SingleColumn {
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
                    let column_type = match collection_info {
                        CollectionOrProcedureInfo::Collection(ci) => match ci {
                            crate::translation::helpers::CollectionInfo::Table { info, .. } => {
                                info.columns.get(column.as_str()).map(|c| c.r#type.clone())
                            }
                            crate::translation::helpers::CollectionInfo::NativeQuery {
                                info,
                                ..
                            } => info.columns.get(column.as_str()).map(|c| c.r#type.clone()),
                            _ => None,
                        },
                        CollectionOrProcedureInfo::Procedure(_) => None,
                    };
                    let function_type = column_type.and_then(|column_type| {
                        env.metadata
                            .aggregate_functions
                            .0
                            .get(&column_type)
                            .and_then(|functions| functions.get(function.as_str()))
                    });
                    match function.as_str() {
                        "SUM" | "AVG" => sql::ast::Expression::Cast {
                            expression: Box::new(sql::ast::Expression::FunctionCall {
                                function: sql::ast::Function::Unknown(function.to_string()),
                                args: vec![sql::ast::Expression::Cast {
                                    expression: Box::new(column_ref_expression),
                                    r#type: sql::ast::ScalarType("BIGINT".to_string()),
                                }],
                            }),
                            r#type: sql::ast::ScalarType("varchar".to_string()),
                        },
                        _ => {
                            let expression = sql::ast::Expression::FunctionCall {
                                function: sql::ast::Function::Unknown(function.to_string()),
                                args: vec![column_ref_expression],
                            };
                            match function_type.map(|f| f.return_type.clone()) {
                                Some(scalar_type) => {
                                    cast_column(&ScalarType(scalar_type.0), expression)
                                }
                                None => expression,
                            }
                        }
                    }
                }
                ndc_models::Aggregate::StarCount {} => {
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
