//! Helpers for processing the QueryRequest and building SQL.

use std::collections::BTreeMap;

use ndc_models::ArgumentName;

use super::error::Error;
use crate::translation::values;
use query_engine_metadata::metadata::{self, NativeQuerySql};
use query_engine_sql::sql;

#[derive(Debug)]
/// Static information from the query and metadata.
pub struct Env<'a> {
    pub metadata: &'a metadata::Metadata,
    relationships: BTreeMap<ndc_models::RelationshipName, ndc_models::Relationship>,
}

#[derive(Debug)]
/// Stateful information changed throughout the translation process.
pub struct State {
    native_queries: NativeQueries,
    mutations: Vec<MutationOperation>,
    global_table_index: TableAliasIndex,
}

#[derive(Debug)]
pub struct TableAliasIndex(pub u64);

#[derive(Debug)]
/// Store top-level native queries generated throughout the translation process.
///
/// Native queries are implemented as `WITH <native_query_name_<index>> AS (<native_query>) <query>`
struct NativeQueries {
    /// native queries that receive different arguments should result in different CTEs,
    /// and be used via a AliasedTable in the query.
    native_queries: Vec<NativeQueryInfo>,
}

impl NativeQueries {
    fn new() -> NativeQueries {
        NativeQueries {
            native_queries: vec![],
        }
    }
}

#[derive(Debug)]
pub struct MutationOperation {
    pub name: String,
    pub arguments: BTreeMap<ndc_models::ArgumentName, serde_json::Value>,
    pub fields: Option<ndc_models::NestedField>,
    pub kind: MutationOperationKind,
}

#[derive(Debug)]
pub enum MutationOperationKind {
    NativeMutation(NativeMutationInfo),
    StoredProcedure(StoredProcedureInfo),
}

#[derive(Debug)]
/// Information we store about a native query call.
pub struct NativeQueryInfo {
    pub info: metadata::NativeQueryInfo,
    pub arguments: BTreeMap<ndc_models::ArgumentName, ndc_models::Argument>,
    pub alias: sql::ast::TableAlias,
}

#[derive(Debug)]
pub struct NativeMutationInfo {
    /// Name of the native mutation
    pub name: String,
    pub info: metadata::NativeMutationInfo,
}

#[derive(Debug)]
pub struct StoredProcedureInfo {
    /// Name of the stored procedure
    pub name: String,
    pub info: metadata::stored_procedures::StoredProcedureInfo,
}

/// For the root table in the query, and for the current table we are processing,
/// We'd like to track what is their reference in the query (the name we can use to address them,
/// an alias we generate), and what is their name in the metadata (so we can get
/// their information such as which columns are available for that table).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootAndCurrentTables {
    /// The root (top-most) table in the query.
    pub root_table: TableNameAndReference,
    /// The current table we are processing.
    pub current_table: TableNameAndReference,
}

/// For a table in the query, We'd like to track what is its reference in the query
/// (the name we can use to address them, an alias we generate), and what is their name in the
/// metadata (so we can get their information such as which columns are available for that table).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableNameAndReference {
    /// Table name for column lookup
    pub name: String,
    /// Table alias to query from
    pub reference: sql::ast::TableReference,
}

#[derive(Debug)]
/// Information about columns
pub struct ColumnInfo {
    pub name: sql::ast::ColumnName,
    pub r#type: metadata::ScalarType,
}

#[derive(Debug)]
/// Metadata information about a specific collection.
pub enum CollectionInfo {
    Table {
        name: String,
        info: metadata::TableInfo,
    },
    NativeQuery {
        name: String,
        info: metadata::NativeQueryInfo,
    },
    NativeMutation {
        name: String,
        info: metadata::NativeMutationInfo,
    },
}

#[derive(Debug)]
/// Metadata information about a procedure.
pub enum ProcedureInfo {
    NativeMutation {
        name: String,
        info: metadata::NativeMutationInfo,
    },
    StoredProcedure {
        name: String,
        info: metadata::stored_procedures::StoredProcedureInfo,
    },
}

