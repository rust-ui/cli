use serde_json;

use crate::command_init::fetch::Fetch;
use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::rust_ui_client::RustUIClient;

const LABEL: &str = "label";

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

pub struct UserInput {}

impl UserInput {
    pub async fn handle_index_styles() -> CliResult<()> {
        let styles_index_result = Fetch::from_url(&RustUIClient::styles_index_url()).await;

        // Parse the JSON string into Vec<serde_json::Value>
        if let Ok(styles_index) = styles_index_result {
            // Convert the String to a Vec<serde_json::Value>
            let vec_styles = serde_json::from_str::<Vec<serde_json::Value>>(&styles_index).map_err(|e| {
                CliError::malformed_registry(&format!("Failed to parse styles index JSON: {e}"))
            })?;
            ask_user_choose_style(vec_styles)?
        }
        Ok(())
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Ask user to choose a style (automatically selects Default)
fn ask_user_choose_style(vec_styles: Vec<serde_json::Value>) -> CliResult<()> {
    // Look for "Default" style and select it automatically
    for style in &vec_styles {
        if let Some(label) = style.get(LABEL)
            && label.as_str() == Some("Default")
        {
            println!("🎨 Automatically selecting Default style (no user input required)");
            println!("Selected style: {label}");
            return Ok(());
        }
    }

    // Fallback: if no "Default" found, use the first available style
    if let Some(first_style) = vec_styles.first()
        && let Some(label) = first_style.get(LABEL)
    {
        println!("🎨 No Default style found, automatically selecting first available style: {label}");
        return Ok(());
    }

    // If no styles available, return an error
    Err(CliError::validation("No styles available in registry"))
}
