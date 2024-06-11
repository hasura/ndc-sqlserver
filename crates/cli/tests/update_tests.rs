mod common;

use tokio::fs;

use ndc_sqlserver_cli::*;
use ndc_sqlserver_configuration::environment::FixedEnvironment;
use ndc_sqlserver_configuration::RawConfiguration;
use ndc_sqlserver_configuration::{self as configuration};

const CONNECTION_URI: &str =
    "Server=localhost,64003;Uid=SA;Database=Chinook;Pwd=Password!;TrustServerCertificate=true";

#[tokio::test]
async fn test_update_configuration() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;

    let connection_uri =
        configuration::ConnectionUri(configuration::secret::Secret::FromEnvironment {
            variable: "CONNECTION_URI".into(),
        });

    {
        let configuration_file_path = dir.path().join("configuration.json");
        // let connection_settings =
        //     configuration::version3::connection_settings::DatabaseConnectionSettings {
        //         connection_uri: connection_uri.clone(),
        //         ..configuration::version3::connection_settings::DatabaseConnectionSettings::empty()
        //     };
        let input = configuration::RawConfiguration {
            mssql_connection_string: connection_uri.clone(),
            ..configuration::RawConfiguration::empty()
        };
        dbg!("input", input.clone());
        fs::write(
            configuration_file_path.clone(),
            serde_json::to_string(&input)?,
        )
        .await?;
        let contents = fs::read_to_string(configuration_file_path).await?;
        dbg!("contents before update", contents.clone());
    }
    dbg!("before run");

    let environment =
        FixedEnvironment::from([("CONNECTION_URI".into(), CONNECTION_URI.to_string())]);
    dbg!(environment.clone());
    let context = Context {
        context_path: dir.path().to_owned(),
        environment,
        release_version: None,
    };
    run(Command::Update, context).await?;
    dbg!("after run");

    let configuration_file_path = dir.path().join("configuration.json");
    dbg!(configuration_file_path.clone());
    assert!(configuration_file_path.exists());
    let contents = fs::read_to_string(configuration_file_path).await?;
    // dbg!("contents after update", contents.clone());
    common::assert_ends_with_newline(&contents);
    let output: RawConfiguration = serde_json::from_str(&contents)?;
    match output {
        configuration::RawConfiguration {
            mssql_connection_string,
            metadata,
            ..
        } => {
            assert_eq!(mssql_connection_string, connection_uri);
            let some_table_metadata = metadata.tables.0.get("Artist");
            assert!(some_table_metadata.is_some());
        }
    }

    Ok(())
}
