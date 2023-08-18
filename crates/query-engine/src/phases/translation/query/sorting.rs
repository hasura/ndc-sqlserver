use std::collections::BTreeMap;

use ndc_hub::models;

use super::error::Error;
use super::helpers::{RootAndCurrentTables, TableNameAndReference};
use super::relationships;
use crate::metadata;
use crate::phases::translation::sql;

// if we are using limit / offset then we MUST have an ordering too, so this orders by the first
// unique column we see
pub fn default_order_by(
    table_info: &metadata::TableInfo,
    table_name: sql::ast::TableName,
) -> Result<sql::ast::OrderBy, Error> {
    tracing::info!("{:?}", table_info);
    match &table_info.uniqueness_constraints {
        metadata::UniquenessConstraints(uniques_map) => match uniques_map.clone().pop_first() {
            Some((_, metadata::UniquenessConstraint(constraint))) => {
                let order_by_element = sql::ast::OrderByElement {
                    direction: sql::ast::OrderByDirection::Asc,
                    target: sql::ast::Expression::ColumnName(sql::ast::ColumnName::AliasedColumn {
                        table: table_name,
                        name: sql::helpers::make_column_alias(
                            constraint.first().unwrap().to_string(),
                        ),
                    }),
                };

                Ok(sql::ast::OrderBy {
                    elements: vec![order_by_element],
                })
            }
            None => Err(Error::NoConstraintsForOrdering),
        },
    }
}

/// Convert the order by fields from a QueryRequest to a SQL ORDER BY clause and potentially
/// JOINs when we order by relationship fields.
pub fn translate_order_by(
    tables_info: &metadata::TablesInfo,
    relationships: &BTreeMap<String, models::Relationship>,
    root_and_current_tables: &RootAndCurrentTables,
    order_by: &Option<models::OrderBy>,
) -> Result<(sql::ast::OrderBy, Vec<sql::ast::Join>), Error> {
    let mut joins: Vec<sql::ast::Join> = vec![];

    // For each order_by field, extract the relevant field name, direction, and join table (if relevant).
    match order_by {
        None => Ok((sql::ast::OrderBy { elements: vec![] }, vec![])),
        Some(models::OrderBy { elements }) => {
            let order_by_parts = elements
                .iter()
                .enumerate() // We enumerate to give each query a unique alias.
                .map(|(index, order_by)| {
                    let target = match &order_by.target {
                        models::OrderByTarget::Column { name, path } => {
                            let (column_alias, optional_relationship_select) =
                                translate_order_by_target_for_column(
                                    tables_info,
                                    relationships,
                                    name.clone(),
                                    path,
                                )?;

                            match optional_relationship_select {
                                // The column is from the source table, we just need to query it directly
                                // by refering to the table's alias.
                                None => {
                                    let column_name = sql::ast::Expression::ColumnName(
                                        sql::ast::ColumnName::AliasedColumn {
                                            table: sql::ast::TableName::AliasedTable(
                                                root_and_current_tables
                                                    .current_table
                                                    .reference
                                                    .clone(),
                                            ),
                                            name: column_alias,
                                        },
                                    );

                                    Ok(column_name)
                                }

                                // The column is from a relationship table, we need to join with this
                                // select query.
                                Some(select) => {
                                    // Give it a nice unique alias.
                                    let table_alias = sql::helpers::make_order_by_table_alias(
                                        index,
                                        &root_and_current_tables.current_table.name,
                                    );

                                    let whole_column_alias =
                                        sql::helpers::make_column_alias("json".to_string());

                                    // Build a join and push it to the accumulated joins.
                                    let new_join = sql::ast::OuterApply {
                                        select: Box::new(select),
                                        alias: table_alias.clone(),
                                        alias_path: sql::ast::AliasPath {
                                            elements: vec![whole_column_alias],
                                        },
                                    };

                                    joins.push(sql::ast::Join::OuterApply(new_join));

                                    // Build an alias to query the column from this select.
                                    let column_name = sql::ast::Expression::ColumnName(
                                        sql::ast::ColumnName::AliasedColumn {
                                            table: sql::ast::TableName::AliasedTable(table_alias),
                                            name: column_alias,
                                        },
                                    );

                                    Ok(column_name)
                                }
                            }
                        }

                        models::OrderByTarget::SingleColumnAggregate { .. } => Err(
                            Error::NotSupported("order by column aggregates".to_string()),
                        ),
                        models::OrderByTarget::StarCountAggregate { path } => {
                            let (column_alias, select) = translate_order_by_star_count_aggregate(
                                tables_info,
                                relationships,
                                &root_and_current_tables.current_table,
                                path,
                            )?;

                            // Give it a nice unique alias.
                            let table_alias = sql::helpers::make_table_alias(format!(
                                "%ORDER_{}_COUNT_{}",
                                index, root_and_current_tables.current_table.name
                            ));

                            let whole_column_alias =
                                sql::helpers::make_column_alias("json".to_string());

                            // Build a join ...
                            let new_join = sql::ast::OuterApply {
                                select: Box::new(select),
                                alias: table_alias.clone(),
                                alias_path: sql::ast::AliasPath {
                                    elements: vec![whole_column_alias],
                                },
                            };

                            // ... push it to the accumulated joins.
                            joins.push(sql::ast::Join::OuterApply(new_join));

                            // Build an alias to query the column from this select.
                            let column_name = sql::ast::Expression::ColumnName(
                                sql::ast::ColumnName::AliasedColumn {
                                    table: sql::ast::TableName::AliasedTable(table_alias),
                                    name: column_alias,
                                },
                            );

                            // return the column to order by (from our fancy join)
                            Ok(column_name)
                        }
                    }?;
                    let direction = match order_by.order_direction {
                        models::OrderDirection::Asc => sql::ast::OrderByDirection::Asc,
                        models::OrderDirection::Desc => sql::ast::OrderByDirection::Desc,
                    };
                    Ok(sql::ast::OrderByElement { target, direction })
                })
                .collect::<Result<Vec<sql::ast::OrderByElement>, Error>>()?;

            Ok((
                sql::ast::OrderBy {
                    elements: order_by_parts,
                },
                joins,
            ))
        }
    }
}

