use std::process::ExitCode;

use ndc_sdk::default_main::default_main_with;
use ndc_sqlserver::connector::SQLServerSetup;
use ndc_sqlserver_configuration::environment::ProcessEnvironment;

#[tokio::main]
pub async fn main() -> ExitCode {
    let result = default_main_with(SQLServerSetup::new(ProcessEnvironment)).await;
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
