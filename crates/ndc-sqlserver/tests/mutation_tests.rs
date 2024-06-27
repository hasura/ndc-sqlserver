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
        let fresh_deployment = FreshDeployment::create(original_db_config, "static/tests", vec![])
            .await
            .unwrap();

        let result = run_mutation(
            "insert_artist_and_return_id",
            fresh_deployment.connection_uri.clone(),
        )
        .await;

        insta::assert_json_snapshot!(result);
    }

    #[tokio::test(flavor = "multi_thread")]
    #[serial]
    /// Native mutation that selects a relationship.
    async fn native_mutation_insert_artist_and_return_artist() {
        let original_db_config = MSSQLDatabaseConfig::original_db_config();
        let fresh_deployment = FreshDeployment::create(original_db_config, "static/tests", vec![])
            .await
            .unwrap();

        let result = run_mutation(
            "insert_artist_and_return_artist",
            fresh_deployment.connection_uri.clone(),
        )
        .await;

        insta::assert_json_snapshot!(result);
    }

    mod negative_native_mutations_test {
        use crate::common::{
            database::MSSQLDatabaseConfig,
            helpers::{run_mutation_fail, run_query_with_connection_uri},
        };

        use super::*;

        use hyper::StatusCode;
        use serial_test::serial;

        #[tokio::test(flavor = "multi_thread")]
        #[serial]
        async fn test_atomicity_native_mutations() {
            let original_db_config = MSSQLDatabaseConfig::original_db_config();
            let fresh_deployment =
                FreshDeployment::create(original_db_config, "static/tests", vec![])
                    .await
                    .unwrap();

            // Mutation that tries to insert two records with the same primary key
            // So, the second mutation is expected to fail. Since, we run the whole
            // thing in a transaction, we expect the whole transaction to be rolled
            // back.
            let _ = run_mutation_fail(
                "fail_insert_artist_and_return_id",
                fresh_deployment.connection_uri.clone(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .await;

            let result = run_query_with_connection_uri(
                "fetch_artist_count",
                &fresh_deployment.connection_uri.clone(),
            )
            .await;

            insta::assert_json_snapshot!(result);
        }
    }
}

mod stored_procedures {
    use crate::common::{
        configuration::get_path_from_project_root,
        database::MSSQLDatabaseConfig,
        helpers::{run_mutation, run_mutation_fail},
    };

    use super::common::fresh_deployments::FreshDeployment;

    use hyper::StatusCode;
    use serial_test::serial;

    #[tokio::test(flavor = "multi_thread")]
    #[serial]
    async fn basic_stored_procedure_execution() {
        let original_db_config = MSSQLDatabaseConfig::original_db_config();

        let stored_procs_setup_file_path =
            get_path_from_project_root("static/tests/stored_procedures_sql/return_one.sql");

        let fresh_deployment = FreshDeployment::create(
            original_db_config,
            "static/tests",
            vec![stored_procs_setup_file_path],
        )
        .await
        .unwrap();

        let result = run_mutation(
            "mutations/stored_procedures/return_one",
            fresh_deployment.connection_uri.clone(),
        )
        .await;

        insta::assert_json_snapshot!(result);
    }

    #[tokio::test(flavor = "multi_thread")]
    #[serial]
    async fn execute_stored_procedure_without_providing_arguments() {
        let original_db_config = MSSQLDatabaseConfig::original_db_config();

        let stored_procs_setup_file_path = get_path_from_project_root(
            "static/tests/stored_procedures_sql/get_customer_details.sql",
        );

        let fresh_deployment = FreshDeployment::create(
            original_db_config,
            "static/tests",
            vec![stored_procs_setup_file_path],
        )
        .await
        .unwrap();

        let result = run_mutation_fail(
            "mutations/stored_procedures/get_customer_details_without_arguments",
            fresh_deployment.connection_uri.clone(),
            StatusCode::BAD_REQUEST,
        )
        .await;

        insta::assert_json_snapshot!(result);
    }
}
