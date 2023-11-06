//! Type definitions of a SQL AST representation.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Explain<'a> {
    Select(&'a Select),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct With {
    pub recursive: bool,
    pub common_table_expressions: Vec<CommonTableExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommonTableExpression {
    pub table_name: TableAlias,
    pub column_names: Option<Vec<ColumnAlias>>,
    pub select: Box<Select>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
pub enum ForJson {
    NoJson,
    ForJsonPath,
    ForJsonPathWithoutArrayWrapper,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectList {
    SelectList(Vec<(ColumnAlias, Expression)>),
    SelectStar,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum From {
    Table {
        name: TableName,
        alias: TableAlias,
    },
    Select {
        select: Box<Select>,
        alias: TableAlias,
        alias_path: AliasPath,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AliasPath {
    pub elements: Vec<ColumnAlias>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Join {
    OuterApply(OuterApply),
    CrossJoin(CrossJoin),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrossJoin {
    pub select: Box<Select>,
    pub alias: TableAlias,
    pub alias_path: AliasPath,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OuterApply {
    pub select: Box<Select>,
    pub alias: TableAlias,
    pub alias_path: AliasPath,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Where(pub Expression);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupBy {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderBy {
    pub elements: Vec<OrderByElement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderByDirection {
    Asc,
    Desc,
}

// todo: should we also include option for specifying NULLS FIRST | NULLS LAST
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderByElement {
    pub target: Expression,
    pub direction: OrderByDirection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Limit {
    pub limit: Option<u32>,
    pub offset: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    And {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Or {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Not(Box<Expression>),
    BinaryOperator {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    BinaryArrayOperator {
        left: Box<Expression>,
        operator: BinaryArrayOperator,
        right: Vec<Expression>,
    },
    UnaryOperator {
        column: Box<Expression>,
        operator: UnaryOperator,
    },
    FunctionCall {
        function: Function,
        args: Vec<Expression>,
    },
    Exists {
        select: Box<Select>,
    },
    Table(TableName),
    ColumnName(ColumnName),
    Value(Value),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Json,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperator {
    IsNull,
}

// we should consider at least the list in `Hasura.Backends.SQLServer.Translate.BoolExp`
// have skipped column checks for now, ie, CEQ, CNE, CGT etc
// have skipped casts for now
// we'd like to remove all the Not variants internally, but first we'll check there are no
// performance implications
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOperator {
    Equals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
    Like,
    NotLike,
    CaseInsensitiveLike,
    NotCaseInsensitiveLike,
    Similar,
    NotSimilar,
    Regex,
    NotRegex,
    CaseInsensitiveRegex,
    NotCaseInsensitiveRegex,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryArrayOperator {
    In,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Function {
    IsNull,
    JsonAgg,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CountType {
    Star,
    Simple(ColumnName),
    Distinct(ColumnName),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Int4(i32),
    Bool(bool),
    String(String),
    Array(Vec<Value>),
    EmptyJsonArray,
    Variable(String),
}

/// aliases that we give to relations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableAlias {
    pub unique_index: u64,
    pub name: String,
}
/// aliases that we give to columns
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColumnAlias {
    pub unique_index: u64,
    pub name: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TableName {
    /// refers to a db table object name
    DBTable { schema: String, table: String },
    /// refers to an alias we created
    AliasedTable(TableAlias),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ColumnName {
    /// refers to a db column object name
    TableColumn { table: TableName, name: String },
    /// refers to an alias we created
    AliasedColumn { table: TableName, name: ColumnAlias },
}