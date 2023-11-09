//! Configuration and state for our connector.
use serde::Deserialize;

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
