//! Metadata information regarding the database and tracked information.

use models::{AggregateFunctionName, CollectionName, ComparisonOperatorName, FieldName};
use ndc_models as models;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// The scalar types supported by the Engine.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScalarType(pub String);

/// The type of values that a column, field, or argument may take.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    ScalarType(ScalarType),
    CompositeType(String),
    ArrayType(Box<Type>),
}

/// The complete list of supported binary operators for scalar types.
/// Not all of these are supported for every type.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ComparisonOperators(
    pub BTreeMap<ScalarType, BTreeMap<ComparisonOperatorName, ComparisonOperator>>,
);

/// Represents a postgres binary comparison operator
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ComparisonOperator {
    pub operator_name: String,
    pub argument_type: ScalarType,
    pub operator_kind: OperatorKind,
}

/// Is it a built-in operator, or a custom operator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum OperatorKind {
    Equal,
    In,
    Custom,
}

/// Mapping from a "table" name to its information.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TablesInfo(pub BTreeMap<CollectionName, TableInfo>);

/// Information about a database table (or any other kind of relation).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TableInfo {
    pub schema_name: String,
    pub table_name: String,
    pub columns: BTreeMap<FieldName, ColumnInfo>,
    #[serde(default)]
    pub uniqueness_constraints: UniquenessConstraints,
    #[serde(default)]
    pub foreign_relations: ForeignRelations,
    #[serde(default)]
    pub description: Option<String>,
}

/// Can this column contain null values
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Nullable {
    #[default]
    Nullable,
    NonNullable,
}

/// Information about a database column.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ColumnInfo {
    pub name: String,
    pub r#type: ScalarType,
    #[serde(default)]
    pub nullable: Nullable,
    #[serde(default)]
    pub description: Option<String>,
}

/// Information about a native mutation column.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NativeMutationColumnInfo {
    #[serde(flatten)]
    pub column_info: ColumnInfo,
    /// The database type that the column should be cast as.
    /// For example, if a native mutation query returns a string
    /// for the column 'foo'. Then, this field can be an indicator
    /// to cast the value as an integer. When `cast_as` is `None`,
    /// the value is casted to `r#type`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cast_as: Option<String>,
}

/// A mapping from the name of a unique constraint to its value.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UniquenessConstraints(pub BTreeMap<String, UniquenessConstraint>);

/// The set of columns that make up a uniqueness constraint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UniquenessConstraint(pub BTreeSet<FieldName>);

/// A mapping from the name of a foreign key constraint to its value.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ForeignRelations(pub BTreeMap<String, ForeignRelation>);

/// A foreign key constraint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ForeignRelation {
    pub foreign_table: String,
    pub column_mapping: BTreeMap<FieldName, FieldName>,
}

/// All supported aggregate functions, grouped by type.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AggregateFunctions(
    pub BTreeMap<ScalarType, BTreeMap<AggregateFunctionName, AggregateFunction>>,
);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AggregateFunction {
    pub return_type: ScalarType,
}
