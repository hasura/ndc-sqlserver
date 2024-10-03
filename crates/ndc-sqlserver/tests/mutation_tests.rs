#[cfg(test)]
pub mod common;

mod native_mutations {
    use crate::common::{database::MSSQLDatabaseConfig, helpers::run_mutation};

    use super::common::fresh_deployments::FreshDeployment;

    #[tokio::test(flavor = "multi_thread")]
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

        #[tokio::test(flavor = "multi_thread")]
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
                StatusCode::UNPROCESSABLE_ENTITY,
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

    #[tokio::test(flavor = "multi_thread")]
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

    #[tokio::test(flavor = "multi_thread")]
    async fn execute_stored_procedure_with_relationships() {
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

        let result = run_mutation(
            "mutations/stored_procedures/get_customer_details_with_invoices",
            fresh_deployment.connection_uri.clone(),
        )
        .await;

        insta::assert_json_snapshot!(result);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn execute_stored_procedures_concurrently() {
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

        let fresh_deployment_1 = fresh_deployment.connection_uri.clone();

        let result1 = tokio::spawn(async move {
            let connection_uri = fresh_deployment_1.clone();

            run_mutation(
                "mutations/stored_procedures/get_customer_details_with_invoices",
                connection_uri.clone(),
            )
            .await
        });

        let result2 = tokio::spawn(async move {
            run_mutation(
                "mutations/stored_procedures/get_customer_details_with_invoices",
                fresh_deployment.connection_uri.clone(),
            )
            .await
        });

        // If either of the results are `Err`, then this will panic and the test will fail.
        let (_result1, _result2) = tokio::try_join!(result1, result2).unwrap();
    }
}
