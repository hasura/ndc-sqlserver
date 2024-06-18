use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::{ColumnInfo, Nullable, ScalarType};

/// Information about a stored procedure's argument
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StoredProcedureArgumentInfo {
    /// Name of the argument
    pub name: String,
    /// Type of the argument
    pub r#type: ScalarType,
    /// Nullability of the argument
    pub is_nullable: Nullable,
    /// Indicator, if the argument is an `OUTPUT` argument
    /// of the stored procedure.
    #[serde(default)]
    pub is_output: bool,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum StoredProcedureReturnType {
    // TODO(KC): check if this is feasible?
    // Table {
    //     schema: String,
    //     table: String,
    // },
    /// Columns that are expected to be returned by the stored procedure.
    Columns(BTreeMap<String, ColumnInfo>),
}

/// Information about a stored procedure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StoredProcedureInfo {
    /// Name of the stored procedure
    pub name: String,
    /// Name of the stored procedure's schema.
    pub schema: String,
    /// Arguments to the stored procedure
    pub arguments: BTreeMap<String, StoredProcedureArgumentInfo>,
    #[serde(default)]
    /// Return type of the stored procedure. This should be
    /// filled in by the user, because the return type of a
    /// stored procedure can't be deduced by introspecting the
    /// database.
    pub returns: Option<StoredProcedureReturnType>,
    /// Description of the stored procedure.
    pub description: Option<String>,
}

/// Metadata information of the stored procedures.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
pub struct StoredProcedures(pub BTreeMap<String, StoredProcedureInfo>);
