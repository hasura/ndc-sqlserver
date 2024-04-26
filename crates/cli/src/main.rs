//! The CLI application. This is used to configure a deployment of ndc-sqlserver.
//!
//! This is intended to be automatically downloaded and invoked via the Hasura CLI, as a plugin.
//! It is unlikely that end-users will use it directly.

use std::env;
use std::path::PathBuf;

use clap::Parser;

use ndc_sqlserver_cli::*;
// use ndc_sqlserver_ndc_sqlserverql_src as configuration;

/// The release version specified at build time.
///
/// We should use the latest version if this is not specified.
const RELEASE_VERSION: Option<&str> = option_env!("RELEASE_VERSION");

/// The command-line arguments.
#[derive(Debug, Parser)]
#[command(
    version = RELEASE_VERSION.unwrap_or("unknown"),
    about = "Configuration tool for ndc-sqlserver"
)]
pub struct Args {
    /// The path to the configuration. Defaults to the current directory.
    #[arg(long = "context", env = "HASURA_PLUGIN_CONNECTOR_CONTEXT_PATH")]
    pub context_path: Option<PathBuf>,
    /// The command to invoke.
    #[command(subcommand)]
    // TODO(PY): Add command in lib.rs
    pub subcommand: Command,
    #[arg(long = "connection-uri", env = "CONNECTION_URI")]
    pub url: String,
}

/// The application entrypoint. It pulls information from the environment and then calls the [run]
/// function. The library remains unaware of the environment, so that we can more easily test it.
#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    // Default the context path to the current directory.
    let context_path = match args.context_path {
        Some(path) => path,
        None => env::current_dir()?,
    };
    let uri = args.url;
    let context = Context {
        context_path,
        release_version: RELEASE_VERSION,
        uri,
    };
    run(args.subcommand, context).await?;
    Ok(())
}
