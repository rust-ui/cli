// use dotenv::dotenv;
// use std::env;
use std::io::Write;

// use crate::constants::env::ENV;
use crate::{
    command_init::config::UiConfig,
    constants::{file_name::FileName, url::MyUrl},
    shared::cli_error::{CliError, CliResult},
};

use serde_json;

pub struct Registry {}

impl Registry {
    pub async fn fetch_index_content(url: &str) -> CliResult<String> {
        // Attempt to fetch the content from the URL
        let response = reqwest::get(url).await
            .map_err(|_| CliError::registry_request_failed())?;

        let status = response.status();
        if !status.is_success() {
            return Err(CliError::registry_request_failed());
        }

        let index_content_from_url = response.text().await
            .map_err(|_| CliError::registry_request_failed())?;

        // Check if the fetched content is empty
        if index_content_from_url.is_empty() {
            return Err(CliError::registry_request_failed());
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
    ) -> CliResult<RegistryComponent> {
        let base_url_styles_default = MyUrl::BASE_URL_STYLES_DEFAULT;
        let formatted_url_json = format!("{base_url_styles_default}/{component_name_json}.json");

        let response = reqwest::get(&formatted_url_json).await
            .map_err(|_| CliError::registry_request_failed())?;
        
        let status = response.status();
        if !status.is_success() {
            return Err(CliError::component_not_found(&component_name_json));
        }
        
        let json_content: serde_json::Value = response.json().await
            .map_err(|_| CliError::registry_invalid_format())?;

        let registry_json_path = json_content
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| CliError::registry_component_missing())?
            .to_string();
        let registry_json_content = json_content
            .get("files")
            .and_then(|v| v.get(0).and_then(|v| v.get("content").and_then(|v| v.as_str())))
            .ok_or_else(|| CliError::registry_component_missing())?
            .to_string();

        Ok(RegistryComponent {
            registry_json_path,
            registry_json_content,
            component_name_json,
        })
    }

    pub async fn then_write_to_file(self) -> CliResult<()> {
        let components_base_path = UiConfig::try_reading_ui_config(FileName::UI_CONFIG_TOML)?.base_path_components;
        let full_path_component = std::path::Path::new(&components_base_path).join(&self.registry_json_path);

        let full_path_component_without_name_rs = full_path_component
            .parent()
            .ok_or_else(|| CliError::file_operation("Failed to get parent directory"))?
            .to_str()
            .ok_or_else(|| CliError::file_operation("Failed to convert path to string"))?
            .to_string();

        write_component_name_in_mod_rs_if_not_exists(self.component_name_json, full_path_component_without_name_rs)?;

        let dir = full_path_component
            .parent()
            .ok_or_else(|| CliError::file_operation("Failed to get parent directory"))?;
        std::fs::create_dir_all(dir)
            .map_err(|_| CliError::directory_create_failed())?;

        std::fs::write(&full_path_component, self.registry_json_content)
            .map_err(|_| CliError::file_write_failed())?;

        Ok(())
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

fn write_component_name_in_mod_rs_if_not_exists(component_name: String, full_path_component_without_name_rs: String) -> CliResult<()> {
    let mod_rs_path = std::path::Path::new(&full_path_component_without_name_rs).join("mod.rs");

    // Create the directory if it doesn't exist
    let dir = mod_rs_path
        .parent()
        .ok_or_else(|| CliError::file_operation("Failed to get parent directory"))?;
    std::fs::create_dir_all(dir)
        .map_err(|_| CliError::directory_create_failed())?;

    // Check if the mod.rs file already exists
    let mut mod_rs_content = String::new();
    if mod_rs_path.exists() {
        mod_rs_content = std::fs::read_to_string(&mod_rs_path)
            .map_err(|_| CliError::file_read_failed())?;
    }

    // Check if the component already exists
    if mod_rs_content.contains(&component_name) {
        println!("Component {component_name} already exists in mod.rs");
        return Ok(()); // Exit the function if the component already exists
    }

    // Append the component name to mod.rs
    let mut mod_rs_file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&mod_rs_path)
        .map_err(|_| CliError::file_operation("Failed to open mod.rs file"))?;

    // Write the new component name
    writeln!(mod_rs_file, "pub mod {component_name};").map_err(|_| {
        CliError::file_write_failed()
    })?;
    Ok(())
}
