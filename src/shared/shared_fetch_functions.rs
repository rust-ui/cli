use crate::error::{CliError, Result};

// ADD + INIT
pub async fn shared_fetch_registry_return_json(url: &str) -> Result<serde_json::Value> {
    let response = reqwest::get(url).await.map_err(|e| {
        CliError::registry_fetch(format!("Failed to fetch from {url}: {e}"))
    })?;
    
    let status = response.status();
    if !status.is_success() {
        return Err(CliError::registry_fetch(format!(
            "Server returned status {}: {}",
            status.as_u16(),
            status.canonical_reason().unwrap_or("Unknown error")
        )));
    }
    
    response.json::<serde_json::Value>().await.map_err(|e| {
        CliError::registry_fetch(format!("Failed to parse JSON response: {e}"))
    })
}
