//! Convert a SQL AST to a low-level SQL string.

use super::ast::*;
use super::helpers;
use super::string::*;

// Convert to SQL strings

impl With {
    pub fn to_sql(&self, sql: &mut SQL) {
        if self.common_table_expressions.is_empty() {
        } else {
            sql.append_syntax("WITH ");

            let ctes = &self.common_table_expressions;
            for (index, cte) in ctes.iter().enumerate() {
                cte.to_sql(sql);
                if index < (ctes.len() - 1) {
                    sql.append_syntax(", ")
                }
            }
        }
    }
}

impl CommonTableExpression {
    pub fn to_sql(&self, sql: &mut SQL) {
        self.alias.to_sql(sql);
        match &self.column_names {
            None => {}
            Some(names) => {
                sql.append_syntax("(");
                for (index, name) in names.iter().enumerate() {
                    name.to_sql(sql);
                    if index < (names.len() - 1) {
                        sql.append_syntax(",")
                    }
                }
                sql.append_syntax(")");
            }
        }

        sql.append_syntax(" AS (");
        self.select.to_sql(sql);
        // the newline is important because a native query might end with a comment
        sql.append_syntax("\n)");
    }
}

impl WithJSONSchema {
    pub fn to_sql(&self, sql: &mut SQL) {
        sql.append_syntax("WITH (");
        for (index, (col_name, col_type)) in self.0.iter().enumerate() {
            col_name.to_sql(sql);
            sql.append_syntax(" ");
            col_type.to_sql(sql);
            if index < (self.0.len() - 1) {
                sql.append_syntax(", ")
            }
        }
        sql.append_syntax(")");
    }
}

impl CTExpr {
    pub fn to_sql(&self, sql: &mut SQL) {
        match self {
            CTExpr::RawSql(raw_vec) => {
                for item in raw_vec {
                    item.to_sql(sql);
                }
            }
            CTExpr::Select(select) => {
                select.to_sql(sql);
            }
        }
    }
}

impl RawSql {
    pub fn to_sql(&self, sql: &mut SQL) {
        match self {
            RawSql::RawText(text) => sql.append_syntax(text),
            RawSql::Expression(exp) => {
                exp.to_sql(sql);
            }
        }
    }
}

impl RawSQLStatement {
    pub fn to_sql(&self, sql: &mut SQL) {
        for item in self.0.iter() {
            item.to_sql(sql);
        }
    }
}

impl RawSQLQuery {
    pub fn to_sql(&self, sql: &mut SQL) {
        for item in self.0.iter() {
            item.to_sql(sql);
            sql.append_syntax(";");
        }
    }
}

impl TemporaryTableName {
    pub fn to_sql(&self, sql: &mut SQL) {
        sql.append_identifier(&self.0.format());
    }
}

impl TemporaryTable {
    pub fn to_sql(&self, sql: &mut SQL) {
        sql.append_syntax("CREATE TABLE ");
        self.name.to_sql(sql);
        sql.append_syntax(" (");

        for (index, (col_name, col_type)) in self.columns.iter().enumerate() {
            let col_type = match col_type.0.as_str() {
                "varchar" => "VARCHAR(MAX)",
                anything_else => anything_else,
            };
            sql.append_syntax(format!("{col_name} {}", col_type).as_str());
            if index < (self.columns.len() - 1) {
                sql.append_syntax(", ");
            }
        }
        sql.append_syntax(");");
    }

    pub fn drop_temp_table(&self, sql: &mut SQL) {
        sql.append_syntax("DROP TABLE ");
        self.name.to_sql(sql);
        sql.append_syntax(";");
    }
}

impl ExecProcedure {
    pub fn to_sql(&self, sql: &mut SQL) {
        sql.append_syntax("EXEC ");
        sql.append_identifier(&self.procedure_schema);
        sql.append_syntax(".");
        sql.append_identifier(&self.procedure_name);

        if !self.arguments.is_empty() {
            for (index, (arg_name, arg_value)) in self.arguments.iter().enumerate() {
                sql.append_syntax(format!("@{arg_name} ").as_str());
                sql.append_syntax(" = ");
                arg_value.to_sql(sql);
                if index < (self.arguments.len() - 1) {
                    sql.append_syntax(", ");
                }
            }
        }
    }
}

impl ExecProcedureInsertIntoTempTable {
    pub fn to_sql(&self, sql: &mut SQL) {
        self.temp_table.to_sql(sql);
        sql.append_syntax("\n");
        sql.append_syntax("INSERT INTO ");
        self.temp_table.name.to_sql(sql);
        sql.append_syntax("( ");
        for (index, (col_name, _)) in self.temp_table.columns.iter().enumerate() {
            sql.append_identifier(col_name);
            if index < (self.temp_table.columns.len() - 1) {
                sql.append_syntax(", ");
            }
        }
        sql.append_syntax(") ");
        self.exec_procedure.to_sql(sql);
        sql.append_syntax(";");
        self.response_selection.to_sql(sql);
    }
}