#[derive(Debug)]
pub enum CollectionOrProcedureInfo {
    Collection(CollectionInfo),
    Procedure(ProcedureInfo),
}

/// Substitutes the value of the arguments
/// in the parameterized SQL statement and returns
/// a SQL statement that can be run in the DB.
pub fn generate_native_query_sql(
    type_arguments: &BTreeMap<ArgumentName, query_engine_metadata::metadata::ColumnInfo>,
    native_query_arguments: &BTreeMap<ndc_models::ArgumentName, ndc_models::Argument>,
    native_query_sql: &NativeQuerySql,
) -> Result<Vec<sql::ast::RawSql>, Error> {
    native_query_sql
        .0
        .iter()
        .map(|part| match part {
            metadata::NativeQueryPart::Text(text) => Ok(sql::ast::RawSql::RawText(text.clone())),
            metadata::NativeQueryPart::Parameter(param) => {
                let typ = match type_arguments.get(param) {
                    None => Err(Error::ArgumentNotFound(param.to_string())),
                    Some(argument) => Ok(argument.r#type.clone()),
                }?;

                let exp = match native_query_arguments.get(param.as_str()) {
                    None => Err(Error::ArgumentNotFound(param.to_string())),
                    Some(argument) => match argument {
                        ndc_models::Argument::Literal { value } => {
                            values::translate_json_value(value, &typ)
                        }
                        ndc_models::Argument::Variable { name } => {
                            Ok(values::translate_variable(name, &typ))
                        }
                    },
                }?;
                Ok(sql::ast::RawSql::Expression(exp))
            }
        })
        .collect()
}

impl<'a> Env<'a> {
    /// Create a new Env by supplying the metadata and relationships.
    pub fn new(
        metadata: &'a metadata::Metadata,
        relationships: BTreeMap<ndc_models::RelationshipName, ndc_models::Relationship>,
    ) -> Env<'a> {
        Env {
            metadata,
            relationships,
        }
    }

    // TODO(KC): Modify the `lookup` functions to not lookup twice. This can be
    // done by creating a schema from the metadata and collecting all the `procedure`
    // `query` together.

    pub fn lookup_procedure(&self, procedure_name: &str) -> Option<ProcedureInfo> {
        let native_mutation =
            self.metadata
                .native_mutations
                .0
                .get(procedure_name)
                .map(|native_mutation_info| ProcedureInfo::NativeMutation {
                    name: procedure_name.to_string(),
                    info: native_mutation_info.clone(),
                });

        match native_mutation {
            None => {
                self.metadata
                    .stored_procedures
                    .0
                    .get(procedure_name)
                    .map(|stored_procedure_info| ProcedureInfo::StoredProcedure {
                        name: procedure_name.to_string(),
                        info: stored_procedure_info.clone(),
                    })
            }
            Some(native_mutation) => Some(native_mutation),
        }
    }

    /// Lookup a collection's information in the metadata.
    pub fn lookup_collection(
        &self,
        collection_name: &ndc_models::CollectionName,
    ) -> Result<CollectionOrProcedureInfo, Error> {
        let table = self
            .metadata
            .tables
            .0
            .get(collection_name.as_str())
            .map(|t| CollectionInfo::Table {
                name: collection_name.to_string(),
                info: t.clone(),
            });

        match table {
            Some(table) => Ok(CollectionOrProcedureInfo::Collection(table)),
            None => {
                let proc_maybe = self.lookup_procedure(collection_name.as_str());

                match proc_maybe {
                    Some(proc_info) => Ok(CollectionOrProcedureInfo::Procedure(proc_info)),
                    None => {
                        let native_query = self
                            .metadata
                            .native_queries
                            .0
                            .get(collection_name.as_str())
                            .map(|nq| CollectionInfo::NativeQuery {
                                name: collection_name.to_string(),
                                info: nq.clone(),
                            });
                        // FIXME(KC): THis is terrible. Please refactor this.
                        match native_query {
                            Some(native_query) => {
                                Ok(CollectionOrProcedureInfo::Collection(native_query))
                            }
                            None => self
                                .metadata
                                .native_mutations
                                .0
                                .get(collection_name.as_str())
                                .map(|nq| CollectionInfo::NativeMutation {
                                    name: collection_name.to_string(),
                                    info: nq.clone(),
                                })
                                .map(CollectionOrProcedureInfo::Collection)
                                .ok_or(Error::CollectionNotFound(collection_name.to_string())),
                        }
                    }
                }
            }
        }
    }

    pub fn lookup_relationship(
        &self,
        name: &ndc_models::RelationshipName,
    ) -> Result<&ndc_models::Relationship, Error> {
        self.relationships
            .get(name.as_str())
            .ok_or(Error::RelationshipNotFound(name.to_string()))
    }

    /// Looks up the binary comparison operator's MSSQL name and arguments' type in the metadata.
    pub fn lookup_comparison_operator(
        &self,
        scalar_type: &metadata::ScalarType,
        name: &ndc_models::ComparisonOperatorName,
    ) -> Result<&'a metadata::ComparisonOperator, Error> {
        self.metadata
            .comparison_operators
            .0
            .get(scalar_type)
            .and_then(|ops| ops.get(name.as_str()))
            .ok_or(Error::OperatorNotFound {
                operator_name: name.to_string(),
                type_name: scalar_type.clone(),
            })
    }
}

