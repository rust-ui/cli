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

}