impl Explain<'_> {
    pub fn to_sql(&self, sql: &mut SQL) {
        sql.append_syntax("EXPLAIN ");
        match self {
            Explain::Select(select) => select.to_sql(sql),
        }
    }
}

impl SelectList {
    pub fn to_sql(&self, sql: &mut SQL) {
        match self {
            SelectList::SelectList(select_list) => {
                for (index, (col, expr)) in select_list.iter().enumerate() {
                    expr.to_sql(sql);
                    sql.append_syntax(" AS ");
                    col.to_sql(sql);
                    if index < (select_list.len() - 1) {
                        sql.append_syntax(", ")
                    }
                }
            }
            SelectList::SelectStar => {
                sql.append_syntax("*");
            }
        }
    }
}

impl Select {
    pub fn to_sql(&self, sql: &mut SQL) {
        self.with.to_sql(sql);

        sql.append_syntax("SELECT ");

        self.select_list.to_sql(sql);

        sql.append_syntax(" ");

        match &self.from {
            Some(from) => from.to_sql(sql),
            None => (),
        }

        for join in self.joins.iter() {
            join.to_sql(sql)
        }

        self.where_.to_sql(sql);

        self.order_by.to_sql(sql);

        match &self.limit {
            Some(limit) => limit.to_sql(sql),
            None => (),
        }

        self.for_json.to_sql(sql);
    }
}

impl ForJson {
    pub fn to_sql(&self, sql: &mut SQL) {
        match self {
            ForJson::NoJson => {}
            ForJson::ForJsonPath => sql.append_syntax(" FOR JSON PATH, INCLUDE_NULL_VALUES "),
            ForJson::ForJsonPathWithoutArrayWrapper => {
                sql.append_syntax(" FOR JSON PATH, INCLUDE_NULL_VALUES, WITHOUT_ARRAY_WRAPPER")
            }
        }
    }
}

impl From {
    pub fn to_sql(&self, sql: &mut SQL) {
        sql.append_syntax("FROM ");
        match &self {
            From::Table { reference, alias } => {
                reference.to_sql(sql);
                sql.append_syntax(" AS ");
                alias.to_sql(sql);
            }
            From::Select {
                select,
                alias,
                alias_path,
            } => {
                sql.append_syntax("(");
                select.to_sql(sql);
                sql.append_syntax(")");
                sql.append_syntax(" AS ");
                alias.to_sql(sql);
                alias_path.to_sql(sql);
            }
            From::OpenJSON {
                alias,
                with_json_schema,
                json_value_param,
            } => {
                sql.append_syntax("OPENJSON");
                sql.append_syntax("(");
                sql.append_param(json_value_param.clone());
                sql.append_syntax(")");
                sql.append_syntax(" ");
                with_json_schema.to_sql(sql);
                alias.to_sql(sql);
            }
        }
    }
}

impl AliasPath {
    pub fn to_sql(&self, sql: &mut SQL) {
        if !self.elements.is_empty() {
            sql.append_syntax("(");
            for (i, path_item) in self.elements.iter().enumerate() {
                path_item.to_sql(sql);
                if i < self.elements.len() - 1 {
                    sql.append_syntax(",");
                }
            }
            sql.append_syntax(")");
        }
    }
}

impl Join {
    pub fn to_sql(&self, sql: &mut SQL) {
        match self {
            Join::InnerJoin(InnerJoin { select, alias, on }) => {
                sql.append_syntax(" INNER JOIN ");
                sql.append_syntax("(");
                select.to_sql(sql);
                sql.append_syntax(")");
                sql.append_syntax(" AS ");
                alias.to_sql(sql);
                sql.append_syntax(" ON (");
                on.to_sql(sql);
                sql.append_syntax(")");
            }
            Join::CrossJoin(CrossJoin {
                select,
                alias,
                alias_path,
            }) => {
                sql.append_syntax(" CROSS JOIN ");
                sql.append_syntax("(");
                select.to_sql(sql);
                sql.append_syntax(")");
                sql.append_syntax(" AS ");
                alias.to_sql(sql);
                alias_path.to_sql(sql)
            }
            Join::OuterApply(OuterApply {
                select,
                alias,
                alias_path,
            }) => {
                sql.append_syntax(" OUTER APPLY ");
                sql.append_syntax("(");
                select.to_sql(sql);
                sql.append_syntax(")");
                sql.append_syntax(" AS ");
                alias.to_sql(sql);
                alias_path.to_sql(sql)
            }
        }
    }
}

