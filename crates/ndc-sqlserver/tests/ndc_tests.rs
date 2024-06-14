//! Runs the tests provided by the ndc-spec.
pub mod common;

#[cfg(test)]
mod ndc_tests {
    use super::common;
    use std::net;

    #[tokio::test]
    async fn test_connector() -> Result<(), Vec<ndc_test::reporter::FailedTest>> {
        let router = common::helpers::create_router(common::POSTGRESQL_CONNECTION_STRING).await;
        let server = hyper::Server::bind(&net::SocketAddr::new(
            net::IpAddr::V4(net::Ipv4Addr::LOCALHOST),
            0,
        ))
        .serve(router.into_make_service());

        let base_path =
            reqwest::Url::parse(format!("http://{}", server.local_addr()).as_str()).unwrap();

        eprintln!("Starting the server on {}", base_path);

        tokio::task::spawn(async {
            if let Err(err) = server.await {
                eprintln!("Server error: {}", err);
            }
        });

        let configuration = ndc_test::client::Configuration {
            base_path,
            client: reqwest::Client::new(),
        };

        let mut test_results = ndc_test::reporter::TestResults::default();

        ndc_test::test_connector(
            &ndc_test::configuration::TestConfiguration {
                seed: None,
                snapshots_dir: None,
                gen_config: Default::default(),
            },
            &configuration,
            &mut test_results,
        )
        .await;
        if test_results.failures.is_empty() {
            Ok(())
        } else {
            Err(test_results.failures)
        }
    }
}