// a StarCountAggregate allows us to express stuff like "order albums by number of tracks they have",
// ie order by a COUNT(*) over the items of an array relationship
fn translate_order_by_star_count_aggregate(
    tables_info: &metadata::TablesInfo,
    relationships: &BTreeMap<String, models::Relationship>,
    source_table: &TableNameAndReference,
    path: &Vec<models::PathElement>,
) -> Result<(sql::ast::ColumnAlias, sql::ast::Select), Error> {
    // we can only do one level of star count aggregate atm
    if path.len() > 1 {
        Err(Error::NotSupported(
            "star count for nested relationships".to_string(),
        ))
    } else {
        Ok(())
    }?;

    match path.get(0) {
        Some(path_element) => {
            let models::PathElement {
                relationship: relationship_name,
                ..
            } = path_element;

            // examine the path elements' relationship.
            let relationship = relationships
                .get(relationship_name)
                .ok_or(Error::RelationshipNotFound(relationship_name.clone()))?;

            let target_collection_alias: sql::ast::TableAlias =
                sql::helpers::make_table_alias(relationship.target_collection.clone());

            let target_collection_alias_name: sql::ast::TableName =
                sql::ast::TableName::AliasedTable(target_collection_alias.clone());

            // make a very basic select COUNT(*) as "Count" FROM
            // <nested-table> WHERE <join-conditions>
            let column_alias = sql::helpers::make_column_alias("count".to_string());

            let select_cols = vec![(
                column_alias.clone(),
                sql::ast::Expression::Count(sql::ast::CountType::Star),
            )];

            // build a select query from this table where join condition.
            let mut select = sql::helpers::simple_select(select_cols);

            // generate a condition for this join.
            let join_condition = relationships::translate_column_mapping(
                tables_info,
                source_table,
                sql::helpers::empty_where(),
                relationship,
            )?;

            select.where_ = sql::ast::Where(join_condition);

            select.from = Some(sql::ast::From::Table {
                name: target_collection_alias_name,
                alias: target_collection_alias,
            });

            // return the column to order by (from our fancy join)
            Ok((column_alias, select))
        }
        None => Err(Error::NotSupported(
            "order by star count aggregates".to_string(),
        )),
    }
}

