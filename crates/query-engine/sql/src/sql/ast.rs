//! Type definitions of a SQL AST representation.

use std::collections::BTreeMap;

use super::string::Param;

/// An EXPLAIN clause
#[derive(Debug, Clone, PartialEq)]
pub enum Explain<'a> {
    Select(&'a Select),
}

/// A WITH clause
#[derive(Debug, Clone, PartialEq)]
pub struct With {
    pub common_table_expressions: Vec<CommonTableExpression>,
}

/// Execution of a stored procedure
#[derive(Debug, Clone, PartialEq)]
pub struct ExecProcedure {
    /// Arguments to the procedure
    pub arguments: BTreeMap<String, Expression>,
    /// Name of the stored procedure
    pub procedure_name: String,
    /// Schema of the stored procedure
    pub procedure_schema: String,
}

/// Type of a DB column.
#[derive(Debug, Clone, PartialEq)]
pub struct ColumnType(pub String);

/// Name of a temporary table.
#[derive(Debug, Clone, PartialEq)]
pub struct TemporaryTableName(pub TableAlias);

/// Given a name and a set of columns, create a temporary
/// table.
#[derive(Debug, Clone, PartialEq)]
pub struct TemporaryTable {
    /// Name of the temporary table.
    pub name: TemporaryTableName,
    /// Columns in the temporary table.
    pub columns: BTreeMap<String, ColumnType>,
}

/// Execute a stored procedure and insert the response
/// into a temp table
#[derive(Debug, Clone, PartialEq)]
pub struct ExecProcedureInsertIntoTempTable {
    /// The target temp table where the stored procedure's
    /// results needs to be stored in.
    pub temp_table: TemporaryTable,
    /// Info about the stored procedure.
    pub exec_procedure: ExecProcedure,
    /// Response to be selected from the temporary table.
    pub response_selection: Select,
}

/// A single Common Table Expression
#[derive(Debug, Clone, PartialEq)]
pub struct CommonTableExpression {
    pub alias: TableAlias,
    pub column_names: Option<Vec<ColumnAlias>>,
    pub select: CTExpr,
}

/// The 'body' side of a Common Table Expression
#[derive(Debug, Clone, PartialEq)]
pub enum CTExpr {
    RawSql(Vec<RawSql>),
    Select(Select),
}

/// A collection of `RawSQLStatement` that will
/// be executed together. The `RawSQLStatement`s will
/// be separated by semicolons.
pub struct RawSQLQuery(pub Vec<RawSQLStatement>);

/// A single SQL statement.
#[derive(Debug, Clone, PartialEq)]
pub struct RawSQLStatement(pub Vec<RawSql>);

/// Raw SQL written by a user which is opaque to us
#[derive(Debug, Clone, PartialEq)]
pub enum RawSql {
    /// Raw SQL text
    RawText(String),
    /// An expression
    Expression(Expression),
}

