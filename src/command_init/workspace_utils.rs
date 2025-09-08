use std::fs;
use std::path::Path;

use crate::shared::cli_error::{CliError, CliResult};

/// Detects if the current directory is part of a Rust workspace
pub fn detect_workspace() -> CliResult<bool> {
    let cargo_toml_path = Path::new("Cargo.toml");
    
    if !cargo_toml_path.exists() {
        return Ok(false);
    }
    
    let contents = fs::read_to_string(cargo_toml_path)
        .map_err(|e| CliError::file_operation(&format!("Failed to read Cargo.toml: {e}")))?;
    
    // Check if the Cargo.toml contains a [workspace] section
    Ok(contents.contains("[workspace]"))
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
    
    if !cargo_toml_path.exists() {
        return Err(CliError::file_operation("Cargo.toml not found in current directory"));
    }
    
    let contents = fs::read_to_string(&cargo_toml_path)
        .map_err(|e| CliError::file_operation(&format!("Failed to read Cargo.toml: {e}")))?;
    
    let toml_value: toml::Value = toml::from_str(&contents)
        .map_err(|e| CliError::config(&format!("Failed to parse Cargo.toml: {e}")))?;
    
    // Check in [dependencies] section
    if let Some(deps) = toml_value.get("dependencies").and_then(|d| d.as_table()) {
        if deps.contains_key("leptos") {
            return Ok(true);
        }
    }
    
    // Check in [workspace.dependencies] section for workspaces
    if let Some(workspace) = toml_value.get("workspace") {
        if let Some(deps) = workspace.get("dependencies").and_then(|d| d.as_table()) {
            if deps.contains_key("leptos") {
                return Ok(true);
            }
        }
    }
    
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    
    #[test]
    fn test_detect_workspace_with_workspace_toml() {
        let temp_dir = TempDir::new().unwrap();
        let cargo_toml_path = temp_dir.path().join("Cargo.toml");
        
        fs::write(&cargo_toml_path, r#"
[workspace]
members = ["app", "lib"]

[package]
name = "test"
version = "0.1.0"
"#).unwrap();
        
        std::env::set_current_dir(temp_dir.path()).unwrap();
        assert!(detect_workspace().unwrap());
    }
    
    #[test]
    fn test_detect_workspace_without_workspace_toml() {
        let temp_dir = TempDir::new().unwrap();
        let cargo_toml_path = temp_dir.path().join("Cargo.toml");
        
        fs::write(&cargo_toml_path, r#"
[package]
name = "test"
version = "0.1.0"
"#).unwrap();
        
        std::env::set_current_dir(temp_dir.path()).unwrap();
        assert!(!detect_workspace().unwrap());
    }
    
    #[test]
    fn test_detect_workspace_no_cargo_toml() {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        assert!(!detect_workspace().unwrap());
    }
    
    #[test]
    fn test_check_leptos_dependency_with_leptos() {
        let temp_dir = TempDir::new().unwrap();
        let cargo_toml_path = temp_dir.path().join("Cargo.toml");
        
        fs::write(&cargo_toml_path, r#"
[package]
name = "test"
version = "0.1.0"

[dependencies]
leptos = { version = "0.6", features = ["csr", "hydrate", "ssr"] }
"#).unwrap();
        
        assert!(check_leptos_dependency_in_path(temp_dir.path().to_str().unwrap()).unwrap());
    }
    
    #[test]
    fn test_check_leptos_dependency_without_leptos() {
        let temp_dir = TempDir::new().unwrap();
        let cargo_toml_path = temp_dir.path().join("Cargo.toml");
        
        fs::write(&cargo_toml_path, r#"
[package]
name = "test"
version = "0.1.0"

[dependencies]
serde = "1.0"
"#).unwrap();
        
        assert!(!check_leptos_dependency_in_path(temp_dir.path().to_str().unwrap()).unwrap());
    }
    
    #[test]
    fn test_check_leptos_dependency_workspace_deps() {
        let temp_dir = TempDir::new().unwrap();
        let cargo_toml_path = temp_dir.path().join("Cargo.toml");
        
        fs::write(&cargo_toml_path, r#"
[workspace]
members = ["app", "lib"]

[workspace.dependencies]
leptos = { version = "0.6", features = ["csr", "hydrate", "ssr"] }

[package]
name = "test"
version = "0.1.0"
"#).unwrap();
        
        assert!(check_leptos_dependency_in_path(temp_dir.path().to_str().unwrap()).unwrap());
    }
}