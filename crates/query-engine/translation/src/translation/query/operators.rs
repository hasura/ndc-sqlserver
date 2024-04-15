use super::{error::Error, helpers::Env};
use query_engine_metadata::metadata;
use query_engine_sql::sql;

/// Maps a binary comparison operator to their appropriate SqlServer name and arguments type.
// TODO(PY): Remove this function
pub fn _translate_comparison_operator(
    env: &Env,
    left_type: &metadata::ScalarType,
    operator: &String,
) -> Result<(sql::ast::BinaryOperator, metadata::ScalarType), Error> {
    let op = env.lookup_comparison_operator(left_type, operator)?;
    if op.operator_kind == metadata::OperatorKind::In {
        Ok((
            sql::ast::BinaryOperator("IN".to_string()),
            left_type.clone(),
        ))
    } else if op.operator_kind == metadata::OperatorKind::Equal {
        Ok((sql::ast::BinaryOperator("=".to_string()), left_type.clone()))
    } else {
        Ok((
            sql::ast::BinaryOperator(op.operator_name.clone()),
            op.argument_type.clone(),
        ))
    }
}
