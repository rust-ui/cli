// use dotenv::dotenv;
// use std::env;
use std::io::Write;

// use crate::constants::env::ENV;
use crate::{
    command_init::config::UiConfig,
    constants::{file_name::FileName, url::MyUrl},
    shared::cli_error::{CliError, CliResult},
};


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
    pub registry_md_path: String,
    pub registry_md_content: String,
    pub component_name: String,
}

impl RegistryComponent {
    pub async fn fetch_from_registry(
        component_name: String,
    ) -> CliResult<RegistryComponent> {
        let base_url_styles_default = MyUrl::BASE_URL_STYLES_DEFAULT;
        let formatted_url_md = format!("{base_url_styles_default}/{component_name}.md");

        let response = reqwest::get(&formatted_url_md).await
            .map_err(|_| CliError::registry_request_failed())?;
        
        let status = response.status();
        if !status.is_success() {
            return Err(CliError::component_not_found(&component_name));
        }
        
        let markdown_content = response.text().await
            .map_err(|_| CliError::registry_request_failed())?;

        let registry_md_content = extract_rust_code_from_markdown(&markdown_content)
            .ok_or_else(CliError::registry_component_missing)?;
        
        let registry_md_path = format!("ui/{}.rs", component_name);

        Ok(RegistryComponent {
            registry_md_path,
            registry_md_content,
            component_name,
        })
    }

    pub async fn then_write_to_file(self) -> CliResult<()> {
        let components_base_path = UiConfig::try_reading_ui_config(FileName::UI_CONFIG_TOML)?.base_path_components;
        let full_path_component = std::path::Path::new(&components_base_path).join(&self.registry_md_path);

        let full_path_component_without_name_rs = full_path_component
            .parent()
            .ok_or_else(|| CliError::file_operation("Failed to get parent directory"))?
            .to_str()
            .ok_or_else(|| CliError::file_operation("Failed to convert path to string"))?
            .to_string();

        write_component_name_in_mod_rs_if_not_exists(self.component_name, full_path_component_without_name_rs)?;

        let dir = full_path_component
            .parent()
            .ok_or_else(|| CliError::file_operation("Failed to get parent directory"))?;
        std::fs::create_dir_all(dir)
            .map_err(|_| CliError::directory_create_failed())?;

        std::fs::write(&full_path_component, self.registry_md_content)
            .map_err(|_| CliError::file_write_failed())?;

        Ok(())
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

fn extract_rust_code_from_markdown(markdown: &str) -> Option<String> {
    let lines: Vec<&str> = markdown.lines().collect();
    let mut in_rust_block = false;
    let mut rust_code_lines = Vec::new();
    
    for line in lines {
        if line.trim() == "```rust" {
            in_rust_block = true;
            continue;
        }
        
        if in_rust_block && line.trim() == "```" {
            break;
        }
        
        if in_rust_block {
            rust_code_lines.push(line);
        }
    }
    
    if rust_code_lines.is_empty() {
        None
    } else {
        Some(rust_code_lines.join("\n"))
    }
}

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
