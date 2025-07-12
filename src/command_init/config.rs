use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

use crate::command_init::crates::INIT_CRATES;
use crate::shared::task_spinner::TaskSpinner;
use crate::shared::cli_error::{CliError, Result};

///
/// UiConfig
///
///
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct UiConfig {
    pub base_color: String,
    pub base_path_components: String,
    pub tailwind_input_file: String,
    pub tailwind_config_file: String,
}

#[allow(dead_code)]
impl UiConfig {
    pub fn new(
        tailwind_input_file: &str,
        base_path_components: &str,
        tailwind_config_file: &str,
        base_color: &str,
    ) -> Self {
        UiConfig {
            base_color: base_color.to_string(),
            base_path_components: base_path_components.to_string(),
            tailwind_config_file: tailwind_config_file.to_string(),
            tailwind_input_file: tailwind_input_file.to_string(),
        }
    }

    pub fn try_reading_ui_config(toml_path: &str) -> Result<UiConfig> {
        let contents = fs::read_to_string(toml_path)
            .map_err(|e| CliError::file_operation(&format!("Failed to read config file '{toml_path}': {e}")))?;
        let ui_config: UiConfig = toml::from_str(&contents)
            .map_err(|e| CliError::config(&format!("Failed to parse config file '{toml_path}': {e}")))?;
        Ok(ui_config)
    }
}

impl Default for UiConfig {
    ///
    /// Creates a default UiConfig
    ///
    /// # Example
    /// ```
    /// let ui_config = UiConfig::default();
    ///
    /// assert_eq!(
    ///     ui_config,
    ///     UiConfig {
    ///         base_color: "neutral".to_string(),
    ///         base_path_components: "src/components".to_string(),
    ///         tailwind_config_file: "tailwind.config.js".to_string(),
    ///         tailwind_input_file: "style/tailwind.css".to_string(),
    ///     }
    /// );
    ///
    /// ```
    fn default() -> Self {
        UiConfig {
            base_color: "neutral".to_string(),
            base_path_components: "src/components".to_string(),
            tailwind_config_file: "tailwind.config.js".to_string(),
            tailwind_input_file: "style/tailwind.css".to_string(),
        }
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn add_init_crates() -> Result<()> {
    // `crate` is a reserved keyword.
    for my_crate in INIT_CRATES {
        let spinner = TaskSpinner::new(&format!("Adding and installing {} crate...", my_crate.name));

        let mut args = vec!["add".to_owned(), my_crate.name.to_owned()];
        if !my_crate.features.is_empty() {
            args.push("--features".to_owned());
            args.push(my_crate.features.join(","));
        }
        let output = Command::new("cargo")
            .args(args)
            .output()
            .map_err(|e| CliError::cargo_operation(&format!("Failed to execute cargo add {}: {}", my_crate.name, e)))?;

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
