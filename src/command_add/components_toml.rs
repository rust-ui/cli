use std::fs;

use crate::constants::file_name::FILE_NAME;
use colored::Colorize;

pub struct ComponentsToml {}

impl ComponentsToml {
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
        Err(
            "🔸 Error: 'base_path_components' not found in Components.toml. Please add it to your Components.toml."
                .to_string(),
        )
    }

    #[allow(non_snake_case)]
    pub fn get_base_path_from_Components_toml() -> Result<String, String> {
        // Read the Components.toml file
        let config_str = match std::fs::read_to_string("Components.toml") {
            Ok(content) => content,
            Err(e) => {
                println!("Error reading Components.toml: {}", e);
                return Ok("components".to_string()); // Default to "components"
            }
        };

        let mut base_path = "components".to_string(); // Default value

        // Split the lines and find the base_path
        for line in config_str.lines() {
            if line.starts_with("path = ") {
                // Extract the path value
                let path_value = line.split('=').nth(1).unwrap_or("").trim().trim_matches('"');
                base_path = path_value.to_string();
                break; // Exit the loop once we find the path
            }
        }

        Ok(base_path)
    }
}
