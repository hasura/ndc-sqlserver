//! Functions to convert between internal error types and the error types exposed by ndc-sdk.

use ndc_sdk::connector::{self, ErrorResponse};

/// Convert an error from [query_engine_execution] to [ErrorResponse].
pub fn execution_error_to_response(error: query_engine_execution::error::Error) -> ErrorResponse {
    use query_engine_execution::error::*;
    match error {
        Error::Query(query_error) => {
            connector::QueryError::new_invalid_request(&query_error).into()
        }
        Error::Mutation(mutation_error) => {
            connector::MutationError::new_invalid_request(&mutation_error.to_string()).into()
        }
        Error::ConnectionPool(connection_pool_error) => {
            connector::QueryError::new_unprocessable_content(&connection_pool_error.to_string())
                .into()
        }
        Error::TiberiusError(tiberius_error) => {
            connector::QueryError::new_unprocessable_content(&tiberius_error.to_string()).into()
        }
    }
}

/// Convert an error from [query_engine_translation] to [connector::QueryError].
pub fn translation_error_to_response(
    error: &query_engine_translation::translation::error::Error,
) -> ErrorResponse {
    use query_engine_translation::translation::error::*;
    match error {
        Error::CapabilityNotSupported(_) | Error::NotImplementedYet(_) => {
            connector::QueryError::new_unsupported_operation(&error.to_string()).into()
        }
        _ => connector::QueryError::new_invalid_request(&error.to_string()).into(),
    }
}
