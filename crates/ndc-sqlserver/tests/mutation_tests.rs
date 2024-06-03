#[cfg(test)]
pub mod common;

mod native_mutations {
    use crate::common::{database::MSSQLDatabaseConfig, helpers::run_mutation};

    use super::common::fresh_deployments::FreshDeployment;

    use serial_test::serial;

    #[tokio::test(flavor = "multi_thread")]
    #[serial]
    async fn native_mutation_insert_artist_and_return_id() {
        let original_db_config = MSSQLDatabaseConfig::original_db_config();
        let _ndc_metadata = FreshDeployment::create(original_db_config, "static")
            .await
            .unwrap();

        let result = run_mutation("insert_artist_and_return_id").await;

        insta::assert_json_snapshot!(result);
    }
}

mod negative_native_mutations_test {
    use crate::common::{
        database::MSSQLDatabaseConfig,
        helpers::{run_mutation_fail, run_query},
    };

    use super::common::fresh_deployments::FreshDeployment;

    use hyper::StatusCode;
    use serial_test::serial;

    #[tokio::test(flavor = "multi_thread")]
    #[serial]
    async fn test_atomicity_native_mutations() {
        let original_db_config = MSSQLDatabaseConfig::original_db_config();
        let _ndc_metadata = FreshDeployment::create(original_db_config, "static")
            .await
            .unwrap();

        // Mutation that tries to insert two records with the same primary key
        // So, the second mutation is expected to fail. Since, we run the whole
        // thing in a transaction, we expect the whole transaction to be rolled
        // back.
        let _ = run_mutation_fail(
            "fail_insert_artist_and_return_id",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .await;

        let result = run_query("fetch_artist_count").await;

        insta::assert_json_snapshot!(result);
    }
}
