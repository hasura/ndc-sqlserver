use std::process::ExitCode;

use ndc_sdk::default_main::default_main;
use ndc_sqlserver::connector::SQLServer;

#[tokio::main]
pub async fn main() -> ExitCode {
    let result = default_main::<SQLServer>().await;
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
