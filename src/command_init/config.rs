// use dotenv::dotenv;
use indicatif::ProgressBar;
// use serde::de::Error;
use serde::{Deserialize, Serialize};
use std::error::Error;
// use std::fmt::Result;
// use std::env;
use std::fs;
use std::process::Command;
use std::time::Duration;
use crate::constants::others::{CARGO_TOML_FILE, SPINNER_UPDATE_DURATION};


///
/// AppConfig
/// 
/// 
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct AppConfig {
    pub tailwind_input_file: String,
    pub base_path_components: String
}

#[allow(dead_code)]
impl AppConfig {

    pub fn new(tailwind_input_file: &str, base_path_components: &str) -> Self {
        AppConfig { 
            tailwind_input_file: tailwind_input_file.to_string(), 
            base_path_components: base_path_components.to_string() 
        }
    }

    pub fn try_reading_app_config(toml_path: &str) -> Result<AppConfig, Box<dyn Error>> {
        let contents = fs::read_to_string(toml_path)?;
        let app_config: AppConfig = toml::from_str(&contents)?;
        Ok(app_config)
    }

}

impl Default for AppConfig {
    ///
    /// Creates a default AppConfig
    /// 
    /// # Example
    /// ```
    /// let app_config = AppConfig::default();
    /// 
    /// assert_eq!(
    ///     app_config, 
    ///     AppConfig {
    ///         tailwind_input_file: "style/tailwind.css".to_string(),
    ///         base_path_components: "src/components".to_string()
    ///     }
    /// );
    /// 
    /// ```
    fn default() -> Self {
        AppConfig { 
            tailwind_input_file: "style/tailwind.css".to_string(), 
            base_path_components: "src/components".to_string()
        }
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/


#[allow(unused)]
fn add_tailwind_fuse_and_leptos_use() {
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Adding crates: rustui_merge and leptos-use");
    spinner.enable_steady_tick(Duration::from_millis(SPINNER_UPDATE_DURATION));

    let output = Command::new("cargo")
        .args([
            "add",
            "rustui_merge@0.3.0",
            "--features",
            "rustui_merge@0.3.0/variant,rustui_merge@0.3.0/debug",
            "leptos-use@0.13.5",
            "--features",
            "leptos-use@0.13.5/storage,leptos-use@0.13.5/docs,leptos-use@0.13.5/math",
        ])
        .output()
        .expect("🔸 Failed to execute cargo add command");

    if output.status.success() {
        spinner.finish_with_message("✔️ Crates added successfully.");
    } else {
        spinner.finish_with_message(format!(
            "🔸 Error adding crates: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
}

#[allow(unused)]
fn handle_adding_leptos_use_to_ssr_features() {
    match fs::read_to_string(CARGO_TOML_FILE) {
        Ok(mut contents) => {
            if let Some(start_pos) = contents.find("ssr = [") {
                // Find the position to insert the new features
                if let Some(end_pos) = contents[start_pos..].find(']') {
                    let insert_pos = start_pos + end_pos;
                    let new_features = r#"    "leptos-use/ssr",
    "leptos-use/axum",
"#;

                    // Check if the features are already present to avoid duplicates
                    if !contents[start_pos..insert_pos].contains("leptos-use/ssr") {
                        contents.insert_str(insert_pos, new_features);
                    }
                }
            } else {
                println!("'ssr' feature not found.");
            }

            // Write the modified contents back to the file
            if let Err(e) = fs::write(CARGO_TOML_FILE, &contents) {
                eprintln!("Error writing to file: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}