impl Where {
    pub fn to_sql(&self, sql: &mut SQL) {
        let Where(expression) = self;
        if *expression != helpers::true_expr() {
            sql.append_syntax(" WHERE ");
            expression.to_sql(sql);
        }
    }
}

// scalars
impl Expression {
    pub fn to_sql(&self, sql: &mut SQL) {
        match &self {
            Expression::ColumnReference(column_reference) => column_reference.to_sql(sql),
            Expression::Value(value) => {
                value.to_sql(sql);
            }
            Expression::Cast { expression, r#type } => {
                sql.append_syntax("cast");
                sql.append_syntax("(");
                expression.to_sql(sql);
                sql.append_syntax(" as ");
                r#type.to_sql(sql);
                sql.append_syntax(")");
            }
            Expression::And { left, right } => {
                sql.append_syntax("(");
                left.to_sql(sql);
                sql.append_syntax(" AND ");
                right.to_sql(sql);
                sql.append_syntax(")");
            }
            Expression::Or { left, right } => {
                sql.append_syntax("(");
                left.to_sql(sql);
                sql.append_syntax(" OR ");
                right.to_sql(sql);
                sql.append_syntax(")");
            }
            Expression::Not(expr) => {
                sql.append_syntax("NOT ");
                expr.to_sql(sql);
            }
            Expression::BinaryOperation {
                left,
                operator,
                right,
            } => {
                sql.append_syntax("(");
                left.to_sql(sql);
                operator.to_sql(sql);
                right.to_sql(sql);
                sql.append_syntax(")");
            }
            Expression::BinaryArrayOperation {
                left,
                operator,
                right,
            } => {
                sql.append_syntax("(");
                {
                    left.to_sql(sql);
                    operator.to_sql(sql);
                    sql.append_syntax("(");
                    for (index, item) in right.iter().enumerate() {
                        item.to_sql(sql);
                        if index < (right.len() - 1) {
                            sql.append_syntax(", ")
                        }
                    }
                    sql.append_syntax(")");
                }
                sql.append_syntax(")");
            }
            Expression::UnaryOperation {
                expression,
                operator,
            } => {
                sql.append_syntax("(");
                expression.to_sql(sql);
                operator.to_sql(sql);
                sql.append_syntax(")");
            }
            Expression::FunctionCall { function, args } => {
                function.to_sql(sql);
                sql.append_syntax("(");
                for (index, arg) in args.iter().enumerate() {
                    arg.to_sql(sql);
                    if index < (args.len() - 1) {
                        sql.append_syntax(", ")
                    }
                }
                sql.append_syntax(")");
            }
            Expression::Exists { select } => {
                sql.append_syntax("EXISTS ");
                sql.append_syntax("(");
                select.to_sql(sql);
                sql.append_syntax(")");
            }
            Expression::Count(count_type) => {
                sql.append_syntax("COUNT");
                sql.append_syntax("(");
                count_type.to_sql(sql);
                sql.append_syntax(")")
            }
            Expression::JsonQuery(target, path) => {
                sql.append_syntax("JSON_QUERY");
                sql.append_syntax("(");
                target.to_sql(sql);
                sql.append_syntax(", ");
                path.to_sql(sql);
                sql.append_syntax(")")
            }
            Expression::JsonValue(target, path) => {
                sql.append_syntax("JSON_VALUE");
                sql.append_syntax("(");
                target.to_sql(sql);
                sql.append_syntax(", ");
                path.to_sql(sql);
                sql.append_syntax(")")
            }
        }
    }
}

impl JsonPath {
    pub fn to_sql(&self, sql: &mut SQL) {
        sql.append_syntax("'$");
        for ColumnAlias {
            name: path_item, ..
        } in self.elements.iter()
        {
            sql.append_syntax(".");
            sql.append_syntax(path_item);
        }
        sql.append_syntax("'");
    }
}

impl UnaryOperator {
    pub fn to_sql(&self, sql: &mut SQL) {
        match self {
            UnaryOperator::IsNull => sql.append_syntax(" IS NULL "),
        }
    }
}

impl BinaryOperator {
    pub fn to_sql(&self, sql: &mut SQL) {
        sql.append_syntax(" ");
        sql.append_syntax(self.0.as_str());
        sql.append_syntax(" ");
    }
}

impl BinaryArrayOperator {
    pub fn to_sql(&self, sql: &mut SQL) {
        match self {
            BinaryArrayOperator::In => sql.append_syntax(" IN "),
        }
    }
}

