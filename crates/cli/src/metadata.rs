//! Structures that represent the connector metadata definition.
//!
//! See https://github.com/hasura/ndc-hub/blob/main/rfcs/0011-cli-and-connector-packaging.md

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorMetadataDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>, // "v1"
    pub packaging_definition: PackagingDefinition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_toolchain_definition: Option<NativeToolchainDefinition>,
    pub supported_environment_variables: Vec<EnvironmentVariableDefinition>,
    pub commands: Commands,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cli_plugin: Option<CliPluginDefinition>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub docker_compose_watch: DockerComposeWatch,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_page: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", tag = "type")]
pub enum PackagingDefinition {
    PrebuiltDockerImage(PrebuiltDockerImagePackaging),
    ManagedDockerBuild,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrebuiltDockerImagePackaging {
    pub docker_image: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeToolchainDefinition {
    pub commands: NativeToolchainCommands,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeToolchainCommands {
    pub start: Command,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Command>,
    pub watch: Command,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Command {
    String(String),
    Dockerized(DockerizedCommand),
    ShellScript(ShellScriptCommand),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Commands {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Command>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch: Option<Command>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_schema_and_capabilities: Option<Command>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_configuration: Option<Command>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentVariableDefinition {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    pub required: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerizedCommand {
    #[serde(rename = "type")]
    pub command_type: String, // "Dockerized"
    pub docker_image: String,
    pub command_args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellScriptCommand {
    #[serde(rename = "type")]
    pub command_type: String, // "ShellScript"
    pub bash: String,
    pub powershell: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum CliPluginDefinition {
    Binary {
        name: String,
        version: String,
    },
    BinaryInline {
        platforms: Vec<BinaryCliPluginPlatform>,
    },
    Docker {
        docker_image: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinaryCliPluginPlatform {
    pub selector: PlatformSelector,
    pub uri: String,
    pub sha256: String,
    pub bin: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlatformSelector {
    DarwinArm64,
    LinuxArm64,
    DarwinAmd64,
    WindowsAmd64,
    LinuxAmd64,
}

pub type DockerComposeWatch = Vec<DockerComposeWatchItem>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerComposeWatchItem {
    pub path: String,
    pub action: DockerComposeWatchAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ignore: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DockerComposeWatchAction {
    Rebuild,
    Sync,
    #[serde(rename = "sync+restart")]
    SyncAndRestart,
}
