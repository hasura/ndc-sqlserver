use thiserror::Error;

#[derive(Debug, Error)]
pub enum NativeMutationResponseParseError {
    #[error("Unable to parse the float number, because it is not a valid JSON float number.")]
    InvalidJSONFloatNumber,
    #[error("Unable to parse response of type {0:#?}. HINT: Try casting the output of the column as as string in the native mutation SQL query.")]
    UnknownType(tiberius::ColumnType),
    #[error("Unable to parse response: {0}. HINT: Try casting the output of the column as a string in the native mutation SQL query.")]
    UnableToParseResponse(tiberius::error::Error),
}

#[derive(Debug, Error)]
pub enum MutationError {
    #[error("Error executing native mutation, column name: {column_name}, column type: {column_type:#?}, error: {error}")]
    NativeMutation {
        column_name: String,
        column_type: tiberius::ColumnType,
        error: NativeMutationResponseParseError,
    },
    #[error("The native mutation {native_mutation_name} is returning more than one set of rows. A native mutation statement is expected to return exactly one set of row set")]
    NativeMutationMoreThanOneRowSet { native_mutation_name: String },
    #[error("Error in serializing the native mutation response to JSON. Error: {0}")]
    JSONSerializationError(serde_json::Error),
    #[error("Error in translating the response selection query: {0}")]
    NativeMutationResponseSelectionError(query_engine_translation::translation::error::Error),
}

#[derive(Debug)]
pub enum Error {
    Query(String),
    ConnectionPool(bb8::RunError<bb8_tiberius::Error>),
    TiberiusError(tiberius::error::Error),
    Mutation(MutationError),
}
