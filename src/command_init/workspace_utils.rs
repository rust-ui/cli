use std::path::Path;

use cargo_toml::Manifest;

use crate::shared::cli_error::{CliError, CliResult};

/// Detects if the current directory is part of a Rust workspace
pub fn detect_workspace() -> CliResult<bool> {
    let cargo_toml_path = Path::new("Cargo.toml");
    
    let manifest = load_cargo_manifest(cargo_toml_path)?;
    
    // Check if the manifest has a workspace section
    Ok(manifest.map_or(false, |m| m.workspace.is_some()))
}

/// Gets the appropriate base path for components based on workspace detection
pub fn get_component_base_path(is_workspace: bool) -> String {
    if is_workspace {
        // In a workspace, components might be in a specific workspace member
        // For now, we'll use the same default but this could be enhanced
        "src/components".to_string()
    } else {
        "src/components".to_string()
    }
}

/// Checks if Leptos is installed as a dependency in Cargo.toml
pub fn check_leptos_dependency() -> CliResult<bool> {
    check_leptos_dependency_in_path(".")
}

/// Helper function to check leptos dependency in a specific path (useful for testing)
fn check_leptos_dependency_in_path(dir_path: &str) -> CliResult<bool> {
    let cargo_toml_path = Path::new(dir_path).join("Cargo.toml");

    let manifest = load_cargo_manifest(&cargo_toml_path)?;
    
    let Some(manifest) = manifest else {
        return Err(CliError::file_operation("Cargo.toml not found in current directory"));
    };

    // Check in [dependencies] section
    if manifest.dependencies.contains_key("leptos") {
        return Ok(true);
    }

    // Check in [workspace.dependencies] section for workspaces
    if let Some(workspace) = manifest.workspace {
        if workspace.dependencies.contains_key("leptos") {
            return Ok(true);
        }
    }

    Ok(false)
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Helper function to load a Cargo.toml manifest from a path
fn load_cargo_manifest(cargo_toml_path: &Path) -> CliResult<Option<Manifest>> {
    if !cargo_toml_path.exists() {
        return Ok(None);
    }

    // Try to load with full workspace resolution first
    match Manifest::from_path(cargo_toml_path) {
        Ok(manifest) => Ok(Some(manifest)),
        Err(_) => {
            // If workspace resolution fails (e.g., in tests), try parsing without workspace resolution  
            let contents = std::fs::read_to_string(cargo_toml_path)?;
            let manifest = Manifest::from_slice(contents.as_bytes())?;
            Ok(Some(manifest))
        }
    }
}

