//! Describe the SQL execution plan.

use crate::sql;

use std::collections::BTreeMap;

#[derive(Debug)]
/// Definition of a mutation execution plan to be run against the database.
pub struct MutationExecutionPlan {
    /// The mutation query to be run.
    pub mutation_query: sql::string::SQL,
    // /// Select query that will run on the response
    // /// of the `mutation_query` and will ultimately
    // /// return the response of the NDC request.
    // pub response_selection: sql::ast::Select,
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
