//! Common functions used across test cases.

use std::fs;
use std::path::PathBuf;

use axum::http::StatusCode;
use axum_test_helper::TestClient;
use serde::Deserialize;

pub mod configuration;
pub mod database;
pub mod fresh_deployments;

use ndc_sqlserver::connector;

use self::database::MSSQLDatabaseConfig;

/// Run a query against the server, get the result, and compare against the snapshot.
pub async fn run_query(testname: &str) -> serde_json::Value {
    run_against_server("query", testname).await
}

/// Run a query against the server, get the result, and compare against the snapshot.
pub async fn run_mutation(testname: &str) -> serde_json::Value {
    run_against_server("mutation", testname).await
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ExactExplainResponse {
    pub details: ExplainDetails,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ExplainDetails {
    #[serde(rename = "SQL Query")]
    pub query: String,
    #[serde(rename = "Execution Plan")]
    pub plan: String,
}

// TODO(PY): add run_explain_mutation
/// Run a query against the server, get the result, and compare against the snapshot.
pub async fn run_explain(testname: &str) -> ExactExplainResponse {
    run_against_server("query/explain", testname).await
}

/// Run a query against the server, get the result, and compare against the snapshot.
pub async fn get_schema() -> ndc_sdk::models::SchemaResponse {
    make_request(|client| client.get("/schema")).await
}

/// Run an action against the server, and get the response.
async fn run_against_server<Response: for<'a> serde::Deserialize<'a>>(
    action: &str,
    testname: &str,
) -> Response {
    let path = format!("/{}", action);
    let body = match fs::read_to_string(format!("tests/goldenfiles/{}.json", testname)) {
        Ok(body) => body,
        Err(err) => {
            println!("Error: {}", err);
            panic!("error look up");
        }
    };
    make_request(|client| {
        client
            .post(&path)
            .header("Content-Type", "application/json")
            .body(body)
    })
    .await
}

pub async fn create_router() -> axum::Router {
    let _ = env_logger::builder().is_test(true).try_init();

    // work out where the deployment configs live
    let test_deployment_file = get_deployment_file();

    let setup = connector::SQLServer::default();

    // initialise server state with the static configuration.
    let state = ndc_sdk::default_main::init_server_state::<connector::SQLServer>(
        setup,
        test_deployment_file,
    )
    .await
    .unwrap();

    // create a fresh client
    ndc_sdk::default_main::create_router(state, None)
}

/// Make a single request against the server, and get the response.
async fn make_request<Response: for<'a> serde::Deserialize<'a>>(
    request: impl FnOnce(axum_test_helper::TestClient) -> axum_test_helper::RequestBuilder,
) -> Response {
    // create a fresh client
    let router = create_router().await;
    let client = TestClient::new(router);

    // make the request
    let response = request(client).send().await;

    // ensure we get a successful response
    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Expected a successful response but got status {}.\nBody:\n{}",
        response.status(),
        response.text().await
    );

    // deserialize the response
    response.json().await
}

/// Check if all keywords are contained in this vector of strings.
/// Used to check the output of EXPLAIN. We use this method instead of
/// snapshot testing because small details (like cost) can change from
/// run to run rendering the output unstable.
pub fn is_contained_in_lines(keywords: Vec<&str>, lines: String) {
    tracing::info!("expected keywords: {:?}\nlines: {}", keywords, lines,);
    assert!(keywords.iter().all(|&s| lines.contains(s)));
}

/// Find the project root via the crate root provided by `cargo test`,
/// and get our single static configuration file.
pub fn get_deployment_file() -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("../../static/");
    d
}
