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
        recursive: false,
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

pub fn empty_alias_path() -> AliasPath {
    AliasPath { elements: vec![] }
}

// Aliasing //

/// Generate a column expression refering to a specific table.
pub fn make_column(
    table: TableName,
    name: String,
    alias: ColumnAlias,
) -> (ColumnAlias, Expression) {
    (
        alias,
        Expression::ColumnName(ColumnName::TableColumn { table, name }),
    )
}

/// Create column aliases using this function so we build everything in one place.
/// We originally wanted indices, but we didn't end up using them.
/// Leaving them here for now, but will probably remove them in the future.
pub fn make_column_alias(name: String) -> ColumnAlias {
    ColumnAlias {
        unique_index: 0,
        name,
    }
}
/// Create table aliases using this function so they get a unique index.
/// We originally wanted indices, but we didn't end up using them.
/// Leaving them here for now, but will probably remove them in the future.
pub fn make_table_alias(name: String) -> TableAlias {
    TableAlias {
        unique_index: 0,
        name,
    }
}

/// Create a table alias for order by column.
/// Provide an index and a source table name (to point at the table being ordered),
/// and get an alias.
pub fn make_order_by_table_alias(index: usize, source_table_name: &String) -> TableAlias {
    make_table_alias(format!("%ORDER_{}_FOR_{}", index, source_table_name))
}

/// Create a table alias for count aggregate order by column.
/// Provide an index and a source table name /// (to point at the table being ordered),
/// and get an alias.
pub fn make_order_by_count_table_alias(index: usize, source_table_name: &String) -> TableAlias {
    make_table_alias(format!("%ORDER_{}_COUNT_FOR_{}", index, source_table_name))
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
        for_json: ForJson::ForJsonPath,
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
/// SELECT JSON_VALUE([aggregates].[json], "$.aggregates") as [aggregates],
///  JSON_QUERY([rows].[json], "$.json") AS [rows]
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
                Expression::ColumnName(ColumnName::AliasedColumn {
                    name: row_column_alias.clone(),
                    table: TableName::AliasedTable(row_table_alias.clone()),
                }),
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
                    Box::new(Expression::ColumnName(ColumnName::AliasedColumn {
                        name: aggregate_column_alias.clone(),
                        table: TableName::AliasedTable(aggregate_table_alias.clone()),
                    })),
                    "$".to_string(),
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
                        Box::new(Expression::ColumnName(ColumnName::AliasedColumn {
                            name: row_column_alias.clone(),
                            table: TableName::AliasedTable(row_table_alias.clone()),
                        })),
                        "$.json".to_string(),
                    ),
                ),
                (
                    make_column_alias("aggregates".to_string()),
                    Expression::JsonQuery(
                        Box::new(Expression::JsonValue(
                            Box::new(Expression::ColumnName(ColumnName::AliasedColumn {
                                name: aggregate_column_alias.clone(),
                                table: TableName::AliasedTable(aggregate_table_alias.clone()),
                            })),
                            "$.json".to_string(),
                        )),
                        "$".to_string(),
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
