//! Describe the SQL execution plan.

use indexmap::IndexMap;

use crate::sql;

use std::collections::BTreeMap;

use ndc_sdk::models;

use super::ast::{Select, TableAlias, WithJSONSchema};

#[derive(Debug)]
pub struct NativeMutationResponseSelection {
    /// JSON schema of the response obtained after
    /// running the mutation SQL query.
    pub response_json_schema: WithJSONSchema,
    /// Select AST that will be run on the
    /// response of the native mutation and then
    /// ultimately form the response of the NDC
    /// request.
    /// Note that, the mutation response needs to
    /// be added in a CTE, after we run the mutation
    /// SQL and get the response back.
    pub response_select: Select,
    /// Alias of the CTE where the response is expected to be.
    pub response_cte_table_alias: TableAlias,
}

#[derive(Debug)]
pub struct NativeMutationExecutionPlan {
    /// First query that will run when executed.
    ///
    /// In case of native mutations, this will be the SQL statement
    /// where the user provided native mutation SQL query will be run
    /// after substituting the value of the arguments.
    /// The response obtained from this query will become the source
    /// for the `response_selection_cte`.
    pub mutation_sql_query: sql::string::SQL,
    /// Select query that will run on the response
    /// of the `mutation_sql_query`
    pub response_selection: NativeMutationResponseSelection,
    /// Name of the operation, this should be the name of the procedure
    pub native_mutation_name: String,
}

#[derive(Debug)]
pub enum MutationExecutionPlan {
    NativeMutation(NativeMutationExecutionPlan),
}

#[derive(Debug)]
/// Definition of a mutation execution plan to be run against the database.
pub struct MutationsExecutionPlan {
    /// First query that will run when executed.
    ///
    /// In case of native mutations, this will be the SQL statement
    /// where the user provided native mutation SQL query will be run
    /// after substituting the value of the arguments.
    /// The response obtained from this query will become the source
    /// for the `response_selection`.
    //    pub pre: sql::string::SQL,
    /// The mutations that need to be run.
    pub mutations: Vec<MutationExecutionPlan>,
    // TODO(KC): Add a `post` step here
}

#[derive(Debug)]
/// Definition of a query execution plan to be run against the database.
pub struct QueryExecutionPlan {
    pub variables: Option<Vec<BTreeMap<String, serde_json::Value>>>,
    pub root_field: String,
    /// The query.
    pub query: sql::ast::Select,
    /// Run after the query. Should be a sql::ast in the future.
    pub post: Vec<sql::string::DDL>,
}

impl QueryExecutionPlan {
    /// Extract the query component as SQL.
    pub fn query(&self) -> sql::string::SQL {
        select_to_sql(&self.query)
    }
}

pub fn select_to_sql(select: &sql::ast::Select) -> sql::string::SQL {
    let mut sql = sql::string::SQL::new();
    select.to_sql(&mut sql);
    sql
}

pub fn explain_to_sql(explain: &sql::ast::Explain) -> sql::string::SQL {
    let mut sql = sql::string::SQL::new();
    explain.to_sql(&mut sql);
    sql
}

/// A simple execution plan with only a root field and a query.
pub fn simple_exec_plan(
    variables: Option<Vec<BTreeMap<String, serde_json::Value>>>,
    root_field: String,
    query: sql::ast::Select,
) -> QueryExecutionPlan {
    QueryExecutionPlan {
        variables,
        root_field,
        query,
        post: vec![],
    }
}