impl CollectionOrProcedureInfo {
    pub fn lookup_column(&self, column_name: &ndc_models::FieldName) -> Result<ColumnInfo, Error> {
        match &self {
            CollectionOrProcedureInfo::Collection(collection_info) => {
                collection_info.lookup_column(column_name.as_str())
            }
            CollectionOrProcedureInfo::Procedure(procedure_info) => {
                procedure_info.lookup_column(column_name.as_str())
            }
        }
    }
}

impl ProcedureInfo {
    pub fn lookup_column(&self, column_name: &str) -> Result<ColumnInfo, Error> {
        match self {
            ProcedureInfo::NativeMutation { name, info } => info
                .columns
                .get(column_name)
                .ok_or(Error::ColumnNotFoundInProcedure(
                    column_name.to_string(),
                    name.to_string(),
                ))
                .map(|c| column_info_to_sql_column_info(&c.column_info)),
            ProcedureInfo::StoredProcedure { name, info } => info
                .returns
                .clone()
                .unwrap_or_default()
                .get(column_name)
                .ok_or(Error::ColumnNotFoundInProcedure(
                    column_name.to_string(),
                    name.to_string(),
                ))
                .map(column_info_to_sql_column_info),
        }
    }
}

fn column_info_to_sql_column_info(
    column_info: &query_engine_metadata::metadata::ColumnInfo,
) -> ColumnInfo {
    ColumnInfo {
        name: sql::ast::ColumnName(column_info.name.clone()),
        r#type: column_info.r#type.clone(),
    }
}

impl CollectionInfo {
    /// Lookup a column in a collection.
    pub fn lookup_column(&self, column_name: &str) -> Result<ColumnInfo, Error> {
        match self {
            CollectionInfo::Table { name, info } => info
                .columns
                .get(column_name)
                .map(column_info_to_sql_column_info)
                .ok_or(Error::ColumnNotFoundInCollection(
                    column_name.to_string(),
                    name.clone(),
                )),
            CollectionInfo::NativeQuery { name, info } => info
                .columns
                .get(column_name)
                .map(column_info_to_sql_column_info)
                .ok_or(Error::ColumnNotFoundInCollection(
                    column_name.to_string(),
                    name.clone(),
                )),
            // TODO(KC): Remove this, once we refactor lookup_collection.
            CollectionInfo::NativeMutation { name, info } => info
                .columns
                .get(column_name)
                .map(|column_info| ColumnInfo {
                    name: sql::ast::ColumnName(column_info.column_info.name.clone()),
                    r#type: column_info.column_info.r#type.clone(),
                })
                .ok_or(Error::ColumnNotFoundInCollection(
                    column_name.to_string(),
                    name.clone(),
                )),
        }
    }
}

