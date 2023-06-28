use ndc_postgres::*;

use axum::{
    body::{Bytes, Full},
    response::Json,
    response::Response,
    routing::get,
    routing::post,
    Router,
};

use axum::extract::{Path, Query};
use serde_json::Value;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/id/:id", get(id))
        .route("/json", post(json))
        .route("/select", get(select));

    let server =
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service());

    println!("Starting axum server at 0.0.0.0:3000");

    server.await.unwrap();
}

// routes

async fn root() -> &'static str {
    "hi"
}

async fn id(
    Path(user_id): Path<i64>,
    Query(params): Query<HashMap<String, String>>,
) -> Response<Full<Bytes>> {
    Response::builder()
        .header("x-powered-by", "benchmark")
        .header("Content-Type", "text/plain")
        .body(Full::from(format!("{} {}", user_id, params["name"])))
        .unwrap()
}

async fn json(Json(payload): Json<serde_json::Value>) -> Json<Value> {
    Json(payload)
}

async fn select() -> Json<Vec<types::output::RowSet>> {
    let plan = translate::translate(empty_query_request());
    let types::output::QueryResponse(results) = execute::execute(plan);
    Json(results)
}

// utils

fn empty_query_request() -> types::input::QueryRequest {
    types::input::QueryRequest {
        table: "bamba".to_string(),
        query: empty_query(),
        arguments: HashMap::new(),
        table_relationships: HashMap::new(),
        variables: None,
    }
}

fn empty_query() -> types::input::Query {
    types::input::Query {
        aggregates: None,
        fields: None,
        limit: None,
        offset: None,
        order_by: None,
        predicate: None,
    }
}
