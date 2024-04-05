use super::{error::Error, helpers::Env};
use ndc_sdk::models;
use query_engine_metadata::metadata;
use query_engine_sql::sql;

/// Maps a binary comparison operator to their appropriate SqlServer name and arguments type.
pub fn translate_comparison_operator(
    env: &Env,
    left_type: &metadata::ScalarType,
    operator: &String,
) -> Result<(sql::ast::BinaryOperator, metadata::ScalarType), Error> {
    let op = env.lookup_comparison_operator(&left_type, operator)?;
    if op.operator_kind == metadata::OperatorKind::In {
        Ok((sql::ast::BinaryOperator("IN".to_string()), left_type.clone()))
    }
    else if op.operator_kind == metadata::OperatorKind::Equal {
        Ok((sql::ast::BinaryOperator("=".to_string()), left_type.clone()))
    }
    else {
        Ok((
            sql::ast::BinaryOperator(op.operator_name.clone()),
            op.argument_type.clone()
        ))
    }
    // match operator {
    //     models::ComparisonOperatorDefinition::Equal => {
    //         Ok((sql::ast::BinaryOperator("=".to_string()), left_type.clone()))
    //     }
    //     models::ComparisonOperatorDefinition::In { .. } => {
    //         Ok((sql::ast::BinaryOperator("IN".to_string()), left_type.clone()))
    //     }
    //     models::ComparisonOperatorDefinition::Custom { argument_type : models::Type::Named { name } } => {
    //         let op = env.lookup_comparison_operator(left_type, name)?;

    //         Ok((
    //             sql::ast::BinaryOperator(op.operator_name.clone()),
    //             op.argument_type.clone(),
    //         ))
    //     },
    //     models::ComparisonOperatorDefinition::Custom { argument_type : models::Type::Predicate { object_type_name } } => {
    //         let op = env.lookup_comparison_operator(left_type, object_type_name)?;

    //         Ok((
    //             sql::ast::BinaryOperator(op.operator_name.clone()),
    //             op.argument_type.clone(),
    //         ))
    //     },
    //     models::ComparisonOperatorDefinition::Custom { argument_type : models::Type::Nullable { underlying_type } } => {
    //         // let op = env.lookup_comparison_operator(left_type, underlying_type)?;

    //         // Ok((
    //         //     sql::ast::BinaryOperator(op.operator_name.clone()),
    //         //     op.argument_type.clone(),
    //         // ))
    //         Err(Error::NotSupported("nullable".to_string())) // nullable comparison operators are not supported
    //     },
    //     models::ComparisonOperatorDefinition::Custom { argument_type : models::Type::Array { element_type } } => {
    //         // let op = env.lookup_comparison_operator(left_type, underlying_type)?;

    //         // Ok((
    //         //     sql::ast::BinaryOperator(op.operator_name.clone()),
    //         //     op.argument_type.clone(),
    //         // ))
    //         Err(Error::NotSupported("array".to_string()))
    //     }
    // }
}
