//! Helpers for building sql::ast types in certain shapes and patterns.

use super::ast::*;

/// Used as input to helpers to construct SELECTs which return 'rows' and/or 'aggregates' results.
pub enum SelectSet {
    Rows(Select),
    Aggregates(Select),
    RowsAndAggregates(Select, Select),
}

// Empty clauses //

/// An empty `WITH` clause.
pub fn empty_with() -> With {
    With {
        common_table_expressions: vec![],
    }
}

/// An empty `WHERE` clause.
pub fn empty_where() -> Expression {
    Expression::Value(Value::Bool(true))
}

/// An empty `GROUP BY` clause.
pub fn empty_group_by() -> GroupBy {
    GroupBy {}
}

/// An empty `ORDER BY` clause.
pub fn empty_order_by() -> OrderBy {
    OrderBy { elements: vec![] }
}

/// A `true` expression.
pub fn true_expr() -> Expression {
    Expression::Value(Value::Bool(true))
}

/// A `false` expression.
pub fn false_expr() -> Expression {
    Expression::Value(Value::Bool(false))
}

// Aliasing //

/// Generate a column expression refering to a specific table.
pub fn make_column(
    table: TableReference,
    name: ColumnName,
    alias: ColumnAlias,
) -> (ColumnAlias, Expression) {
    (
        alias,
        Expression::ColumnReference(ColumnReference::TableColumn { table, name }),
    )
}
/// Create column aliases using this function so we build everything in one place.
pub fn make_column_alias(name: String) -> ColumnAlias {
    ColumnAlias { name }
}

/// we need this in a bunch of places
pub fn make_json_column_alias() -> ColumnAlias {
    ColumnAlias {
        name: "json".to_string(),
    }
}

pub fn empty_json_path() -> JsonPath {
    JsonPath { elements: vec![] }
}

pub fn empty_alias_path() -> AliasPath {
    AliasPath { elements: vec![] }
}

// SELECTs //

/// Build a simple select with a select list and the rest are empty.
pub fn simple_select(select_list: Vec<(ColumnAlias, Expression)>) -> Select {
    Select {
        with: empty_with(),
        select_list: SelectList::SelectList(select_list),
        from: None,
        joins: vec![],
        where_: Where(empty_where()),
        group_by: empty_group_by(),
        order_by: empty_order_by(),
        limit: None,
        for_json: ForJson::NoJson,
    }
}

/// Build a simple select *
pub fn star_select(from: From) -> Select {
    Select {
        with: empty_with(),
        select_list: SelectList::SelectStar,
        from: Some(from),
        joins: vec![],
        where_: Where(empty_where()),
        group_by: empty_group_by(),
        order_by: empty_order_by(),
        limit: None,
        for_json: ForJson::ForJsonPath,
    }
}

