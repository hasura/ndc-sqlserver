//! Metadata information regarding native queries.

use super::database::*;

use ndc_models::{ArgumentName, FieldName, ProcedureName};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// Types

/// Metadata information of native queries.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NativeQueries(pub BTreeMap<String, NativeQueryInfo>);

/// Metadata information of native mutations that are supposed to be
/// tracked as mutations.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NativeMutations(pub BTreeMap<ProcedureName, NativeMutationInfo>);

/// Information about a Native Query.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NativeQueryInfo {
    /** SQL expression to use for the Native Query. We can interpolate values using `{{variable_name}}` syntax, such as `SELECT * FROM authors WHERE name = {{author_name}}` */
    pub sql: NativeQuerySql,
    /** Columns returned by the Native Query */
    pub columns: BTreeMap<ndc_models::FieldName, ColumnInfo>,
    #[serde(default)]
    /** Names and types of arguments that can be passed to this Native Query */
    pub arguments: BTreeMap<ndc_models::ArgumentName, ColumnInfo>,
    #[serde(default)]
    pub description: Option<String>,
}

/// Information about a Native Mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NativeMutationInfo {
    /** SQL expression to use for the Native Query. We can interpolate values using `{{variable_name}}` syntax, such as `SELECT * FROM authors WHERE name = {{author_name}}` */
    pub sql: NativeQuerySql,
    /** Columns returned by the Native Query */
    pub columns: BTreeMap<FieldName, NativeMutationColumnInfo>,
    #[serde(default)]
    /** Names and types of arguments that can be passed to this Native Query */
    pub arguments: BTreeMap<ArgumentName, ColumnInfo>,
    #[serde(default)]
    pub description: Option<String>,
}

/// A part of a Native Query text, either raw text or a parameter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeQueryPart {
    /// A raw text part
    Text(String),
    /// A parameter
    Parameter(smol_str::SmolStr),
}

/// A Native Query SQL after parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeQuerySql(pub Vec<NativeQueryPart>);

// Serialization
impl Serialize for NativeQuerySql {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut sql: String = String::new();
        for part in self.0.iter() {
            match part {
                NativeQueryPart::Text(text) => sql.push_str(text.as_str()),
                NativeQueryPart::Parameter(param) => {
                    sql.push_str(format!("{{{{{param}}}}}").as_str())
                }
            }
        }
        serializer.serialize_str(&sql)
    }
}

struct NQVisitor;

impl serde::de::Visitor<'_> for NQVisitor {
    type Value = NativeQuerySql;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A Native Query SQL")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo: write a better parser
        let sql = parse_native_query(value);

        Ok(NativeQuerySql(sql))
    }
}

impl<'de> Deserialize<'de> for NativeQuerySql {
    fn deserialize<D>(deserializer: D) -> Result<NativeQuerySql, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(NQVisitor)
    }
}

impl JsonSchema for NativeQuerySql {
    fn schema_name() -> String {
        "Native_query_sql".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
}

// Parsing

/// Parse a native query into parts where variables have the
/// syntax `{{<variable>}}`.
pub fn parse_native_query(string: &str) -> Vec<NativeQueryPart> {
    let vec: Vec<Vec<NativeQueryPart>> = string
        .split("{{")
        .map(|part| match part.split_once("}}") {
            None => vec![NativeQueryPart::Text(part.to_string())],
            Some((var, text)) => {
                if text.is_empty() {
                    vec![NativeQueryPart::Parameter(var.into())]
                } else {
                    vec![
                        NativeQueryPart::Parameter(var.into()),
                        NativeQueryPart::Text(text.to_string()),
                    ]
                }
            }
        })
        .collect();
    vec.concat()
}

#[cfg(test)]
mod tests {
    use super::{parse_native_query, NativeQueryPart};

    #[test]
    fn no_parameters() {
        assert_eq!(
            parse_native_query("select 1"),
            vec![NativeQueryPart::Text("select 1".to_string())]
        );
    }

    #[test]
    fn one_parameter() {
        assert_eq!(
            parse_native_query("select * from t where {{name}} = name"),
            vec![
                NativeQueryPart::Text("select * from t where ".to_string()),
                NativeQueryPart::Parameter("name".into()),
                NativeQueryPart::Text(" = name".to_string()),
            ]
        );
    }

    #[test]
    fn multiple_parameters() {
        assert_eq!(
            parse_native_query("select * from t where id = {{id}} and {{name}} = {{other_name}}"),
            vec![
                NativeQueryPart::Text("select * from t where id = ".to_string()),
                NativeQueryPart::Parameter("id".into()),
                NativeQueryPart::Text(" and ".to_string()),
                NativeQueryPart::Parameter("name".into()),
                NativeQueryPart::Text(" = ".to_string()),
                NativeQueryPart::Parameter("other_name".into()),
            ]
        );
    }

    #[test]
    fn one_parameter_and_curly_text() {
        assert_eq!(
            parse_native_query("select * from t where {{name}} = '{name}'"),
            vec![
                NativeQueryPart::Text("select * from t where ".to_string()),
                NativeQueryPart::Parameter("name".into()),
                NativeQueryPart::Text(" = '{name}'".to_string()),
            ]
        );
    }
}
