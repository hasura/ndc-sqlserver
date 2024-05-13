mod metadata;

use std::fs;

use std::path::PathBuf;

use clap::Subcommand;

use configuration::environment::Environment;
use configuration::validate_raw_configuration;
use ndc_sqlserver_configuration as configuration;

const UPDATE_ATTEMPTS: u8 = 3;

/// The various contextual bits and bobs we need to run.
#[derive(Debug)]
pub struct Context<Env: Environment> {
    pub context_path: PathBuf,
    pub release_version: Option<&'static str>,
    pub environment: Env,
}

/// The command invoked by the user.
#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    /// Initialize a configuration in the current (empty) directory.
    Initialize {
        #[arg(long)]
        /// Whether to create the hasura connector metadata.
        with_metadata: bool,
    },
    /// Update the configuration by introspecting the database, using the configuration options.
    Update,
}

/// The set of errors that can go wrong _in addition to_ generic I/O or parsing errors.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("directory is not empty")]
    DirectoryIsNotEmpty,
}

/// Run a command in a given directory.
pub async fn run(command: Command, context: Context<impl Environment>) -> anyhow::Result<()> {
    match command {
        Command::Initialize { with_metadata } => initialize(with_metadata, context).await?,
        Command::Update => update(context).await?,
    };
    Ok(())
}

/// Initialize an empty directory with an empty connector configuration.
///
/// An empty configuration contains default settings and options, and is expected to be filled with
/// information such as the database connection string by the user, and later on metadata
/// information via introspection.
///
/// Optionally, this can also create the connector metadata, which is used by the Hasura CLI to
/// automatically work with this CLI as a plugin.
async fn initialize(with_metadata: bool, context: Context<impl Environment>) -> anyhow::Result<()> {
    let configuration_file = context
        .context_path
        .join(configuration::CONFIGURATION_FILENAME);
    fs::create_dir_all(&context.context_path)?; // TODO(PY): .await

    // refuse to initialize the directory unless it is empty
    let mut items_in_dir = fs::read_dir(&context.context_path)?;
    if items_in_dir.next().is_some() {
        Err(Error::DirectoryIsNotEmpty)?;
    }

    // let raw_configuration = match context.uri {
    //     Some(uri) => configuration::RawConfiguration::with_mssql_connection_string(uri),
    //     None => configuration::RawConfiguration::empty(),
    // };
    // create the configuration file
    fs::write(
        configuration_file,
        serde_json::to_string_pretty(&configuration::RawConfiguration::empty())? + "\n",
    )?;

    // create the jsonschema file
    let configuration_jsonschema_file_path = context
        .context_path
        .join(configuration::CONFIGURATION_JSONSCHEMA_FILENAME);

    let output = schemars::schema_for!(configuration::RawConfiguration);
    fs::write(
        configuration_jsonschema_file_path,
        serde_json::to_string_pretty(&output)? + "\n",
    )?;

    // if requested, create the metadata
    if with_metadata {
        let metadata_dir = context.context_path.join(".hasura-connector");
        let _ = fs::create_dir(&metadata_dir);
        let metadata_file = metadata_dir.join("connector-metadata.yaml");
        let metadata = metadata::ConnectorMetadataDefinition {
            packaging_definition: metadata::PackagingDefinition::PrebuiltDockerImage(
                metadata::PrebuiltDockerImagePackaging {
                    docker_image: format!(
                        "ghcr.io/hasura/ndc-sqlserver:{}",
                        context.release_version.unwrap_or("latest")
                    ),
                },
            ),
            supported_environment_variables: vec![metadata::EnvironmentVariableDefinition {
                name: "CONNECTION_URI".to_string(),
                description: "The SQL server connection URI".to_string(),
                default_value: None,
            }],
            commands: metadata::Commands {
                update: Some("hasura-ndc-sqlserver update".to_string()),
                watch: None,
            },
            cli_plugin: Some(metadata::CliPluginDefinition {
                name: "ndc-sqlserver".to_string(),
                version: context.release_version.unwrap_or("latest").to_string(),
            }),
            docker_compose_watch: vec![metadata::DockerComposeWatchItem {
                path: "./".to_string(),
                target: Some("/etc/connector".to_string()),
                action: metadata::DockerComposeWatchAction::SyncAndRestart,
                ignore: vec![],
            }],
        };

        fs::write(metadata_file, serde_yaml::to_string(&metadata)?)?;
    }

    Ok(())
}

