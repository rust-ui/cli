use std::io::Write;

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

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
    pub fn create_components_mod_if_not_exists_with_pub_mods(user_config_path: String, parent_dirs: Vec<String>) -> anyhow::Result<()> {
        let components_mod_path = format!("{user_config_path}/mod.rs");

        // println!("Parent directories to add to components/mod.rs: {:?}", parent_dirs);

        // Create the directory if it doesn't exist
        let dir = std::path::Path::new(&components_mod_path)
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Failed to get parent directory for {}", components_mod_path))?;
        std::fs::create_dir_all(dir)?;

        // Initialize mod_content
        let mut mod_content = String::new();

        // Check if the mod.rs file already exists
        if std::path::Path::new(&components_mod_path).exists() {
            mod_content = std::fs::read_to_string(&components_mod_path)?;
        }

        // Create or open the mod.rs file for writing
        let mut mod_rs_file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(components_mod_path)?;

        // Add each parent directory as a module if it doesn't already exist
        for parent_dir in parent_dirs {
            if !mod_content.contains(&format!("pub mod {parent_dir};")) {
                writeln!(mod_rs_file, "pub mod {parent_dir};")?;
            }
        }
        Ok(())
    }

    pub fn register_components_in_application_entry(entry_file_path: &str) -> anyhow::Result<()> {
        let file_content = std::fs::read_to_string(entry_file_path)?;

        const MOD_COMPONENTS: &str = "mod components;";

        if file_content.contains(MOD_COMPONENTS) {
            return Ok(());
        }
        let mod_components_import = format!("{MOD_COMPONENTS}\n{file_content}");
        std::fs::write(entry_file_path, mod_components_import.as_bytes())?;
        Ok(())
    }
}
