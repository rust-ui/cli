use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::process::Command;
use std::time::Duration;

use crate::constants::dependencies::INIT_DEPENDENCIES;
use crate::constants::others::SPINNER_UPDATE_DURATION;

///
/// UiConfig
///
///
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct UiConfig {
    pub tailwind_input_file: String,
    pub base_path_components: String,
}

#[allow(dead_code)]
impl UiConfig {
    pub fn new(tailwind_input_file: &str, base_path_components: &str) -> Self {
        UiConfig {
            tailwind_input_file: tailwind_input_file.to_string(),
            base_path_components: base_path_components.to_string(),
        }
    }

    pub fn try_reading_ui_config(toml_path: &str) -> Result<UiConfig, Box<dyn Error>> {
        let contents = fs::read_to_string(toml_path)?;
        let ui_config: UiConfig = toml::from_str(&contents)?;
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
    ///         tailwind_input_file: "style/tailwind.css".to_string(),
    ///         base_path_components: "src/components".to_string()
    ///     }
    /// );
    ///
    /// ```
    fn default() -> Self {
        UiConfig {
            tailwind_input_file: "style/tailwind.css".to_string(),
            base_path_components: "src/components".to_string(),
        }
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn add_init_dependencies() {
    for dep in INIT_DEPENDENCIES {
        let spinner = ProgressBar::new_spinner();
        spinner.set_message(format!("Adding and installing {} crate...", dep.name));
        spinner.enable_steady_tick(Duration::from_millis(SPINNER_UPDATE_DURATION));

        let mut args = vec!["add".to_owned(), dep.name.to_owned()];
        if !dep.features.is_empty() {
            args.push("--features".to_owned());
            args.push(dep.features.join(","));
        }
        let output = Command::new("cargo")
            .args(args)
            .output()
            .expect("🔸 Failed to add crate!");

        if output.status.success() {
            spinner.finish_with_message("✔️ Crates added successfully.");
        } else {
            spinner.finish_with_message(format!(
                "🔸 Error adding crates: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }
}
