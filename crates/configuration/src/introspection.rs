//! Configuration and state for our connector.
use serde::Deserialize;
use ndc_models::TypeRepresentation;

#[derive(Deserialize, Debug)]
pub struct IntrospectStoredProcedureArgument {
    pub name: String,
    pub r#type: String,
    pub is_nullable: bool,
    pub max_length: u8,
    pub is_output: bool,
}

#[derive(Deserialize, Debug)]
pub struct IntrospectStoredProcedure {
    pub schema: String,
    pub name: String,
    #[serde(default)]
    pub arguments: Vec<IntrospectStoredProcedureArgument>,
}

#[derive(Deserialize, Debug)]
pub struct IntrospectionTable {
    pub name: String,
    pub type_desc: String,
    pub joined_sys_schema: IntrospectionSchema,
    pub joined_sys_column: Vec<IntrospectionColumn>,
    pub joined_sys_primary_key: Option<IntrospectionPrimaryKey>,
}

#[derive(Deserialize, Debug)]
pub struct IntrospectionColumn {
    pub name: String,
    pub is_nullable: bool,
    pub is_identity: bool,
    pub is_computed: bool,
    pub joined_sys_type: IntrospectionType,
    pub joined_foreign_key_columns: Vec<IntrospectionForeignKeyColumn>,
}

#[derive(Deserialize, Debug)]
pub struct IntrospectionType {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct IntrospectionPrimaryKey {
    pub name: String,
    pub columns: Vec<IntrospectionPrimaryKeyColumn>,
}

#[derive(Deserialize, Debug)]
pub struct IntrospectionPrimaryKeyColumn {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct IntrospectionForeignKeyColumn {
    pub joined_referenced_table_name: String,
    pub joined_referenced_column_name: String,
    pub joined_referenced_sys_schema: IntrospectionSchema,
}

#[derive(Deserialize, Debug)]
pub struct IntrospectionSchema {
    pub name: String,
}

pub fn map_type_representation(sql_type: &str) -> Option<TypeRepresentation> {
    match sql_type.to_lowercase().as_str() {
        "bigint" => Some(TypeRepresentation::Int64),
        "bit" => Some(TypeRepresentation::Boolean),
        "decimal" | "numeric" | "money" | "smallmoney" => Some(TypeRepresentation::BigDecimal),
        "int" => Some(TypeRepresentation::Int32),
        "smallint" => Some(TypeRepresentation::Int16),
        "tinyint" => Some(TypeRepresentation::Int16),
        "float" => Some(TypeRepresentation::Float64),
        "real" => Some(TypeRepresentation::Float32),
        "date" => Some(TypeRepresentation::Date),
        "datetime" | "datetime2" | "smalldatetime" | "timestamp" => Some(TypeRepresentation::Timestamp),
        "datetimeoffset" => Some(TypeRepresentation::TimestampTZ),
        "time" => Some(TypeRepresentation::String),
        "char" | "varchar" | "text" | "nchar" | "nvarchar" | "ntext" => Some(TypeRepresentation::String),
        "binary" | "varbinary" | "image" => Some(TypeRepresentation::String),
        "uniqueidentifier" => Some(TypeRepresentation::UUID),
        "xml" => Some(TypeRepresentation::String),
        "geometry" => Some(TypeRepresentation::Geometry),
        "geography" => Some(TypeRepresentation::Geography),
        "json" => Some(TypeRepresentation::JSON),
        // TODO: Add support for hierarchyid and sql_variant
        // "hierarchyid" | "sql_variant" => XXX,
        _ => None,
    }
}
