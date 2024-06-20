use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::secret;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub struct ConnectionUri(pub secret::Secret);

impl From<String> for ConnectionUri {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl From<&str> for ConnectionUri {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
