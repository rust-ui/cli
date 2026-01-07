use std::path::{Path, PathBuf};

use cargo_toml::{Dependency, Manifest};

use crate::shared::cli_error::{CliError, CliResult};

/// Information about the workspace and target crate
#[derive(Debug, Clone, PartialEq)]
pub struct WorkspaceInfo {
    /// Whether we're in a workspace
    pub is_workspace: bool,
    /// The workspace root directory (if in a workspace)
    pub workspace_root: Option<PathBuf>,
    /// The target crate name where components should be installed
    pub target_crate: Option<String>,
    /// The path to the target crate directory
    pub target_crate_path: Option<PathBuf>,
    /// The base path for components relative to current working directory
    pub components_base_path: String,
}

impl Default for WorkspaceInfo {
    fn default() -> Self {
        Self {
            is_workspace: false,
            workspace_root: None,
            target_crate: None,
            target_crate_path: None,
            components_base_path: "src/components".to_string(),
        }
    }
}

/// Analyzes the current directory to detect workspace structure and find the appropriate
/// crate for installing components.
pub fn analyze_workspace() -> CliResult<WorkspaceInfo> {
    let current_dir = std::env::current_dir()?;
    analyze_workspace_from_path(&current_dir)
}

/// Analyzes workspace from a specific path (useful for testing)
pub fn analyze_workspace_from_path(start_path: &Path) -> CliResult<WorkspaceInfo> {
    // First, check if we're in a workspace member directory
    let local_cargo_toml = start_path.join("Cargo.toml");

    if !local_cargo_toml.exists() {
        return Err(CliError::file_operation("Cargo.toml not found in current directory"));
    }

    let local_manifest = load_cargo_manifest(&local_cargo_toml)?
        .ok_or_else(|| CliError::file_operation("Failed to parse Cargo.toml"))?;

    // Check if this is a workspace root
    if local_manifest.workspace.is_some() {
        return analyze_from_workspace_root(start_path, &local_manifest);
    }

    // Check if we're in a workspace member by looking for workspace root
    if let Some(workspace_root) = find_workspace_root(start_path)? {
        return analyze_from_workspace_member(start_path, &workspace_root);
    }

    // Not in a workspace - simple single-crate project
    let has_leptos = check_leptos_in_manifest(&local_manifest);

    if !has_leptos {
        return Err(CliError::config("Leptos dependency not found in Cargo.toml"));
    }

    Ok(WorkspaceInfo {
        is_workspace: false,
        workspace_root: None,
        target_crate: local_manifest.package.as_ref().map(|p| p.name.clone()),
        target_crate_path: Some(start_path.to_path_buf()),
        components_base_path: "src/components".to_string(),
    })
}

/// Analyze when running from workspace root
fn analyze_from_workspace_root(workspace_root: &Path, manifest: &Manifest) -> CliResult<WorkspaceInfo> {
    let workspace =
        manifest.workspace.as_ref().ok_or_else(|| CliError::config("Expected workspace manifest"))?;

    // Find workspace member with Leptos
    let members = expand_workspace_members(workspace_root, &workspace.members)?;

    for member_path in &members {
        let member_cargo_toml = member_path.join("Cargo.toml");
        if let Some(member_manifest) = load_cargo_manifest(&member_cargo_toml)?
            && member_manifest.dependencies.contains_key("leptos")
        {
            let crate_name = member_manifest
                .package
                .as_ref()
                .map(|p| p.name.clone())
                .or_else(|| member_path.file_name().map(|n| n.to_string_lossy().to_string()))
                .unwrap_or_default();

            let relative_path = member_path.strip_prefix(workspace_root).unwrap_or(member_path);

            return Ok(WorkspaceInfo {
                is_workspace: true,
                workspace_root: Some(workspace_root.to_path_buf()),
                target_crate: Some(crate_name),
                target_crate_path: Some(member_path.clone()),
                components_base_path: format!("{}/src/components", relative_path.display()),
            });
        }
    }

    // Check workspace.dependencies for leptos
    if workspace.dependencies.contains_key("leptos") {
        // Leptos is in workspace deps, but we need to find which member uses it
        for member_path in &members {
            let member_cargo_toml = member_path.join("Cargo.toml");
            if let Some(member_manifest) = load_cargo_manifest(&member_cargo_toml)?
                && let Some(dep) = member_manifest.dependencies.get("leptos")
                && matches!(dep, Dependency::Inherited(_))
            {
                let crate_name = member_manifest
                    .package
                    .as_ref()
                    .map(|p| p.name.clone())
                    .or_else(|| member_path.file_name().map(|n| n.to_string_lossy().to_string()))
                    .unwrap_or_default();

                let relative_path = member_path.strip_prefix(workspace_root).unwrap_or(member_path);

                return Ok(WorkspaceInfo {
                    is_workspace: true,
                    workspace_root: Some(workspace_root.to_path_buf()),
                    target_crate: Some(crate_name),
                    target_crate_path: Some(member_path.clone()),
                    components_base_path: format!("{}/src/components", relative_path.display()),
                });
            }
        }
    }

    Err(CliError::config(
        "No workspace member with Leptos dependency found. Please run from a crate directory with Leptos installed.",
    ))
}

