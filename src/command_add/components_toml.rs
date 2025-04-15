use std::fs;

use crate::constants::file_name::FILE_NAME;
use colored::Colorize;

pub struct ComponentsToml {}



impl ComponentsToml {
    pub fn check_if_exists() -> Result<(), String> {
        if !std::path::Path::new(FILE_NAME::COMPONENTS_TOML).exists() {
            return Err(format!(
                "Components.toml not found Please run `{}` first.",
                "ui init".yellow()
            ));
        }
        Ok(())
    }

    pub fn try_extract_base_path_components_from_components_toml() -> Result<String, String> {
        let file_path = FILE_NAME::COMPONENTS_TOML;
        let contents = fs::read_to_string(file_path).unwrap();

        // TODO. There is this line:
        // base_path_components = "src/components"
        // Extract the value after the = sign and remove any quotes
        let parts: Vec<&str> = contents.split("base_path_components =").collect();
        if parts.len() > 1 {
            let value = parts[1].trim();
            if value.starts_with("\"") && value.ends_with("\"") {   
                return Ok(value.replace("\"", ""));
            }
        }
        Err("🔸 Error: 'base_path_components' not found in Components.toml. Please add it to your Components.toml.".to_string())
    }
    

}
