pub mod common;

#[tokio::test]
async fn get_schema() {
    let result = common::helpers::get_schema(common::helpers::create_router().await).await;
    insta::assert_json_snapshot!(result);
}
