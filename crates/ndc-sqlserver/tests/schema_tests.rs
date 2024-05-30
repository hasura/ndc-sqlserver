pub mod common;

#[tokio::test]
async fn get_schema() {
    let result = common::helpers::get_schema().await;
    insta::assert_json_snapshot!(result);
}
