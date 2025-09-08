use std::io::Write;

use crate::shared::cli_error::{CliError, CliResult};

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub struct Components {}

impl Components {
    // TODO. Have instead all_resolved_parent_dirs instead of compomnents.
    pub fn create_components_mod_if_not_exists_with_pub_mods(
        user_config_path: String,
        parent_dirs: Vec<String>,
    ) -> CliResult<()> {
        let components_mod_path = std::path::Path::new(&user_config_path).join("mod.rs");

        // println!("Parent directories to add to components/mod.rs: {:?}", parent_dirs);

        // Create the directory if it doesn't exist
        let dir = components_mod_path
            .parent()
            .ok_or_else(|| CliError::file_operation("Failed to get parent directory"))?;
        std::fs::create_dir_all(dir).map_err(|_| CliError::directory_create_failed())?;

        // Initialize mod_content
        let mut mod_content = String::new();

        // Check if the mod.rs file already exists
        if components_mod_path.exists() {
            mod_content =
                std::fs::read_to_string(&components_mod_path).map_err(|_| CliError::file_read_failed())?;
        }

        // Create or open the mod.rs file for writing
        let mut mod_rs_file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&components_mod_path)
            .map_err(|_| CliError::file_operation("Failed to open mod.rs file"))?;

        // Add each parent directory as a module if it doesn't already exist
        for parent_dir in parent_dirs {
            if !mod_content.contains(&format!("pub mod {parent_dir};")) {
                writeln!(mod_rs_file, "pub mod {parent_dir};").map_err(|_| CliError::file_write_failed())?;
            }
        }
        Ok(())
    }

    pub fn register_components_in_application_entry(entry_file_path: &str) -> CliResult<()> {
        let file_content =
            std::fs::read_to_string(entry_file_path).map_err(|_| CliError::file_read_failed())?;

        const MOD_COMPONENTS: &str = "mod components;";

        if file_content.contains(MOD_COMPONENTS) {
            return Ok(());
        }
        let mod_components_import = format!("{MOD_COMPONENTS}\n{file_content}");
        std::fs::write(entry_file_path, mod_components_import.as_bytes())
            .map_err(|_| CliError::file_write_failed())?;
        Ok(())
    }
}
