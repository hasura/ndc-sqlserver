//! Deployment configuration functions used across test cases.
//! Use via helpers in `mod.rs` rather than directly.

use std::path::{Path, PathBuf};

use tokio::fs;

use ndc_sqlserver_configuration::RawConfiguration;

/// Find the project root via the crate root provided by `cargo test`,
/// and get our single static configuration file.
/// This depends on the convention that all our crates live in `/crates/<name>`
/// and will break in the unlikely case that we change this
pub fn get_path_from_project_root(deployment_path: impl AsRef<Path>) -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("../../");
    d.push(deployment_path);
    d
}

/// Load NDC metadata at `main_ndc_metadata_path`
/// replace url with `new_mssql_url`
/// save at `new_ndc_metadata_path`
pub async fn copy_ndc_metadata_with_new_mssql_url(
    main_ndc_metadata_path: impl AsRef<Path>,
    new_connection_uri: &str,
    temp_deploys_path: std::path::PathBuf,
    db_name: &str,
) -> anyhow::Result<()> {
    let ndc_metadata_dir_path = get_path_from_project_root(main_ndc_metadata_path);
    let ndc_metadata_path = ndc_metadata_dir_path.join("configuration.json");
    let mut new_ndc_metadata = serde_json::from_str::<RawConfiguration>(
        &std::fs::read_to_string(&ndc_metadata_path)
            .map_err(|err| anyhow::anyhow!("{}: {}", &ndc_metadata_path.display(), err))?,
    )
    .map_err(|err| anyhow::anyhow!("{}: {}", &ndc_metadata_path.display(), err))?;

    set_connection_uri(&mut new_ndc_metadata, new_connection_uri.into());

    let temp_deploys_path = get_path_from_project_root(temp_deploys_path);

    let new_ndc_metadata_dir = temp_deploys_path.join(db_name);

    fs::create_dir_all(&new_ndc_metadata_dir)
        .await
        .map_err(|err| anyhow::anyhow!("{}: {}", &new_ndc_metadata_dir.display(), err))?;

    let new_ndc_metadata_path = new_ndc_metadata_dir.join("configuration.json");
    fs::write(
        get_path_from_project_root(&new_ndc_metadata_path),
        serde_json::to_string_pretty(&new_ndc_metadata)?,
    )
    .await
    .map_err(|err| {
        anyhow::anyhow!(
            "{}: {}",
            &get_path_from_project_root(new_ndc_metadata_path).display(),
            err
        )
    })?;
    Ok(())
}

/// Erase test NDC metadata file created at `ndc_metadata_path`
pub async fn delete_ndc_metadata(ndc_metadata_path: impl AsRef<Path>) -> anyhow::Result<()> {
    let absolute_path = get_path_from_project_root(ndc_metadata_path);
    fs::remove_dir_all(&absolute_path).await.map_err(|err| {
        anyhow::anyhow!(
            "{}: {}",
            &get_path_from_project_root(absolute_path).display(),
            err
        )
    })?;

    Ok(())
}

fn set_connection_uri(input: &mut RawConfiguration, connection_uri: String) {
    input.mssql_connection_string = connection_uri.into();
}
