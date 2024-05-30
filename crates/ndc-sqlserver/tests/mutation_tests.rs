#[cfg(test)]
pub mod common;

mod basic {
    use crate::common::{database::MSSQLDatabaseConfig, helpers::run_mutation};

    use super::common::fresh_deployments::FreshDeployment;

    #[tokio::test(flavor = "multi_thread")]
    async fn native_mutation_insert_artist_and_return_id() {
        let original_db_config = MSSQLDatabaseConfig::original_db_config();
        let _ndc_metadata = FreshDeployment::create(original_db_config, "static")
            .await
            .unwrap();

        let result = run_mutation("insert_artist_and_return_id").await;

        insta::assert_json_snapshot!(result);
    }
}
