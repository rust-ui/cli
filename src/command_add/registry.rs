use std::io::Write;

use super::component_type::ComponentType;
use crate::command_init::config::UiConfig;
use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::rust_ui_client::RustUIClient;

const UI_CONFIG_TOML: &str = "ui_config.toml";

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub struct RegistryComponent {
    pub registry_md_path: String,
    pub registry_md_content: String,
    pub component_name: String,
}

impl RegistryComponent {
    pub async fn fetch_from_registry(component_name: String) -> CliResult<RegistryComponent> {
        let registry_md_content = RustUIClient::fetch_styles_default(&component_name).await?;
        let component_type = ComponentType::from_component_name(&component_name);
        let registry_md_path = format!("{}/{}.rs", component_type.to_path(), component_name);

        Ok(RegistryComponent { registry_md_path, registry_md_content, component_name })
    }

    pub async fn then_write_to_file(self) -> CliResult<()> {
        let components_base_path = UiConfig::try_reading_ui_config(UI_CONFIG_TOML)?.base_path_components;
        let full_path_component = std::path::Path::new(&components_base_path).join(&self.registry_md_path);

        let full_path_component_without_name_rs = full_path_component
            .parent()
            .ok_or_else(|| CliError::file_operation("Failed to get parent directory"))?
            .to_str()
            .ok_or_else(|| CliError::file_operation("Failed to convert path to string"))?
            .to_string();

        write_component_name_in_mod_rs_if_not_exists(
            self.component_name,
            full_path_component_without_name_rs,
        )?;

        let dir = full_path_component
            .parent()
            .ok_or_else(|| CliError::file_operation("Failed to get parent directory"))?;
        std::fs::create_dir_all(dir).map_err(|_| CliError::directory_create_failed())?;

        std::fs::write(&full_path_component, self.registry_md_content)
            .map_err(|_| CliError::file_write_failed())?;

        Ok(())
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

fn write_component_name_in_mod_rs_if_not_exists(
    component_name: String,
    full_path_component_without_name_rs: String,
) -> CliResult<()> {
    let mod_rs_path = std::path::Path::new(&full_path_component_without_name_rs).join("mod.rs");

    // Create the directory if it doesn't exist
    let dir =
        mod_rs_path.parent().ok_or_else(|| CliError::file_operation("Failed to get parent directory"))?;
    std::fs::create_dir_all(dir).map_err(|_| CliError::directory_create_failed())?;

    // Check if the mod.rs file already exists
    let mut mod_rs_content = String::new();
    if mod_rs_path.exists() {
        mod_rs_content = std::fs::read_to_string(&mod_rs_path).map_err(|_| CliError::file_read_failed())?;
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
    writeln!(mod_rs_file, "pub mod {component_name};").map_err(|_| CliError::file_write_failed())?;
    Ok(())
}
