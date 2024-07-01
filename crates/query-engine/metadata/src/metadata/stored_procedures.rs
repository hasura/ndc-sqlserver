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
    pub nullable: Nullable,
    /// Indicator, if the argument is an `OUTPUT` argument
    /// of the stored procedure.
    #[serde(default)]
    pub is_output: bool,
    #[serde(default)]
    pub description: Option<String>,
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
    /// Columns returned by the stored procedure.
    /// This is set as optional because during the introspection,
    /// we won't know the return type of the stored procedure. We
    /// expect the user to fill this detail manually.
    pub returns: Option<BTreeMap<String, ColumnInfo>>,
    /// Description of the stored procedure.
    pub description: Option<String>,
}

/// Metadata information of the stored procedures.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
pub struct StoredProcedures(pub BTreeMap<String, StoredProcedureInfo>);
