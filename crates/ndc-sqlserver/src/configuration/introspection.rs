//! Configuration and state for our connector.
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct IntrospectionTable {
    pub name: String,
    schema_id: i32,
    pub type_desc: String,
    pub joined_sys_schema: IntrospectionSchema,
    pub joined_sys_column: Vec<IntrospectionColumn>,
    pub joined_sys_primary_key: Option<IntrospectionPrimaryKey>,
}

#[derive(Deserialize, Debug)]
pub struct IntrospectionColumn {
    pub name: String,
    column_id: i32,
    pub is_nullable: bool,
    is_identity: bool,
    is_computed: bool,
    user_type_id: i32,
    pub joined_sys_type: IntrospectionType,
    pub joined_foreign_key_columns: Vec<IntrospectionForeignKeyColumn>,
}

#[derive(Deserialize, Debug)]
pub struct IntrospectionType {
    pub name: String,
    schema_id: i32,
    user_type_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct IntrospectionPrimaryKey {
    pub name: String,
    index_id: i32,
    pub columns: Vec<IntrospectionPrimaryKeyColumn>,
}

#[derive(Deserialize, Debug)]
pub struct IntrospectionPrimaryKeyColumn {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct IntrospectionForeignKeyColumn {
    constraint_object_id: i32,
    constraint_column_id: i32,
    parent_object_id: i32,
    parent_column_id: i32,
    referenced_object_id: i32,
    referenced_column_id: i32,
    pub joined_referenced_table_name: String,
    pub joined_referenced_column_name: String,
    joined_referenced_sys_schema: IntrospectionSchema,
}

#[derive(Deserialize, Debug)]
pub struct IntrospectionSchema {
    pub name: String,
    schema_id: i32,
}
