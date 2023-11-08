//! Configuration and state for our connector.

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct IntrospectionTable {
    name: String,
    schema_id: i32,
    type_desc: String,
    joined_sys_schema: IntrospectionSchema,
    joined_sys_column: Vec<IntrospectionColumn>,
    joined_sys_primary_key: IntrospectionPrimaryKey,
}

#[derive(Deserialize, Debug)]
pub struct IntrospectionColumn {
    name: String,
    column_id: i32,
    is_nullable: bool,
    is_identity: bool,
    is_computed: bool,
    user_type_id: i32,
    joined_sys_type: IntrospectionType,
    joined_foreign_key_columns: Vec<IntrospectionForeignKeyColumn>,
}

#[derive(Deserialize, Debug)]
pub struct IntrospectionType {
    name: String,
    schema_id: i32,
    user_type_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct IntrospectionPrimaryKey {
    name: String,
    index_id: i32,
    columns: Vec<IntrospectionPrimaryKeyColumn>,
}

#[derive(Deserialize, Debug)]
pub struct IntrospectionPrimaryKeyColumn {
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct IntrospectionForeignKeyColumn {
    constraint_object_id: i32,
    constraint_column_id: i32,
    parent_object_id: i32,
    parent_column_id: i32,
    referenced_object_id: i32,
    referenced_column_id: i32,
    joined_referenced_table_name: String,
    joined_referenced_column_name: String,
    joined_referenced_sys_schema: IntrospectionSchema,
}

#[derive(Deserialize, Debug)]
pub struct IntrospectionSchema {
    name: String,
    schema_id: i32,
}
