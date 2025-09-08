use crate::shared::cli_error::{CliError, CliResult};

pub struct RustUIClient;

impl RustUIClient {
    const BASE_URL: &str = "https://www.rust-ui.com/registry";

    // URL builders - centralized URL construction
    fn tree_url() -> String {
        format!("{}/tree.md", Self::BASE_URL)
    }

    fn component_url(component_name: &str) -> String {
        format!("{}/styles/default/{component_name}.md", Self::BASE_URL)
    }

    pub fn styles_index_url() -> String {
        format!("{}/styles/index.json", Self::BASE_URL)
    }

    // Consolidated HTTP fetch method
    async fn fetch_response(url: &str) -> CliResult<reqwest::Response> {
        let response = reqwest::get(url).await.map_err(|_| CliError::registry_request_failed())?;

        if !response.status().is_success() {
            return Err(CliError::registry_request_failed());
        }

        Ok(response)
    }

    // Public API methods
    pub async fn fetch_tree_md() -> CliResult<String> {
        let response = Self::fetch_response(&Self::tree_url()).await?;
        let content = response.text().await.map_err(|_| CliError::registry_request_failed())?;

        if content.is_empty() {
            return Err(CliError::registry_request_failed());
        }

        Ok(content)
    }

    pub async fn fetch_styles_default(component_name: &str) -> CliResult<String> {
        let response = Self::fetch_response(&Self::component_url(component_name)).await?;
        let markdown_content = response.text().await.map_err(|_| CliError::registry_request_failed())?;

        extract_rust_code_from_markdown(&markdown_content).ok_or_else(CliError::registry_component_missing)
    }

    pub async fn fetch_styles_index() -> CliResult<String> {
        let response = Self::fetch_response(&Self::styles_index_url()).await?;
        let json =
            response.json::<serde_json::Value>().await.map_err(|_| CliError::registry_invalid_format())?;

        serde_json::to_string_pretty(&json)
            .map_err(|err| CliError::malformed_registry(&format!("Failed to convert to pretty JSON: {err}")))
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// Separated utility function for markdown parsing
fn extract_rust_code_from_markdown(markdown: &str) -> Option<String> {
    let lines: Vec<&str> = markdown.lines().collect();
    let mut in_rust_block = false;
    let mut rust_code_lines = Vec::new();

    for line in lines {
        if line.trim() == "```rust" {
            in_rust_block = true;
            continue;
        }

        if in_rust_block && line.trim() == "```" {
            break;
        }

        if in_rust_block {
            rust_code_lines.push(line);
        }
    }

    if rust_code_lines.is_empty() { None } else { Some(rust_code_lines.join("\n")) }
}