impl Default for State {
    fn default() -> State {
        State {
            native_queries: NativeQueries::new(),
            mutations: Vec::new(),
            global_table_index: TableAliasIndex(0),
        }
    }
}

impl State {
    /// Build a new state.
    pub fn new() -> State {
        State::default()
    }

    /// Introduce a new native query to the generated sql.
    pub fn insert_native_query(
        &mut self,
        name: &str,
        info: metadata::NativeQueryInfo,
        arguments: BTreeMap<ndc_models::ArgumentName, ndc_models::Argument>,
    ) -> sql::ast::TableReference {
        let alias = self.make_native_query_table_alias(name);
        self.native_queries.native_queries.push(NativeQueryInfo {
            info,
            arguments,
            alias: alias.clone(),
        });
        sql::ast::TableReference::AliasedTable(alias)
    }

    /// Fetch the tracked native queries used in the query plan and their table alias.
    pub fn get_native_queries(self) -> Vec<NativeQueryInfo> {
        self.native_queries.native_queries
    }

    pub fn get_mutation_operations(self) -> Vec<MutationOperation> {
        self.mutations
    }

    /// increment the table index and return the current one.
    fn next_global_table_index(&mut self) -> TableAliasIndex {
        let TableAliasIndex(index) = self.global_table_index;
        self.global_table_index = TableAliasIndex(index + 1);
        TableAliasIndex(index)
    }

    // aliases

    /// Create table aliases using this function so they get a unique index.
    pub fn make_table_alias(&mut self, name: String) -> sql::ast::TableAlias {
        sql::ast::TableAlias {
            unique_index: self.next_global_table_index().0,
            name,
            is_temporary_table: false,
        }
    }

    /// Create table aliases using this function so they get a unique index.
    pub fn make_temporary_table_alias(&mut self, name: String) -> sql::ast::TableAlias {
        sql::ast::TableAlias {
            unique_index: self.next_global_table_index().0,
            name,
            is_temporary_table: true,
        }
    }

    /// Create a table alias for left outer join lateral part.
    /// Provide an index and a source table name so we avoid name clashes,
    /// and get an alias.
    pub fn make_relationship_table_alias(&mut self, name: &String) -> sql::ast::TableAlias {
        self.make_table_alias(format!("RELATIONSHIP_{name}"))
    }

    /// Create a table alias for order by target part.
    /// Provide an index and a source table name (to disambiguate the table being queried),
    /// and get an alias.
    pub fn make_order_path_part_table_alias(&mut self, table_name: &str) -> sql::ast::TableAlias {
        self.make_table_alias(format!("ORDER_PART_{table_name}"))
    }

    /// Create a table alias for order by column.
    /// Provide an index and a source table name (to point at the table being ordered),
    /// and get an alias.
    pub fn make_order_by_table_alias(
        &mut self,
        source_table_name: &String,
    ) -> sql::ast::TableAlias {
        self.make_table_alias(format!("ORDER_FOR_{source_table_name}"))
    }

    /// Create a table alias for count aggregate order by column.
    /// Provide an index and a source table name /// (to point at the table being ordered),
    /// and get an alias.
    pub fn make_order_by_count_table_alias(
        &mut self,
        source_table_name: &String,
    ) -> sql::ast::TableAlias {
        self.make_table_alias(format!("ORDER_COUNT_FOR_{source_table_name}"))
    }

    pub fn make_native_query_table_alias(&mut self, name: &str) -> sql::ast::TableAlias {
        self.make_table_alias(format!("NATIVE_QUERY_{name}"))
    }

    pub fn make_stored_procedure_table_alias(&mut self, name: &str) -> sql::ast::TableAlias {
        self.make_temporary_table_alias(format!("STORED_PROCEDURE_{name}"))
    }

    /// Create a table alias for boolean expressions.
    /// Provide state for fresh names and a source table name (to point at the table
    /// being filtered), and get an alias.
    pub fn make_boolean_expression_table_alias(
        &mut self,
        source_table_name: &String,
    ) -> sql::ast::TableAlias {
        self.make_table_alias(format!("BOOLEXP_{source_table_name}"))
    }
}