impl Function {
    pub fn to_sql(&self, sql: &mut SQL) {
        match self {
            Function::Coalesce => sql.append_syntax("COALESCE"),
            Function::IsNull => sql.append_syntax("ISNULL"),
            Function::JsonAgg => sql.append_syntax("JSON_AGG"),
            Function::Unknown(name) => sql.append_syntax(name),
        }
    }
}

impl CountType {
    pub fn to_sql(&self, sql: &mut SQL) {
        match self {
            CountType::Star => sql.append_syntax("*"),
            CountType::Simple(column) => column.to_sql(sql),
            CountType::Distinct(column) => {
                sql.append_syntax("DISTINCT ");
                column.to_sql(sql)
            }
        }
    }
}

impl Value {
    pub fn to_sql(&self, sql: &mut SQL) {
        match &self {
            Value::EmptyJsonArray => sql.append_syntax("'[]'"),
            Value::Int8(i) => sql.append_syntax(format!("{}", i).as_str()),
            Value::Float8(n) => sql.append_syntax(format!("{}", n).as_str()),
            Value::Character(s) => sql.append_param(Param::String(s.clone())),
            Value::String(s) => sql.append_param(Param::String(s.clone())),
            Value::Variable(v) => sql.append_param(Param::Variable(v.clone())),
            Value::Bool(true) => sql.append_syntax("1 = 1"),
            Value::Bool(false) => sql.append_syntax("1 = 0"),
            Value::Null => sql.append_syntax("null"),
            Value::Array(items) => {
                sql.append_syntax("ARRAY [");
                for (index, item) in items.iter().enumerate() {
                    item.to_sql(sql);
                    if index < (items.len() - 1) {
                        sql.append_syntax(", ")
                    }
                }
                sql.append_syntax("]");
            }
        }
    }
}

impl ScalarType {
    pub fn to_sql(&self, sql: &mut SQL) {
        sql.append_syntax(self.0.as_str())
    }
}

impl Limit {
    pub fn to_sql(&self, sql: &mut SQL) {
        sql.append_syntax(" OFFSET ");
        sql.append_syntax(format!("{}", self.offset).as_str());
        sql.append_syntax(" ROWS ");
        match self.limit {
            None => (),
            Some(limit) => {
                sql.append_syntax(" FETCH NEXT ");
                sql.append_syntax(format!("{}", limit).as_str());
                sql.append_syntax(" ROWS ONLY");
            }
        };
    }
}

// names
impl TableReference {
    pub fn to_sql(&self, sql: &mut SQL) {
        match self {
            TableReference::DBTable { schema, table } => {
                sql.append_identifier(&schema.0);
                sql.append_syntax(".");
                sql.append_identifier(&table.0);
            }
            TableReference::AliasedTable(alias) => alias.to_sql(sql),
        };
    }
}

impl TableAlias {
    fn format(&self) -> String {
        if self.is_temporary_table {
            format!("#{}_{}", self.unique_index, self.name)
        } else {
            format!("{}_{}", self.unique_index, self.name)
        }
    }

    pub fn to_sql(&self, sql: &mut SQL) {
        let name = self.format();
        sql.append_identifier(&name);
    }
}

impl ColumnReference {
    pub fn to_sql(&self, sql: &mut SQL) {
        match self {
            ColumnReference::TableColumn { table, name } => {
                table.to_sql(sql);
                sql.append_syntax(".");
                sql.append_identifier(&name.0.to_string());
            }
            ColumnReference::AliasedColumn { table, column } => {
                table.to_sql(sql);
                sql.append_syntax(".");
                column.to_sql(sql);
            }
        };
    }
}

impl ColumnAlias {
    pub fn to_sql(&self, sql: &mut SQL) {
        let name = self.name.to_string();
        sql.append_identifier(&name);
    }
}

impl OrderBy {
    pub fn to_sql(&self, sql: &mut SQL) {
        if !self.elements.is_empty() {
            sql.append_syntax(" ORDER BY ");
            for (index, order_by_item) in self.elements.iter().enumerate() {
                order_by_item.to_sql(sql);
                if index < (self.elements.len() - 1) {
                    sql.append_syntax(", ")
                }
            }
        }
    }
}

impl OrderByElement {
    pub fn to_sql(&self, sql: &mut SQL) {
        self.target.to_sql(sql);
        self.direction.to_sql(sql)
    }
}

impl OrderByDirection {
    pub fn to_sql(&self, sql: &mut SQL) {
        match self {
            OrderByDirection::Asc => sql.append_syntax(" ASC "),
            OrderByDirection::Desc => sql.append_syntax(" DESC "),
        }
    }
}
