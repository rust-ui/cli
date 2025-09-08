use std::fs;
use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::command_init::crates::INIT_CRATES;
use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::task_spinner::TaskSpinner;

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

impl UiConfig {
    pub fn try_reading_ui_config(toml_path: &str) -> CliResult<UiConfig> {
        let contents = fs::read_to_string(toml_path).map_err(|e| {
            CliError::file_operation(&format!("Failed to read config file '{toml_path}': {e}"))
        })?;
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

pub async fn add_init_crates() -> CliResult<()> {
    // `crate` is a reserved keyword.
    for my_crate in INIT_CRATES {
        let spinner = TaskSpinner::new(&format!("Adding and installing {} crate...", my_crate.name));

        let mut args = vec!["add".to_owned(), my_crate.name.to_owned()];
        if let Some(features) = my_crate.features
            && !features.is_empty()
        {
            args.push("--features".to_owned());
            args.push(features.join(","));
        }
        let output = Command::new("cargo").args(args).output().map_err(|e| {
            CliError::cargo_operation(&format!("Failed to execute cargo add {}: {}", my_crate.name, e))
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