/// Generate a SELECT query representing querying the requested column from a table
/// (potentially a nested one using joins). Return that select query and the requested column alias.
/// If the column is the root table's column, a `None` will be returned.
fn translate_order_by_target_for_column(
    tables_info: &metadata::TablesInfo,
    relationships: &BTreeMap<String, models::Relationship>,
    column_name: String,
    path: &[models::PathElement],
) -> Result<(sql::ast::ColumnAlias, Option<sql::ast::Select>), Error> {
    // We want to build a select query where "Track" is the root table, and "Artist"."Name"
    // is the column we need for the order by. Our query will look like this:
    //
    // > ( SELECT "Artist"."Name" AS "Name" -- wanted column
    // >   FROM
    // >     ( SELECT "Album"."ArtistId" ---- required for the next join condition
    // >       FROM "Album" AS "Album"
    // >       WHERE "Track"."AlbumId" = "Album"."AlbumId" --- requires 'AlbumId' from 'Track'
    // >     ) AS "Album"
    // >   LEFT OUTER JOIN LATERAL
    // >     ( SELECT "Artist"."Name" AS "Name" ---- the wanted column for the order by
    // >       FROM "Artist" AS "Artist" ---- the last relationship table
    // >       WHERE ("Album"."ArtistId" = "Artist"."ArtistId") ---- requires 'ArtistId' from 'Album'
    // >     ) AS "Artist" ON ('true')
    // > )
    //
    // Note that "Track" will be supplied by the caller of this function.

    // We will add joins according to the path element.
    let mut joins: Vec<sql::ast::OuterApply> = vec![];

    // This will be the column we reference in the order by.
    let selected_column_alias = sql::helpers::make_column_alias(column_name);

    // Loop through relationships in reverse order,
    // building up new joins and replacing the selected column for the order by.
    // for each step in the loop we get the required columns (used as keys in the join),
    // from the next join, we need to select these.
    //
    // We don't need the required columns for the first table because we get them for free
    // from the root table.
    let (_, last_table) = path.iter().try_rfold(
        (vec![selected_column_alias.clone()], None),
        |(required_cols, last_table), path_element| {
            let mut last_table = last_table; // make this mut

            // destruct path_element into parts.
            let models::PathElement {
                relationship: relationship_name,
                ..
            } = path_element;

            // examine the path elements' relationship.
            let relationship = relationships
                .get(relationship_name)
                .ok_or(Error::RelationshipNotFound(relationship_name.clone()))?;

            match relationship.relationship_type {
                models::RelationshipType::Array => Err(Error::NotSupported(
                    "Cannot order by values in an array relationship".to_string(),
                )),
                models::RelationshipType::Object => {
                    let source_table_alias: sql::ast::TableAlias = sql::helpers::make_table_alias(
                        relationship.source_collection_or_type.clone(),
                    );

                    let target_collection_alias: sql::ast::TableAlias =
                        sql::helpers::make_table_alias(relationship.target_collection.clone());

                    let target_collection_alias_name: sql::ast::TableName =
                        sql::ast::TableName::AliasedTable(target_collection_alias.clone());

                    // If last_table is None, we are just starting the loop, let's
                    // put a pin on what the last table is, so we can wrap the joins
                    // in a select querying this table.
                    match last_table {
                        None => last_table = Some(target_collection_alias.clone()),
                        Some(_) => {}
                    };

                    // we select the columns used for the next join or the requested column
                    // for the order by.
                    let select_cols: Vec<(sql::ast::ColumnAlias, sql::ast::Expression)> =
                        required_cols
                            .into_iter()
                            .map(|target_col| {
                                let new_table = target_collection_alias_name.clone();
                                (
                                    target_col.clone(),
                                    sql::ast::Expression::ColumnName(
                                        sql::ast::ColumnName::AliasedColumn {
                                            table: new_table,
                                            name: target_col,
                                        },
                                    ),
                                )
                            })
                            .collect();

                    // We find the columns we need from the "previous" table so we can require them.
                    let source_relationship_table_key_columns: Vec<_> = relationship
                        .column_mapping
                        .keys()
                        .map(|source_col| sql::helpers::make_column_alias(source_col.to_string()))
                        .collect();

                    let source_table = TableNameAndReference {
                        name: relationship.source_collection_or_type.clone(),
                        reference: source_table_alias,
                    };

                    // generate a condition for this join.
                    let join_condition = relationships::translate_column_mapping(
                        tables_info,
                        &source_table,
                        sql::helpers::empty_where(),
                        relationship,
                    )?;

                    // build a select query from this table where join condition.
                    let mut select = sql::helpers::simple_select(select_cols);

                    select.where_ = sql::ast::Where(join_condition);

                    select.from = Some(sql::ast::From::Table {
                        name: target_collection_alias_name,
                        alias: target_collection_alias.clone(),
                    });

                    // build a join from it, and
                    let join = sql::ast::OuterApply {
                        select: Box::new(select),
                        alias: target_collection_alias,
                        alias_path: sql::helpers::empty_alias_path(),
                    };

                    // add the join to our pile'o'joins
                    joins.push(join);

                    // return the required columns for this table's join and the last table we found.
                    Ok((source_relationship_table_key_columns, last_table))
                }
            }
        },
    )?;

    match last_table {
        // if there were no relationship columns, we don't need to build a query, just return the table.
        None => Ok((selected_column_alias, None)),
        // If there was a relationship column, build a wrapping select query selecting the wanted column
        // for the order by, and build a select of all the joins to select from.
        Some(last_table) => {
            // order by columns
            let selected_column_expr =
                sql::ast::Expression::ColumnName(sql::ast::ColumnName::AliasedColumn {
                    table: sql::ast::TableName::AliasedTable(last_table),
                    name: selected_column_alias.clone(),
                });
            // wrapping select
            let mut select = sql::helpers::simple_select(vec![(
                selected_column_alias.clone(),
                selected_column_expr,
            )]);

            // build an inner select from the joins by selecting from the first table
            //
            // remember, we traversed the relationships in reverse order, so the joins
            // we built are also in reverse.
            let inner_join = joins
                .pop()
                .expect("last_table was Some, so joins should also be Some.");
            let inner_select = inner_join.select;
            let inner_alias = inner_join.alias;

            joins.reverse();

            // we start from the first table
            select.from = Some(sql::ast::From::Select {
                select: inner_select,
                alias: inner_alias,
                alias_path: sql::helpers::empty_alias_path(),
            });

            // and add the joins
            select.joins = joins
                .into_iter()
                .map(sql::ast::Join::OuterApply)
                .collect::<Vec<sql::ast::Join>>();

            // and return the requested column alias and the inner select.
            Ok((selected_column_alias, Some(select)))
        }
    }
}
