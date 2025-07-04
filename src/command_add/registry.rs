// use dotenv::dotenv;
// use std::env;
use std::io::Write;

// use crate::constants::env::ENV;
use crate::{
    command_init::config::UiConfig,
    constants::{file_name::FILE_NAME, url::MyUrl},
};

use serde_json;

pub struct Registry {}

impl Registry {
    pub async fn fetch_index_content(url: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Attempt to fetch the content from the URL
        let response = reqwest::get(url).await;

        // Check if the request was successful
        let index_content_from_url = match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.text().await?
                } else {
                    let error_message = format!("🔸 Failed to fetch data: Server returned status {}", resp.status());
                    println!("{error_message}"); // Print the error message
                    return Err(error_message.into());
                }
            }
            Err(err) => {
                let error_message = format!("🔸 Failed to fetch data: {err}");
                println!("{error_message}"); // Print the error message
                return Err(error_message.into());
            }
        };

        // Check if the fetched content is empty
        if index_content_from_url.is_empty() {
            let error_message = "🔸 Failed to fetch data: The server returned an empty response.";
            println!("{error_message}"); // Print the error message
            return Err(error_message.into());
        }

        Ok(index_content_from_url)
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub struct RegistryComponent {
    pub registry_json_path: String,
    pub registry_json_content: String,
    pub component_name_json: String,
}

impl RegistryComponent {
    pub async fn fetch_from_registry(
        component_name_json: String,
    ) -> Result<RegistryComponent, Box<dyn std::error::Error>> {
        let base_url_styles_default = MyUrl::BASE_URL_STYLES_DEFAULT;
        let formatted_url_json = format!("{base_url_styles_default}/{component_name_json}.json");

        let response = reqwest::get(&formatted_url_json).await?;
        let json_content: serde_json::Value = response.json().await?;

        let registry_json_path = json_content
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or("Path not found")?
            .to_string();
        let registry_json_content = json_content
            .get("files")
            .and_then(|v| v.get(0).and_then(|v| v.get("content").and_then(|v| v.as_str())))
            .ok_or("Content not found")?
            .to_string();

        Ok(RegistryComponent {
            registry_json_path,
            registry_json_content,
            component_name_json,
        })
    }

    pub async fn then_write_to_file(self) -> Result<(), Box<dyn std::error::Error>> {
        let components_base_path = UiConfig::try_reading_ui_config(FILE_NAME::UI_CONFIG_TOML)?.base_path_components;
        let full_path_component = format!("{}/{}", components_base_path, self.registry_json_path);

        let full_path_component_without_name_rs = std::path::Path::new(&full_path_component)
            .parent()
            .ok_or("Failed to get parent directory")?
            .to_str()
            .ok_or("Failed to convert path to string")?
            .to_string();

        write_component_name_in_mod_rs_if_not_exists(self.component_name_json, full_path_component_without_name_rs);

        let dir = std::path::Path::new(&full_path_component)
            .parent()
            .ok_or("Failed to get parent directory")?;
        std::fs::create_dir_all(dir)?;

        std::fs::write(full_path_component, self.registry_json_content)?;

        Ok(())
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

fn write_component_name_in_mod_rs_if_not_exists(component_name: String, full_path_component_without_name_rs: String) {
    let mod_rs_path = format!("{full_path_component_without_name_rs}/mod.rs");

    // Create the directory if it doesn't exist
    let dir = std::path::Path::new(&mod_rs_path)
        .parent()
        .expect("Failed to get parent directory");
    std::fs::create_dir_all(dir).expect("Failed to create directories");

    // Check if the mod.rs file already exists
    let mut mod_rs_content = String::new();
    if std::path::Path::new(&mod_rs_path).exists() {
        mod_rs_content = std::fs::read_to_string(&mod_rs_path).expect("Failed to read mod.rs");
    }

    // Check if the component already exists
    if mod_rs_content.contains(&component_name) {
        println!("Component {component_name} already exists in mod.rs");
        return; // Exit the function if the component already exists
    }

    // Append the component name to mod.rs
    let mut mod_rs_file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(mod_rs_path)
        .expect("Failed to open mod.rs");

    // Write the new component name
    writeln!(mod_rs_file, "pub mod {component_name};").expect("Failed to write to mod.rs");
}