/// Analyze when running from a workspace member directory
fn analyze_from_workspace_member(member_path: &Path, workspace_root: &Path) -> CliResult<WorkspaceInfo> {
    let member_cargo_toml = member_path.join("Cargo.toml");
    let member_manifest = load_cargo_manifest(&member_cargo_toml)?
        .ok_or_else(|| CliError::file_operation("Failed to parse member Cargo.toml"))?;

    // Check if this member has leptos
    let has_leptos = check_leptos_in_manifest(&member_manifest);

    // Also check workspace.dependencies
    let workspace_cargo_toml = workspace_root.join("Cargo.toml");
    let workspace_has_leptos = if let Some(ws_manifest) = load_cargo_manifest(&workspace_cargo_toml)? {
        ws_manifest.workspace.as_ref().is_some_and(|ws| ws.dependencies.contains_key("leptos"))
    } else {
        false
    };

    if !has_leptos && !workspace_has_leptos {
        return Err(CliError::config("Leptos dependency not found in this crate or workspace"));
    }

    let crate_name = member_manifest
        .package
        .as_ref()
        .map(|p| p.name.clone())
        .or_else(|| member_path.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or_default();

    Ok(WorkspaceInfo {
        is_workspace: true,
        workspace_root: Some(workspace_root.to_path_buf()),
        target_crate: Some(crate_name),
        target_crate_path: Some(member_path.to_path_buf()),
        // When running from member, components go in local src/components
        components_base_path: "src/components".to_string(),
    })
}

/// Find workspace root by walking up the directory tree
fn find_workspace_root(start_path: &Path) -> CliResult<Option<PathBuf>> {
    let mut current = start_path.parent();

    while let Some(dir) = current {
        let cargo_toml = dir.join("Cargo.toml");
        if cargo_toml.exists()
            && let Some(manifest) = load_cargo_manifest(&cargo_toml)?
            && manifest.workspace.is_some()
        {
            return Ok(Some(dir.to_path_buf()));
        }
        current = dir.parent();
    }

    Ok(None)
}

/// Expand workspace member patterns (handles globs like "crates/*")
fn expand_workspace_members(workspace_root: &Path, members: &[String]) -> CliResult<Vec<PathBuf>> {
    let mut result = Vec::new();

    for member in members {
        if member.contains('*') {
            // Handle glob pattern
            let pattern = workspace_root.join(member);
            let pattern_str = pattern.to_string_lossy();

            if let Ok(paths) = glob::glob(&pattern_str) {
                for path in paths.flatten() {
                    if path.is_dir() && path.join("Cargo.toml").exists() {
                        result.push(path);
                    }
                }
            }
        } else {
            let member_path = workspace_root.join(member);
            if member_path.is_dir() && member_path.join("Cargo.toml").exists() {
                result.push(member_path);
            }
        }
    }

    Ok(result)
}

/// Check if manifest has leptos dependency
fn check_leptos_in_manifest(manifest: &Manifest) -> bool {
    manifest.dependencies.contains_key("leptos")
}

/// Checks if Leptos is installed as a dependency in Cargo.toml
pub fn check_leptos_dependency() -> CliResult<bool> {
    // Use the workspace analysis which handles workspaces properly
    match analyze_workspace() {
        Ok(_) => Ok(true), // If analysis succeeds, leptos was found
        Err(e) => {
            // Check if it's specifically a "leptos not found" error
            let err_msg = format!("{e}");
            if err_msg.contains("Leptos") { Ok(false) } else { Err(e) }
        }
    }
}

/* ========================================================== */
/*                     ✨ HELPERS ✨                          */
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

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;

    /// Helper to create a Cargo.toml with given content
    fn write_cargo_toml(dir: &Path, content: &str) {
        fs::write(dir.join("Cargo.toml"), content).unwrap();
    }

    /// Helper to create a minimal src directory
    fn create_src_dir(dir: &Path) {
        fs::create_dir_all(dir.join("src")).unwrap();
        fs::write(dir.join("src/lib.rs"), "").unwrap();
    }

    #[test]
    fn test_single_crate_with_leptos() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        write_cargo_toml(
            root,
            r#"
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = "0.7"
"#,
        );
        create_src_dir(root);

        let info = analyze_workspace_from_path(root).unwrap();

        assert!(!info.is_workspace);
        assert_eq!(info.target_crate, Some("my-app".to_string()));
        assert_eq!(info.components_base_path, "src/components");
    }

    #[test]
    fn test_single_crate_without_leptos() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        write_cargo_toml(
            root,
            r#"
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1"
"#,
        );
        create_src_dir(root);

        let result = analyze_workspace_from_path(root);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Leptos"));
    }

    #[test]
    fn test_workspace_with_leptos_member() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create workspace root
        write_cargo_toml(
            root,
            r#"
[workspace]
members = ["app", "server"]
"#,
        );

        // Create app member with leptos
        let app_dir = root.join("app");
        fs::create_dir_all(&app_dir).unwrap();
        write_cargo_toml(
            &app_dir,
            r#"
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = "0.7"
"#,
        );
        create_src_dir(&app_dir);

        // Create server member without leptos
        let server_dir = root.join("server");
        fs::create_dir_all(&server_dir).unwrap();
        write_cargo_toml(
            &server_dir,
            r#"
[package]
name = "my-server"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
"#,
        );
        create_src_dir(&server_dir);

        // Test from workspace root
        let info = analyze_workspace_from_path(root).unwrap();

        assert!(info.is_workspace);
        assert_eq!(info.target_crate, Some("my-app".to_string()));
        assert_eq!(info.components_base_path, "app/src/components");
    }

    #[test]
    fn test_workspace_from_member_directory() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create workspace root
        write_cargo_toml(
            root,
            r#"
[workspace]
members = ["frontend"]
"#,
        );

        // Create frontend member with leptos
        let frontend_dir = root.join("frontend");
        fs::create_dir_all(&frontend_dir).unwrap();
        write_cargo_toml(
            &frontend_dir,
            r#"
[package]
name = "frontend"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = "0.7"
"#,
        );
        create_src_dir(&frontend_dir);

        // Test from member directory
        let info = analyze_workspace_from_path(&frontend_dir).unwrap();

        assert!(info.is_workspace);
        assert_eq!(info.target_crate, Some("frontend".to_string()));
        // When running from member, path is relative to member
        assert_eq!(info.components_base_path, "src/components");
    }

    #[test]
    fn test_workspace_with_workspace_dependencies() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create workspace root with workspace.dependencies
        write_cargo_toml(
            root,
            r#"
[workspace]
members = ["app"]

[workspace.dependencies]
leptos = "0.7"
"#,
        );

        // Create app member that inherits leptos
        let app_dir = root.join("app");
        fs::create_dir_all(&app_dir).unwrap();
        write_cargo_toml(
            &app_dir,
            r#"
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos.workspace = true
"#,
        );
        create_src_dir(&app_dir);

        let info = analyze_workspace_from_path(root).unwrap();

        assert!(info.is_workspace);
        assert_eq!(info.target_crate, Some("my-app".to_string()));
    }

    #[test]
    fn test_workspace_no_leptos_member() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        write_cargo_toml(
            root,
            r#"
[workspace]
members = ["server"]
"#,
        );

        let server_dir = root.join("server");
        fs::create_dir_all(&server_dir).unwrap();
        write_cargo_toml(
            &server_dir,
            r#"
[package]
name = "server"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
"#,
        );
        create_src_dir(&server_dir);

        let result = analyze_workspace_from_path(root);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Leptos"));
    }

    #[test]
    fn test_no_cargo_toml() {
        let temp = TempDir::new().unwrap();
        let result = analyze_workspace_from_path(temp.path());

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cargo.toml"));
    }

    #[test]
    fn test_workspace_info_default() {
        let info = WorkspaceInfo::default();

        assert!(!info.is_workspace);
        assert!(info.workspace_root.is_none());
        assert!(info.target_crate.is_none());
        assert_eq!(info.components_base_path, "src/components");
    }
}
