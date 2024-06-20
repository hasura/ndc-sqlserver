mod configuration;
pub mod environment;
mod error;
pub mod introspection;
pub mod secret;
pub mod uri;
pub mod version1;

pub use error::Error;
pub use uri::ConnectionUri;

pub use configuration::{
    CONFIGURATION_FILENAME, CONFIGURATION_JSONSCHEMA_FILENAME, DEFAULT_CONNECTION_URI_VARIABLE,
};

pub use version1::{
    configure, create_state, occurring_scalar_types, validate_raw_configuration, Configuration,
    InitializationError, RawConfiguration, State,
};
