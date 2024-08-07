//! Metadata information regarding the database and tracked information.

pub mod database;
pub mod native_queries;
pub mod stored_procedures;

// re-export without modules
pub use database::*;
pub use native_queries::*;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use self::stored_procedures::StoredProcedures;

/// Metadata information.
#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(default)]
    pub tables: TablesInfo,
    #[serde(default)]
    pub native_queries: NativeQueries,
    #[serde(default)]
    pub native_mutations: NativeMutations,
    #[serde(default)]
    pub aggregate_functions: AggregateFunctions,
    #[serde(default)]
    pub comparison_operators: ComparisonOperators,
    #[serde(default)]
    pub stored_procedures: StoredProcedures,
}
