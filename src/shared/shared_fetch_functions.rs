use crate::shared::cli_error::{CliError, CliResult};

// ADD + INIT
pub async fn shared_fetch_registry_return_json(url: &str) -> CliResult<serde_json::Value> {
    let response = reqwest::get(url).await.map_err(|_| {
        CliError::registry_request_failed()
    })?;
    
    let status = response.status();
    if !status.is_success() {
        return Err(CliError::registry_request_failed());
    }
    
    response.json::<serde_json::Value>().await.map_err(|_| {
        CliError::registry_invalid_format()
    })
}