/// A SELECT clause
#[derive(Debug, Clone, PartialEq)]
pub struct Select {
    pub with: With,
    pub select_list: SelectList,
    pub from: Option<From>,
    pub joins: Vec<Join>,
    pub where_: Where,
    pub group_by: GroupBy,
    pub order_by: OrderBy,
    pub limit: Option<Limit>,
    pub for_json: ForJson,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AliasPath {
    pub elements: Vec<ColumnAlias>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForJson {
    NoJson,
    ForJsonPath,
    ForJsonPathWithoutArrayWrapper,
}

/// A select list
#[derive(Debug, Clone, PartialEq)]
pub enum SelectList {
    SelectList(Vec<(ColumnAlias, Expression)>),
    SelectStar,
}

/// Schema of the JSON value which will help
/// us query the JSON value like a relational
/// table.
/// The first `String` value in the tuple is the name of the
/// column and the second `String` value in the tuple is the
/// type of the column that the value needs to be coerced into.
#[derive(Debug, Clone, PartialEq)]
pub struct WithJSONSchema(pub Vec<(ColumnAlias, ScalarType)>);

/// A FROM clause
#[derive(Debug, Clone, PartialEq)]
pub enum From {
    /// Select from a table reference
    Table {
        reference: TableReference,
        alias: TableAlias,
    },
    /// Select from a subquery
    Select {
        select: Box<Select>,
        alias: TableAlias,
        alias_path: AliasPath,
    },
    /// Query a JSON value, as if it were
    /// a relational table. The `with_json_schema`
    /// specifies the schema of the JSON value provided.
    OpenJSON {
        /// Name of the alias of the OpenJSON expression.
        alias: TableAlias,
        /// Parameter of the JSON payload.
        json_value_param: Param,
        /// Schema of the JSON value which will help
        /// us query the JSON value like a relational
        /// table.
        with_json_schema: WithJSONSchema,
    },
}

/// A JOIN clause
#[derive(Debug, Clone, PartialEq)]
pub enum Join {
    OuterApply(OuterApply),
    /// INNER JOIN
    InnerJoin(InnerJoin),
    /// CROSS JOIN
    CrossJoin(CrossJoin),
}

/// A CROSS JOIN clause
#[derive(Debug, Clone, PartialEq)]
pub struct CrossJoin {
    pub select: Box<Select>,
    pub alias: TableAlias,
    pub alias_path: AliasPath,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OuterApply {
    pub select: Box<Select>,
    pub alias: TableAlias,
    pub alias_path: AliasPath,
}

/// An INNER JOIN clause
#[derive(Debug, Clone, PartialEq)]
pub struct InnerJoin {
    pub select: Box<Select>,
    pub alias: TableAlias,
    pub on: Expression,
}

/// A WHERE clause
#[derive(Debug, Clone, PartialEq)]
pub struct Where(pub Expression);

/// A GROUP BY clause, currently not in use
#[derive(Debug, Clone, PartialEq)]
pub struct GroupBy {}

/// An ORDER BY clause
#[derive(Debug, Clone, PartialEq)]
pub struct OrderBy {
    pub elements: Vec<OrderByElement>,
}

// todo: should we also include option for specifying NULLS FIRST | NULLS LAST
/// A single element in an ORDER BY clause
#[derive(Debug, Clone, PartialEq)]
pub struct OrderByElement {
    pub target: Expression,
    pub direction: OrderByDirection,
}

/// A direction for a single ORDER BY element
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderByDirection {
    Asc,
    Desc,
}

/// LIMIT and OFFSET clauses
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Limit {
    pub limit: Option<u32>,
    pub offset: u32,
}

/// A scalar expression
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// AND clause
    And {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    /// OR clause
    Or {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    /// NOT clause
    Not(Box<Expression>),
    /// A binary operation on two scalar expression
    BinaryOperation {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    /// A binary operation on a scalar expression and an array of scalar expressions
    BinaryArrayOperation {
        left: Box<Expression>,
        operator: BinaryArrayOperator,
        right: Vec<Expression>,
    },
    /// An unary operation on a scalar expression
    UnaryOperation {
        expression: Box<Expression>,
        operator: UnaryOperator,
    },
    /// A scalar function call
    FunctionCall {
        function: Function,
        args: Vec<Expression>,
    },
    /// An EXISTS clause
    Exists {
        select: Box<Select>,
    },
    /// A column reference
    ColumnReference(ColumnReference),
    /// An irreducible value
    Value(Value),
    Cast {
        expression: Box<Expression>,
        r#type: ScalarType,
    },
    /// A COUNT clause
    Count(CountType),
    JsonQuery(Box<Expression>, JsonPath), // JSON_QUERY([album].[json], '$.title') for multiple
    // values
    JsonValue(Box<Expression>, JsonPath), // JSON_VALUE([album].[json], '$.title') for single values
}

// JSON selector path for expressing '$.user.name'
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonPath {
    pub elements: Vec<ColumnAlias>,
}

/// An unary operator
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperator {
    IsNull,
}

/// Represents the name of a binary operator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryOperator(pub String);

/// A binary operator when the rhs is an array
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryArrayOperator {
    In,
}

/// A scalar function
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Function {
    Coalesce,
    IsNull,
    JsonAgg,
    Unknown(String),
}

/// COUNT clause
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CountType {
    Star,
    Simple(ColumnReference),
    Distinct(ColumnReference),
}

/// Value
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int8(i32),
    Float8(f64),
    Bool(bool),
    Character(String),
    String(String),
    Null,
    Array(Vec<Value>),
    EmptyJsonArray,
    Variable(String),
}

/// Scalar type
#[derive(Debug, Clone, PartialEq)]
pub struct ScalarType(pub String);

/// A database schema name
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SchemaName(pub String);

/// A database table name
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableName(pub String);

/// A reference to a table. Used when we want to query it,
/// for example in a FROM clause.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TableReference {
    /// refers to a db table object name
    DBTable {
        schema: SchemaName,
        table: TableName,
    },
    /// refers to an alias we created
    AliasedTable(TableAlias),
}

/// A database table's column name
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColumnName(pub String);

/// A reference to a column. Used when we want to query it,
/// for example in a SELECT list.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ColumnReference {
    /// refers to a db column object name
    TableColumn {
        table: TableReference,
        name: ColumnName,
    },
    /// refers to an alias we created
    AliasedColumn {
        table: TableReference,
        column: ColumnAlias,
    },
}

/// aliases that we give to relations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableAlias {
    pub unique_index: u64,
    pub name: String,
    pub is_temporary_table: bool,
}

/// aliases that we give to columns
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColumnAlias {
    pub name: String,
}
