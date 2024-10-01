use crate::common::helpers::{is_contained_in_lines, run_explain};

pub mod common;

#[tokio::test]
async fn select_by_pk() {
    let result = run_explain("select_by_pk").await;
    is_contained_in_lines(&["Clustered", "Index", "Seek"], &result.details.plan);
    insta::assert_snapshot!(result.details.query);
}

#[tokio::test]
async fn select_where_variable() {
    let result = run_explain("select_where_variable").await;
    is_contained_in_lines(&["Clustered", "Index", "Scan"], &result.details.plan);
    insta::assert_snapshot!(result.details.query);
}

#[tokio::test]
async fn select_where_name_nilike() {
    let result = run_explain("select_where_name_like").await;
    let keywords = &["Compute", "Scalar"];
    is_contained_in_lines(keywords, &result.details.plan);
    insta::assert_snapshot!(result.details.query);
}
