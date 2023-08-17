use ndc_hub::default_main::default_main;
use ndc_sqlserver::connector::SQLServer;

#[tokio::main]
pub async fn main() {
    default_main::<SQLServer>().await.unwrap()
}
