//! Common functions used across test cases.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use axum::http::StatusCode;
use axum_test_helper::TestClient;

use serde::Deserialize;

use ndc_sqlserver::connector;

pub const SQLSERVER_CONNECTION_STRING: &str =
    "Server=localhost,64003;Uid=SA;Database=Chinook;Pwd=Password!;TrustServerCertificate=true";

/// Create a test client from a router.
pub fn create_client(router: axum::Router) -> TestClient {
    TestClient::new(router)
}

/// Run a query against the server, get the result, and compare against the snapshot.
pub async fn run_query(testname: &str) -> serde_json::Value {
    run_query_with_connection_uri(testname, SQLSERVER_CONNECTION_STRING).await
}

/// Run a query against the server, get the result, and compare against the snapshot.
pub async fn run_query_with_connection_uri(
    testname: &str,
    connection_string: &str,
) -> serde_json::Value {
    let router = create_router(connection_string).await;
    let client = create_client(router);
    run_against_server(&client, "query", testname, StatusCode::OK).await
}

/// Run a query against the server, get the result, and compare against the snapshot.
pub async fn run_mutation(testname: &str, db_connection_string: String) -> serde_json::Value {
    let router = create_router(&db_connection_string).await;
    let client = create_client(router);
    run_against_server(&client, "mutation", testname, StatusCode::OK).await
}

/// Run a query against the server, get the result, and compare against the snapshot.
pub async fn run_mutation_fail(
    testname: &str,
    db_connection_string: String,
    expected_status: StatusCode,
) -> serde_json::Value {
    let router = create_router(&db_connection_string).await;
    let client = create_client(router);
    run_against_server(&client, "mutation", testname, expected_status).await
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
    let router = create_router(SQLSERVER_CONNECTION_STRING).await;
    let client = create_client(router);
    run_against_server(&client, "query/explain", testname, StatusCode::OK).await
}

/// Run a query against the server, get the result, and compare against the snapshot.
pub async fn get_schema(router: axum::Router) -> ndc_sdk::models::SchemaResponse {
    let client = create_client(router);
    make_request(&client, |client| client.get("/schema"), StatusCode::OK).await
}

/// Run an action against the server, and get the response.
async fn run_against_server<Response: for<'a> serde::Deserialize<'a>>(
    client: &TestClient,
    action: &str,
    testname: &str,
    expected_status: StatusCode,
) -> Response {
    let path = format!("/{}", action);
    let body = match fs::read_to_string(format!("tests/goldenfiles/{}.json", testname)) {
        Ok(body) => body,
        Err(err) => {
            println!("Error: {}", err);
            panic!("error look up");
        }
    };
    make_request(
        client,
        |client| {
            client
                .post(&path)
                .header("Content-Type", "application/json")
                .body(body)
        },
        expected_status,
    )
    .await
}

pub async fn create_router(connection_uri: &str) -> axum::Router {
    let _ = env_logger::builder().is_test(true).try_init();

    // work out where the deployment configs live
    let test_deployment_file = get_deployment_file();

    // Initialize server state with the configuration above, injecting the URI.
    let environment = HashMap::from([(
        ndc_sqlserver_configuration::DEFAULT_CONNECTION_URI_VARIABLE.into(),
        connection_uri.to_string(),
    )]);

    let setup = connector::SQLServerSetup::new(environment);

    // initialise server state with the static configuration.
    let state = ndc_sdk::default_main::init_server_state(setup, test_deployment_file)
        .await
        .unwrap();

    // create a fresh client
    ndc_sdk::default_main::create_router(state, None)
}

/// Make a single request against a new server, and get the response.
async fn make_request<Response: for<'a> serde::Deserialize<'a>>(
    client: &TestClient,
    request: impl FnOnce(&TestClient) -> axum_test_helper::RequestBuilder,
    expected_status: StatusCode,
) -> Response {
    // make the request
    let response = request(client).send().await;
    let status = response.status();
    let body = response.bytes().await;

    // ensure we get a successful response
    assert_eq!(
        status,
        expected_status,
        "Expected status code {} but got status {}.\nBody:\n{}",
        expected_status,
        status,
        std::str::from_utf8(&body).unwrap()
    );

    // deserialize the response
    serde_json::from_slice(&body).unwrap_or_else(|err| {
        panic!(
            "Invalid JSON in response body.\nError: {}\nBody:\n{:?}\n",
            err,
            std::str::from_utf8(&body).unwrap()
        )
    })
}

/// Check if all keywords are contained in this vector of strings.
/// Used to check the output of EXPLAIN. We use this method instead of
/// snapshot testing because small details (like cost) can change from
/// run to run rendering the output unstable.
pub fn is_contained_in_lines(keywords: &[&str], lines: &str) {
    tracing::info!("expected keywords: {:?}\nlines: {}", keywords, lines,);
    assert!(keywords.iter().all(|&s| lines.contains(s)));
}

/// Find the project root via the crate root provided by `cargo test`,
/// and get our single static configuration file.
pub fn get_deployment_file() -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("../../static/tests/");
    d
}
