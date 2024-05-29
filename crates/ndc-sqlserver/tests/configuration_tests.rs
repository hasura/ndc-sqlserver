use std::fs;
use std::path::Path;

use ndc_sqlserver_configuration as configuration;
use similar_asserts::assert_eq;

mod common;
use common::configuration::get_path_from_project_root;

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

    let mut args: configuration::RawConfiguration = serde_json::from_value(expected_value.clone())
        .expect("Unable to deserialize as RawConfiguration");

    args.mssql_connection_string = connection_string.to_string();

    let actual = configuration::configure(&args)
        .await
        .expect("configuration::configure");

    let actual_value = serde_json::to_value(actual).expect("serde_json::to_value");

    assert_eq!(expected_value, actual_value);
}

pub async fn configure_initial_configuration_is_unchanged(
    connection_string: &str,
) -> configuration::RawConfiguration {
    let args = configuration::RawConfiguration {
        mssql_connection_string: connection_string.to_string(),

        ..configuration::RawConfiguration::empty()
    };

    configuration::configure(&args)
        .await
        .expect("configuration::configure")
}

fn read_configuration(chinook_deployment_path: impl AsRef<Path>) -> serde_json::Value {
    let file = fs::File::open(get_path_from_project_root(chinook_deployment_path))
        .expect("fs::File::open");
    serde_json::from_reader(file).expect("serde_json::from_reader")
}
