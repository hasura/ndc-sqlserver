mod configuration;
pub mod version1;

pub use configuration::{CONFIGURATION_FILENAME, CONFIGURATION_JSONSCHEMA_FILENAME};

pub use version1::{
    configure, create_state, occurring_scalar_types, validate_raw_configuration, Configuration,
    InitializationError, RawConfiguration, State,
};
