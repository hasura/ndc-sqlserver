mod configuration;
mod error;
pub mod introspection;
pub mod version1;

pub use error::Error;

pub use configuration::{CONFIGURATION_FILENAME, CONFIGURATION_JSONSCHEMA_FILENAME};

pub use version1::{
    configure, create_state, occurring_scalar_types, validate_raw_configuration, Configuration,
    InitializationError, RawConfiguration, State,
};