/// given a set of rows and aggregate queries, combine them into
/// one Select
/// SELECT JSON_VALUE([aggregates].[aggregates_json], "$.aggregates_json") as [aggregates],
///  JSON_QUERY(isnull([rows].[row_json],'[]'), "$.row_json") AS [rows]
/// FROM (
///     SELECT *
///     FROM (
///         SELECT [Album].[Title] AS [Title]
///         FROM [Album] AS [Album] --LIMIT 5 OFFSET 3
///           FOR JSON PATH
///       ) AS [rows]([json]) FOR JSON PATH,
///       WITHOUT_ARRAY_WRAPPER
///   ) as [rows]([json])
///   CROSS JOIN (
///     SELECT [aggregates].[json] AS [aggregates]
///     FROM (
///         SELECT COUNT(*) AS [Count]
///         FROM [Album] AS [Album] --LIMIT 5 OFFSET 3
///           FOR JSON PATH,
///           WITHOUT_ARRAY_WRAPPER
///       ) AS [aggregates]([json]) FOR JSON PATH,
///       WITHOUT_ARRAY_WRAPPER
///   ) as [aggregates]([json]) FOR JSON PATH,
///  WITHOUT_ARRAY_WRAPPER
///
/// The `row_select` and `aggregate_set` will not be included if they are not relevant
pub fn select_rowset(
    output_table_alias: TableAlias,
    row_table_alias: TableAlias,
    row_column_alias: ColumnAlias,
    aggregate_table_alias: TableAlias,
    aggregate_column_alias: ColumnAlias,
    select_set: SelectSet,
) -> Select {
    match select_set {
        SelectSet::Rows(row_select) => {
            let rows_row = vec![(
                make_column_alias("rows".to_string()),
                Expression::FunctionCall {
                    function: Function::IsNull,
                    args: vec![
                        Expression::ColumnReference(ColumnReference::AliasedColumn {
                            column: row_column_alias.clone(),
                            table: TableReference::AliasedTable(row_table_alias.clone()),
                        }),
                        Expression::Value(Value::EmptyJsonArray),
                    ],
                },
            )];

            let mut final_row_select = simple_select(rows_row);

            final_row_select.from = Some(From::Select {
                alias: row_table_alias.clone(),
                select: Box::new(row_select),
                alias_path: AliasPath {
                    elements: vec![row_column_alias],
                },
            });
            final_row_select.for_json = ForJson::ForJsonPathWithoutArrayWrapper;

            final_row_select
        }
        SelectSet::Aggregates(aggregate_select) => {
            let aggregates_row = vec![(
                make_column_alias("aggregates".to_string()),
                Expression::JsonQuery(
                    Box::new(Expression::ColumnReference(
                        ColumnReference::AliasedColumn {
                            column: aggregate_column_alias.clone(),
                            table: TableReference::AliasedTable(aggregate_table_alias.clone()),
                        },
                    )),
                    JsonPath { elements: vec![] },
                ),
            )];

            let mut final_aggregate_select = simple_select(aggregates_row);

            final_aggregate_select.from = Some(From::Select {
                alias: aggregate_table_alias,
                select: Box::new(aggregate_select),
                alias_path: AliasPath {
                    elements: vec![aggregate_column_alias],
                },
            });

            final_aggregate_select.for_json = ForJson::ForJsonPathWithoutArrayWrapper;

            final_aggregate_select
        }
        SelectSet::RowsAndAggregates(row_select, aggregate_select) => {
            let both_row = vec![
                (
                    make_column_alias("rows".to_string()),
                    Expression::JsonQuery(
                        Box::new(Expression::FunctionCall {
                            function: Function::IsNull,
                            args: vec![
                                Expression::ColumnReference(ColumnReference::AliasedColumn {
                                    column: row_column_alias.clone(),
                                    table: TableReference::AliasedTable(row_table_alias.clone()),
                                }),
                                Expression::Value(Value::EmptyJsonArray),
                            ],
                        }),
                        JsonPath {
                            elements: vec![row_column_alias.clone()],
                        },
                    ),
                ),
                (
                    make_column_alias("aggregates".to_string()),
                    Expression::JsonQuery(
                        Box::new(Expression::JsonValue(
                            Box::new(Expression::ColumnReference(
                                ColumnReference::AliasedColumn {
                                    column: aggregate_column_alias.clone(),
                                    table: TableReference::AliasedTable(
                                        aggregate_table_alias.clone(),
                                    ),
                                },
                            )),
                            JsonPath {
                                elements: vec![aggregate_column_alias.clone()],
                            },
                        )),
                        JsonPath { elements: vec![] },
                    ),
                ),
            ];

            let mut final_select = simple_select(both_row);

            let mut row_select_star = star_select(From::Select {
                alias: output_table_alias,
                select: Box::new(row_select),
                alias_path: AliasPath {
                    elements: vec![row_column_alias.clone()],
                },
            });

            row_select_star.for_json = ForJson::ForJsonPathWithoutArrayWrapper;

            let mut aggregate_select_star = star_select(From::Select {
                alias: aggregate_table_alias.clone(),
                select: Box::new(aggregate_select),
                alias_path: AliasPath {
                    elements: vec![aggregate_column_alias.clone()],
                },
            });

            aggregate_select_star.for_json = ForJson::ForJsonPathWithoutArrayWrapper;

            final_select.joins = vec![Join::CrossJoin(CrossJoin {
                select: Box::new(aggregate_select_star),
                alias: aggregate_table_alias.clone(),
                alias_path: AliasPath {
                    elements: vec![aggregate_column_alias],
                },
            })];

            final_select.from = Some(From::Select {
                alias: row_table_alias,
                select: Box::new(row_select_star),
                alias_path: AliasPath {
                    elements: vec![row_column_alias],
                },
            });

            final_select.for_json = ForJson::ForJsonPathWithoutArrayWrapper;

            final_select
        }
    }
}

