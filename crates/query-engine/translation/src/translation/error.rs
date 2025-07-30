//! Errors for query translation.

use query_engine_metadata::metadata::database;

/// A type for translation errors.
#[derive(Debug)]
pub enum Error {
    CollectionNotFound(String),
    ColumnNotFoundInCollection(String, String),
    ColumnNotFoundInProcedure(String, String),
    RelationshipNotFound(String),
    ArgumentNotFound(String),
    OperatorNotFound {
        operator_name: String,
        type_name: database::ScalarType,
    },
    RelationshipArgumentWasOverriden(String),
    EmptyPathForStarCountAggregate,
    TypeMismatch(serde_json::Value, database::ScalarType),
    CapabilityNotSupported(UnsupportedCapabilities),
    NoConstraintsForOrdering(String),
    NoColumnsForOrdering,
    NotSupported(String),
    NoFieldsAndAggregates,
    ProcedureNotFound(String),
    SerdeSerializationError(serde_json::Error),
    UnexpectedStructure(String),
    NoProcedureResultFieldsRequested,
    NotImplementedYet(String),
    UnexpectedInternalError(String),
}

/// Capabilities we don't currently support.
#[derive(Debug)]
pub enum UnsupportedCapabilities {}

impl std::fmt::Display for UnsupportedCapabilities {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        todo!()
    }
}

/// Display errors.
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::CollectionNotFound(collection_name) => {
                write!(f, "Collection '{collection_name}' not found.")
            }
            Error::ColumnNotFoundInCollection(column_name, collection_name) => write!(
                f,
                "Column '{column_name}' not found in collection '{collection_name}'."
            ),
            Error::ColumnNotFoundInProcedure(column_name, procedure_name) => write!(
                f,
                "Column '{column_name}' not found in procedure '{procedure_name}'."
            ),

            Error::NoConstraintsForOrdering(table_name) => {
                write!(f, "No constraints found for ordering. An order by clause or a primary key on the table '{table_name}' is required for queries with a limit or offset clause.")
            }
            Error::NoColumnsForOrdering => {
                write!(f, "No columns found for ordering")
            }

            Error::RelationshipNotFound(relationship_name) => {
                write!(f, "Relationship '{relationship_name}' not found.")
            }
            Error::ArgumentNotFound(argument) => {
                write!(f, "Argument '{argument}' not found.")
            }
            Error::OperatorNotFound {
                operator_name,
                type_name,
            } => {
                write!(
                    f,
                    "Operator '{operator_name}' not found in type {type_name:?}."
                )
            }
            Error::RelationshipArgumentWasOverriden(key) => {
                write!(f, "The relationship argument '{key}' was defined as part of the relationship, but was overriden.")
            }
            Error::EmptyPathForStarCountAggregate => {
                write!(f, "No path elements supplied for Star Count Aggregate")
            }
            Error::TypeMismatch(value, typ) => {
                write!(f, "Value '{value}' is not of type '{typ:?}'.")
            }
            Error::CapabilityNotSupported(thing) => {
                write!(f, "Queries containing {thing} are not supported.")
            }
            Error::NotSupported(thing) => {
                write!(f, "Queries containing {thing} are not supported.")
            }
            Error::NoFieldsAndAggregates => {
                write!(f, "No fields or aggregates found in query")
            }
            Error::ProcedureNotFound(name) => {
                write!(f, "Procedure '{name}' not found.")
            }
            Error::SerdeSerializationError(serde_err) => {
                write!(f, "JSON serialization error: {serde_err}")
            }
            Error::UnexpectedStructure(s) => {
                write!(f, "Unexpected structure received: {s}")
            }
            Error::NotImplementedYet(e) => {
                write!(f, "{e} is not implemented yet")
            }
            Error::NoProcedureResultFieldsRequested => {
                write!(f, "No procedure fields were requested.")
            }
            Error::UnexpectedInternalError(s) => {
                write!(f, "Unexepcted internal error: {s}")
            }
        }
    }
}
