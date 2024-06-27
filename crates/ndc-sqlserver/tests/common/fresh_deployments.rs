use crate::common::configuration;
use crate::common::database;
use std::fs;
use std::path::{Path, PathBuf};

use super::configuration::get_path_from_project_root;
use super::database::create_mssql_connection;
use super::database::MSSQLDatabaseConfig;

#[derive(Debug)]
pub struct FreshDeployment {
    pub db_name: String,
    pub ndc_metadata_path: PathBuf,
    pub connection_uri: String,
    admin_connection_uri: String, // for dropping after
}

impl FreshDeployment {
    /// Create a fresh copy of the database and its associated NDC metadata.
    pub async fn create(
        original_connection_db_config: MSSQLDatabaseConfig,
        ndc_metadata_path: impl AsRef<Path>,
        data_setup_file_paths: Vec<PathBuf>,
    ) -> anyhow::Result<FreshDeployment> {
        let new_db_config =
            database::create_fresh_database(original_connection_db_config.clone()).await;

        let temp_deploys_path = PathBuf::from("static/tests/temp-deploys");

        let new_connection_uri = new_db_config.construct_uri();

        configuration::copy_ndc_metadata_with_new_mssql_url(
            ndc_metadata_path,
            &new_connection_uri,
            temp_deploys_path,
            &new_db_config.db_name,
        )
        .await?;

        let new_ndc_metadata_path =
            PathBuf::from("static/tests/temp-deploys").join(&new_db_config.db_name);

        let init_db_sql_file_path =
            get_path_from_project_root("static/tests/chinook-sqlserver.sql");

        let init_db_sql = fs::read_to_string(init_db_sql_file_path).unwrap();

        let mut new_db_connection = create_mssql_connection(&new_db_config).await;

        new_db_connection
            .simple_query(init_db_sql)
            .await
            .expect("Error initializing the newly created DB");

        for file_path in data_setup_file_paths.into_iter() {
            let query = fs::read_to_string(file_path.clone()).unwrap();

            new_db_connection.simple_query(query).await.expect(
                format!("Error in running the query present in the file: {file_path:#?}").as_str(),
            );
        }

        Ok(FreshDeployment {
            db_name: new_db_config.db_name,
            ndc_metadata_path: new_ndc_metadata_path,
            connection_uri: new_connection_uri,
            admin_connection_uri: original_connection_db_config.construct_uri(),
        })
    }
}

impl Drop for FreshDeployment {
    fn drop(&mut self) {
        // To take ownership of the data so that we can move it into a future, we must replace it
        // with something else.
        //
        // We use empty strings, which isn't ideal, but if the compiler has done its job, there
        // should be no dangling references to these anyway.
        //
        // We then swap them in so that we take ownership of the properties we need.
        let mut db_name = "".to_string();
        let mut ndc_metadata_path = PathBuf::from("");
        let mut admin_connection_uri = "".to_string();
        std::mem::swap(&mut self.db_name, &mut db_name);
        std::mem::swap(&mut self.ndc_metadata_path, &mut ndc_metadata_path);
        std::mem::swap(&mut self.admin_connection_uri, &mut admin_connection_uri);

        // In order to call async behavior from a synchronous `Drop`, we must block on it.
        // This magic incantation seems to do the trick. Others did not.
        let result: anyhow::Result<()> = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async move {
                let drop_db_result = database::drop_database(&db_name, admin_connection_uri).await;

                match drop_db_result {
                    Err(e) => println!("Error while dropping the temp db: {e}"),
                    Ok(()) => {}
                }

                configuration::delete_ndc_metadata(&ndc_metadata_path)
                    .await
                    .map_err(|e| {
                        format!("Error while deleting the temporary NDC metadata folder: {e}")
                    })
                    .unwrap();
                Ok(())
            })
        });
        match result {
            Ok(()) => (),
            // We must not panic during a `drop`, so we just print the error in a feeble attempt to
            // do the right thing.
            Err(error) => eprintln!("Error while dropping FreshDeployment: {error}"),
        }
    }
}
