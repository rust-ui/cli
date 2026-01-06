use std::fs;
use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::command_init::crates::INIT_CRATES;
use crate::command_init::workspace_utils::{analyze_workspace, check_leptos_dependency};
use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::task_spinner::TaskSpinner;

///
/// UiConfig - Minimal configuration stored in ui_config.toml
/// Workspace detection is done dynamically via analyze_workspace()
///
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct UiConfig {
    pub base_color: String,
    pub base_path_components: String,
    pub tailwind_input_file: String,
}

impl UiConfig {
    pub fn try_reading_ui_config(toml_path: &str) -> CliResult<UiConfig> {
        if !Path::new(toml_path).exists() {
            return Err(CliError::project_not_initialized());
        }
        let contents = fs::read_to_string(toml_path)?;
        let ui_config: UiConfig = toml::from_str(&contents)?;
        Ok(ui_config)
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        // Detect workspace and set appropriate component path
        let base_path_components = match analyze_workspace() {
            Ok(info) => info.components_base_path,
            Err(_) => "src/components".to_string(),
        };

        UiConfig {
            base_color: "neutral".to_string(),
            base_path_components,
            tailwind_input_file: "style/tailwind.css".to_string(),
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

pub async fn add_init_crates() -> CliResult<()> {
    // Detect workspace dynamically to determine if we need --package flag
    let workspace_info = analyze_workspace().ok();

    for my_crate in INIT_CRATES {
        // Skip leptos if it's already installed to preserve user's existing configuration
        if my_crate.name == "leptos" && check_leptos_dependency()? {
            continue;
        }

        let spinner = TaskSpinner::new(&format!("Adding and installing {} crate...", my_crate.name));

        let mut args = vec!["add".to_owned(), my_crate.name.to_owned()];

        // Add --package flag if we're in a workspace with a target crate
        if let Some(ref info) = workspace_info {
            if info.is_workspace {
                if let Some(ref crate_name) = info.target_crate {
                    args.push("--package".to_owned());
                    args.push(crate_name.clone());
                }
            }
        }

        if let Some(features) = my_crate.features
            && !features.is_empty()
        {
            args.push("--features".to_owned());
            args.push(features.join(","));
        }

        let output = Command::new("cargo").args(&args).output().map_err(|e| {
            CliError::cargo_operation(&format!("Failed to execute cargo add {}: {e}", my_crate.name))
        })?;

        if output.status.success() {
            spinner.finish_success("Crates added successfully.");
        } else {
            return Err(CliError::cargo_operation(&format!(
                "Failed to add crate '{}': {}",
                my_crate.name,
                String::from_utf8_lossy(&output.stderr)
            )));
        }
    }
    Ok(())
}
