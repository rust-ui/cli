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
    pub fn create_components_mod_if_not_exists_with_pub_mods(user_config_path: String, parent_dirs: Vec<String>) {
        let components_mod_path = format!("{}/mod.rs", user_config_path);

        // println!("Parent directories to add to components/mod.rs: {:?}", parent_dirs);

        // Create the directory if it doesn't exist
        let dir = std::path::Path::new(&components_mod_path)
            .parent()
            .expect("Failed to get parent directory");
        std::fs::create_dir_all(dir).expect("Failed to create directories");

        // Initialize mod_content
        let mut mod_content = String::new();

        // Check if the mod.rs file already exists
        if std::path::Path::new(&components_mod_path).exists() {
            mod_content = std::fs::read_to_string(&components_mod_path).expect("Failed to read mod.rs");
        }

        // Create or open the mod.rs file for writing
        let mut mod_rs_file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(components_mod_path)
            .expect("Failed to open mod.rs");

        // Add each parent directory as a module if it doesn't already exist
        for parent_dir in parent_dirs {
            if !mod_content.contains(&format!("pub mod {};", parent_dir)) {
                writeln!(mod_rs_file, "pub mod {};", parent_dir).expect("🔸 Failed to write to mod.rs");
            }
        }
    }

        // TODO: Register components module in application entry point file
    pub fn register_components_in_application_entry(
        entry_file_path: &str
    ) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: read contents of file
        let file_content = std::fs::read_to_string(entry_file_path)?;

        // TODO: see if `mod components;` already exists
        if file_content.contains("mod components;") {
            return Ok(());
        }
        // TODO: if it's not, add it
        let new_contents = format!("{}\n{}", "mod components;", file_content);
        std::fs::write(entry_file_path, new_contents.as_bytes())?;
        Ok(())
    }
}
