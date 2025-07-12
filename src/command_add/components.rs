use std::io::Write;

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use crate::error::{CliError, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MyComponent {
    pub name: String,
    pub registry_dependencies: Vec<String>,
    pub cargo_dependencies: Vec<String>,
    #[serde(rename = "type")]
    pub component_type: String,
    #[serde(rename = "parent_dir")]
    pub parent_dir: String,
}

#[derive(Debug)]
pub struct ResolvedComponent {
    pub component: MyComponent,
    pub resolved_registry_dependencies: HashSet<String>, // All dependencies including transitive ones
    pub resolved_cargo_dependencies: HashSet<String>, // All cargo dependencies including those from transitive dependencies
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub struct Components {}

impl Components {
    // TODO. Have instead all_resolved_parent_dirs instead of compomnents.
    pub fn create_components_mod_if_not_exists_with_pub_mods(user_config_path: String, parent_dirs: Vec<String>) -> Result<()> {
        let components_mod_path = std::path::Path::new(&user_config_path).join("mod.rs");

        // println!("Parent directories to add to components/mod.rs: {:?}", parent_dirs);

        // Create the directory if it doesn't exist
        let dir = components_mod_path
            .parent()
            .ok_or_else(|| CliError::file_operation(format!("Failed to get parent directory for {}", components_mod_path.display())))?;
        std::fs::create_dir_all(dir)
            .map_err(|e| CliError::file_operation(format!("Failed to create directory '{}': {}", dir.display(), e)))?;

        // Initialize mod_content
        let mut mod_content = String::new();

        // Check if the mod.rs file already exists
        if components_mod_path.exists() {
            mod_content = std::fs::read_to_string(&components_mod_path)
                .map_err(|e| CliError::file_operation(format!("Failed to read mod.rs file '{}': {}", components_mod_path.display(), e)))?;
        }

        // Create or open the mod.rs file for writing
        let mut mod_rs_file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&components_mod_path)
            .map_err(|e| CliError::file_operation(format!("Failed to open mod.rs file '{}': {}", components_mod_path.display(), e)))?;

        // Add each parent directory as a module if it doesn't already exist
        for parent_dir in parent_dirs {
            if !mod_content.contains(&format!("pub mod {parent_dir};")) {
                writeln!(mod_rs_file, "pub mod {parent_dir};").map_err(|e| {
                    CliError::file_operation(format!("Failed to write to mod.rs file '{}': {}", components_mod_path.display(), e))
                })?;
            }
        }
        Ok(())
    }

    pub fn register_components_in_application_entry(entry_file_path: &str) -> Result<()> {
        let file_content = std::fs::read_to_string(entry_file_path)
            .map_err(|e| CliError::file_operation(format!("Failed to read entry file '{entry_file_path}': {e}")))?;

        const MOD_COMPONENTS: &str = "mod components;";

        if file_content.contains(MOD_COMPONENTS) {
            return Ok(());
        }
        let mod_components_import = format!("{MOD_COMPONENTS}\n{file_content}");
        std::fs::write(entry_file_path, mod_components_import.as_bytes())
            .map_err(|e| CliError::file_operation(format!("Failed to write entry file '{entry_file_path}': {e}")))?;
        Ok(())
    }
}