/// Update the configuration in the current directory by introspecting the database.
///
/// This expects a configuration with a valid connection URI.
async fn update(context: Context<impl Environment>) -> anyhow::Result<()> {
    // It is possible to change the file in the middle of introspection.
    // We want to detect this scenario and retry, or fail if we are unable to.
    // We do that with a few attempts.
    // update_uri_from_context(&context).await?;
    for _attempt in 1..=UPDATE_ATTEMPTS {
        let configuration_file_path = context
            .context_path
            .join(configuration::CONFIGURATION_FILENAME);
        let raw_configuration: configuration::RawConfiguration = {
            let configuration_file_contents =
                read_config_file_contents(&configuration_file_path).await?;
            serde_json::from_str(&configuration_file_contents)?
        };
        let input = validate_raw_configuration(
            &configuration_file_path,
            raw_configuration.clone(),
            &context.environment,
        )
        .await?;
        let output = configuration::configure(&input).await?;

        // Check that the input file did not change since we started introspecting,
        let raw_configuration_before_write: configuration::RawConfiguration = {
            let configuration_file_contents =
                read_config_file_contents(&configuration_file_path).await?;
            serde_json::from_str(&configuration_file_contents)?
        };

        // and skip this attempt if it has.
        if raw_configuration_before_write == raw_configuration {
            // If the introspection result is different than the current config,
            // change it. Otherwise, continue.
            if input != output {
                fs::write(
                    &configuration_file_path,
                    serde_json::to_string_pretty(&output)? + "\n",
                )?;
            } else {
                // The configuration is up-to-date. Nothing to do.
            }
            return Ok(());
        } else {
            // Input file changed before write.
        }
    }

    // We ran out of attempts.
    Err(anyhow::anyhow!(
        "Cannot override configuration: input changed before write."
    ))
}

async fn read_config_file_contents(configuration_file_path: &PathBuf) -> anyhow::Result<String> {
    fs::read_to_string(configuration_file_path)
        // .await
        .map_err(|err| {
            if err.kind() == std::io::ErrorKind::NotFound {
                anyhow::anyhow!(
                    "{}: No such file or directory. Perhaps you meant to 'initialize' first?",
                    configuration_file_path.display()
                )
            } else {
                anyhow::anyhow!(err)
            }
        })
}

// / Since the URI is not mandatory for `initialize`, we need to update it first. Please not that this will only update
// / the URI if the raw configuration is empty.
// async fn update_uri_from_context(context: &Context<impl Environment>) -> anyhow::Result<()> {
//     let configuration_file_path = context
//         .context_path
//         .join(configuration::CONFIGURATION_FILENAME);
//     let mut input: configuration::RawConfiguration = {
//         let configuration_file_contents =
//             read_config_file_contents(&configuration_file_path).await?;
//         serde_json::from_str(&configuration_file_contents)?
//     };
//     if input.mssql_connection_string.is_empty() {
//         if let Some(uri) = &context.uri {
//             input.mssql_connection_string = uri.clone();
//         } else {
//             return Err(anyhow::anyhow!(
//                 "Cannot introspect without a connection URI. Please provide one."
//             ));
//         }
//         fs::write(
//             &configuration_file_path,
//             serde_json::to_string_pretty(&input)? + "\n",
//         )?;
//     }
//     Ok(())
// }