pub fn select_mutation_rowset(
    (result_table_alias, result_column_alias): (TableAlias, ColumnAlias),
    (row_table_alias, row_column_alias): (TableAlias, ColumnAlias),
    aggregate_table_alias: TableAlias,
    affected_rows_column_alias: ColumnAlias,
    select_set: SelectSet,
) -> Select {
    let row = |result_json_path| {
        vec![
            (
                make_column_alias("type".to_string()),
                Expression::Value(Value::String("procedure".to_string())),
            ),
            (
                make_column_alias("result".to_string()),
                Expression::JsonQuery(
                    Box::new(Expression::ColumnReference(
                        ColumnReference::AliasedColumn {
                            table: TableReference::AliasedTable(result_table_alias.clone()),
                            column: result_column_alias.clone(),
                        },
                    )),
                    JsonPath {
                        elements: result_json_path,
                    },
                ),
            ),
        ]
    };

    match select_set {
        SelectSet::Rows(row_select) => {
            let rows_row = vec![(
                row_column_alias.clone(),
                Expression::FunctionCall {
                    function: Function::IsNull,
                    args: vec![
                        Expression::ColumnReference(ColumnReference::AliasedColumn {
                            column: row_column_alias.clone(),
                            table: TableReference::AliasedTable(row_table_alias.clone()),
                        }),
                        Expression::Value(Value::EmptyJsonArray),
                    ],
                },
            )];

            let mut row_select_subquery = simple_select(rows_row);

            row_select_subquery.from = Some(From::Select {
                select: Box::new(row_select),
                alias: row_table_alias,
                alias_path: AliasPath {
                    elements: vec![row_column_alias],
                },
            });

            row_select_subquery.for_json = ForJson::ForJsonPathWithoutArrayWrapper;

            let mut final_select = simple_select(row(vec![]));

            final_select.from = Some(From::Select {
                select: Box::new(row_select_subquery),
                alias: result_table_alias,
                alias_path: AliasPath {
                    elements: vec![result_column_alias],
                },
            });

            final_select.for_json = ForJson::ForJsonPathWithoutArrayWrapper;

            final_select
        }

        SelectSet::Aggregates(aggregate_select) => {
            let aggregates_row = vec![(
                make_column_alias("aggregates".to_string()),
                Expression::JsonQuery(
                    Box::new(Expression::ColumnReference(
                        ColumnReference::AliasedColumn {
                            column: make_column_alias("aggregates".to_string()),
                            table: TableReference::AliasedTable(aggregate_table_alias.clone()),
                        },
                    )),
                    JsonPath { elements: vec![] },
                ),
            )];

            let mut final_aggregate_select = simple_select(aggregates_row);

            final_aggregate_select.from = Some(From::Select {
                alias: aggregate_table_alias,
                select: Box::new(aggregate_select),
                alias_path: AliasPath {
                    elements: vec![make_column_alias("aggregates".to_string())],
                },
            });

            final_aggregate_select.for_json = ForJson::ForJsonPathWithoutArrayWrapper;

            let mut final_select =
                simple_select(row(vec![make_column_alias("aggregates".to_string())]));

            final_select.from = Some(From::Select {
                select: Box::new(final_aggregate_select),
                alias: result_table_alias,
                alias_path: AliasPath {
                    elements: vec![result_column_alias],
                },
            });

            final_select.for_json = ForJson::ForJsonPathWithoutArrayWrapper;

            final_select
        }
        SelectSet::RowsAndAggregates(row_select, aggregate_select) => {
            let both_row = vec![
                (
                    row_column_alias.clone(),
                    Expression::JsonQuery(
                        Box::new(Expression::FunctionCall {
                            function: Function::IsNull,
                            args: vec![
                                Expression::ColumnReference(ColumnReference::AliasedColumn {
                                    column: row_column_alias.clone(),
                                    table: TableReference::AliasedTable(row_table_alias.clone()),
                                }),
                                Expression::Value(Value::EmptyJsonArray),
                            ],
                        }),
                        JsonPath {
                            elements: vec![row_column_alias.clone()],
                        },
                    ),
                ),
                (
                    affected_rows_column_alias.clone(),
                    Expression::Cast {
                        expression: Box::new(Expression::JsonValue(
                            Box::new(Expression::JsonValue(
                                Box::new(Expression::ColumnReference(
                                    ColumnReference::AliasedColumn {
                                        column: make_column_alias("aggregates".to_string()),
                                        table: TableReference::AliasedTable(
                                            aggregate_table_alias.clone(),
                                        ),
                                    },
                                )),
                                JsonPath {
                                    elements: vec![make_column_alias("aggregates".to_string())],
                                },
                            )),
                            JsonPath {
                                elements: vec![affected_rows_column_alias],
                            },
                        )),
                        r#type: ScalarType("int".to_string()),
                    },
                ),
            ];

            let mut final_select = simple_select(both_row);

            let mut row_select_star = star_select(From::Select {
                alias: row_table_alias.clone(),
                select: Box::new(row_select),
                alias_path: AliasPath {
                    elements: vec![row_column_alias.clone()],
                },
            });

            row_select_star.for_json = ForJson::ForJsonPathWithoutArrayWrapper;

            let mut aggregate_select_star = star_select(From::Select {
                alias: aggregate_table_alias.clone(),
                select: Box::new(aggregate_select),
                alias_path: AliasPath {
                    elements: vec![make_column_alias("aggregates".to_string())],
                },
            });

            aggregate_select_star.for_json = ForJson::ForJsonPathWithoutArrayWrapper;

            final_select.joins = vec![Join::CrossJoin(CrossJoin {
                select: Box::new(aggregate_select_star),
                alias: aggregate_table_alias.clone(),
                alias_path: AliasPath {
                    elements: vec![make_column_alias("aggregates".to_string())],
                },
            })];

            final_select.from = Some(From::Select {
                alias: row_table_alias,
                select: Box::new(row_select_star),
                alias_path: AliasPath {
                    elements: vec![row_column_alias],
                },
            });

            final_select.for_json = ForJson::ForJsonPathWithoutArrayWrapper;

            let mut row_final_select = simple_select(row(vec![]));

            row_final_select.from = Some(From::Select {
                select: Box::new(final_select),
                alias: result_table_alias,
                alias_path: AliasPath {
                    elements: vec![result_column_alias],
                },
            });

            row_final_select.for_json = ForJson::ForJsonPathWithoutArrayWrapper;

            row_final_select
        }
    }
}
