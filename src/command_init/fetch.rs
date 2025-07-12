use crate::shared::shared_fetch_functions::shared_fetch_registry_return_json;
use crate::error::{CliError, Result};

pub struct Fetch {}

impl Fetch {
    pub async fn from_url(url: &str) -> Result<String> {
        let json = shared_fetch_registry_return_json(url).await?;
        
        let pretty_json = serde_json::to_string_pretty(&json)
            .map_err(|e| CliError::malformed_registry(format!("Failed to convert to pretty JSON: {}", e)))?;

        Ok(pretty_json)
    }
}
