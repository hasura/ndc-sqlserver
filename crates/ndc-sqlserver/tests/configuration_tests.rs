use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use configuration::environment::Variable;

use ndc_sqlserver_configuration as configuration;
use ndc_sqlserver_configuration::secret;
use query_engine_metadata::metadata::{NativeMutations, NativeQueries};
use serde::de::DeserializeOwned;
use serde_json::from_value;
use similar_asserts::assert_eq;

pub mod common;
use common::configuration::*;

const CONNECTION_STRING: &str =
    "Server=localhost,64003;Uid=SA;Database=Chinook;Pwd=Password!;TrustServerCertificate=true";

const CHINOOK_DEPLOYMENT_PATH: &str = "static/configuration.json";

#[tokio::test]
async fn test_configure_is_idempotent() {
    configure_is_idempotent(CONNECTION_STRING, CHINOOK_DEPLOYMENT_PATH).await
}

// Tests that configuration generation has not changed.
//
// This test does not use insta snapshots because it checks the deployment file that is shared with
// other tests.
//
// If you have changed it intentionally, run `just generate-chinook-configuration`.
pub async fn configure_is_idempotent(
    connection_string: &str,
    chinook_deployment_path: impl AsRef<Path>,
) {
    let expected_value = read_configuration(chinook_deployment_path);

    let args: configuration::RawConfiguration = serde_json::from_value(expected_value.clone())
        .expect("Unable to deserialize as RawConfiguration");
    let environment = HashMap::from([(
        configuration::DEFAULT_CONNECTION_URI_VARIABLE.into(),
        connection_string.into(),
    )]);
    let file_path = PathBuf::new();

    let mut actual = configuration::configure(&file_path, &args, environment)
        .await
        .expect("configuration::configure");

    // Native queries and native mutations are defined by the user and cannot
    // be obtained by introspecting the database. So, add the native queries and
    // mutations back manually.
    let native_queries_config: NativeQueries =
        read_configuration_as("static/chinook-native-queries.json").unwrap();

    let native_mutations_config: NativeMutations =
        read_configuration_as("static/chinook-native-mutations.json").unwrap();

    actual.metadata.native_mutations = native_mutations_config;
    actual.metadata.native_queries = native_queries_config;

    let actual_value = serde_json::to_value(actual).expect("serde_json::to_value");

    assert_eq!(expected_value, actual_value);
}

pub async fn configure_initial_configuration_is_unchanged(
    connection_string: &str,
) -> configuration::RawConfiguration {
    let connection_uri_variable: Variable = "MAGIC_URI".into();
    let args = configuration::RawConfiguration {
        mssql_connection_string: ndc_sqlserver_configuration::ConnectionUri(
            secret::Secret::FromEnvironment {
                variable: connection_uri_variable.clone(),
            },
        ),

        ..configuration::RawConfiguration::empty()
    };

    let environment = HashMap::from([(connection_uri_variable, connection_string.into())]);
    let file_path = PathBuf::new();

    configuration::configure(&file_path, &args, environment)
        .await
        .expect("configuration::configure")
}

fn read_configuration(chinook_deployment_path: impl AsRef<Path>) -> serde_json::Value {
    let file = fs::File::open(get_path_from_project_root(chinook_deployment_path))
        .expect("fs::File::open");
    serde_json::from_reader(file).expect("serde_json::from_reader")
}

fn read_configuration_as<T: DeserializeOwned>(
    chinook_deployment_path: impl AsRef<Path>,
) -> Result<T, serde_json::Error> {
    let v = read_configuration(chinook_deployment_path);
    from_value(v)
}
