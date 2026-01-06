use std::collections::HashSet;
use std::fs;
use std::path::Path;

use cargo_toml::Manifest;
use toml_edit::{DocumentMut, Item, Value};

use crate::command_init::workspace_utils::{WorkspaceInfo, analyze_workspace};
use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::task_spinner::TaskSpinner;

pub fn process_cargo_deps(cargo_deps: &[String]) -> CliResult<()> {
    let spinner = TaskSpinner::new("Checking dependencies...");

    // Detect workspace to determine how to add dependencies
    let workspace_info = analyze_workspace().ok();

    // Get existing dependencies from the target Cargo.toml
    let existing_deps = get_existing_dependencies(&workspace_info)?;

    // Filter out dependencies that already exist
    let (new_deps, existing_deps_found): (Vec<_>, Vec<_>) =
        cargo_deps.iter().partition(|dep| !existing_deps.contains(*dep));

    if !existing_deps_found.is_empty() {
        let existing_str = existing_deps_found.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
        spinner.set_message(&format!("⏭️  Skipping existing dependencies: [{existing_str}]"));
    }

    if new_deps.is_empty() {
        spinner.finish_with_message("All dependencies already exist in Cargo.toml");
        return Ok(());
    }

    spinner.set_message("Adding new crates to Cargo.toml...");

    // Check if we should use workspace dependencies
    let use_workspace_deps = should_use_workspace_deps(&workspace_info);

    let mut added_deps = Vec::new();

    for dep in &new_deps {
        spinner.set_message(&format!("📦 Adding crate: {dep}"));

        let result = if use_workspace_deps {
            add_workspace_dependency(dep, workspace_info.as_ref().unwrap())
        } else {
            add_dependency_with_cargo(dep, &workspace_info)
        };

        match result {
            Ok(()) => added_deps.push(dep.as_str()),
            Err(e) => return Err(e),
        }
    }

    let dependencies_str = added_deps.join(", ");
    let finish_message = format!("Successfully added to Cargo.toml: [{dependencies_str}] !");
    spinner.finish_success(&finish_message);

    Ok(())
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Check if we should use workspace dependencies pattern
fn should_use_workspace_deps(workspace_info: &Option<WorkspaceInfo>) -> bool {
    let Some(info) = workspace_info else {
        return false;
    };

    if !info.is_workspace {
        return false;
    }

    let Some(workspace_root) = &info.workspace_root else {
        return false;
    };

    // Check if workspace root has [workspace.dependencies] section
    let root_cargo_toml = workspace_root.join("Cargo.toml");
    if !root_cargo_toml.exists() {
        return false;
    }

    let Ok(contents) = fs::read_to_string(&root_cargo_toml) else {
        return false;
    };

    let Ok(doc) = contents.parse::<DocumentMut>() else {
        return false;
    };

    // Check if [workspace.dependencies] exists
    doc.get("workspace")
        .and_then(|w| w.get("dependencies"))
        .is_some()
}

/// Add dependency using workspace pattern:
/// 1. Add to [workspace.dependencies] in root Cargo.toml
/// 2. Add dep.workspace = true to member Cargo.toml
fn add_workspace_dependency(dep: &str, info: &WorkspaceInfo) -> CliResult<()> {
    let workspace_root = info.workspace_root.as_ref()
        .ok_or_else(|| CliError::cargo_operation("Workspace root not found"))?;

    let member_path = info.target_crate_path.as_ref()
        .ok_or_else(|| CliError::cargo_operation("Target crate path not found"))?;

    // First, get the latest version from crates.io
    let version = fetch_latest_version(dep)?;

    // Add to workspace root [workspace.dependencies]
    let root_cargo_toml = workspace_root.join("Cargo.toml");
    add_to_workspace_dependencies(&root_cargo_toml, dep, &version)?;

    // Add to member [dependencies] with workspace = true
    let member_cargo_toml = member_path.join("Cargo.toml");
    add_workspace_ref_to_member(&member_cargo_toml, dep)?;

    Ok(())
}

/// Add dependency to [workspace.dependencies] in root Cargo.toml
fn add_to_workspace_dependencies(cargo_toml_path: &Path, dep: &str, version: &str) -> CliResult<()> {
    let contents = fs::read_to_string(cargo_toml_path)?;
    let mut doc: DocumentMut = contents.parse()
        .map_err(|e| CliError::cargo_operation(&format!("Failed to parse Cargo.toml: {e}")))?;

    // Get or create [workspace.dependencies]
    let workspace = doc.entry("workspace")
        .or_insert(Item::Table(toml_edit::Table::new()));

    let workspace_table = workspace.as_table_mut()
        .ok_or_else(|| CliError::cargo_operation("[workspace] is not a table"))?;

    let deps = workspace_table.entry("dependencies")
        .or_insert(Item::Table(toml_edit::Table::new()));

    let deps_table = deps.as_table_mut()
        .ok_or_else(|| CliError::cargo_operation("[workspace.dependencies] is not a table"))?;

    // Check if already exists
    if deps_table.contains_key(dep) {
        return Ok(());
    }

    // Add the dependency with version
    deps_table.insert(dep, Item::Value(Value::String(toml_edit::Formatted::new(version.to_string()))));

    // Write back
    fs::write(cargo_toml_path, doc.to_string())?;

    Ok(())
}

/// Add dep.workspace = true to member's [dependencies]
fn add_workspace_ref_to_member(cargo_toml_path: &Path, dep: &str) -> CliResult<()> {
    let contents = fs::read_to_string(cargo_toml_path)?;
    let mut doc: DocumentMut = contents.parse()
        .map_err(|e| CliError::cargo_operation(&format!("Failed to parse member Cargo.toml: {e}")))?;

    // Get or create [dependencies]
    let deps = doc.entry("dependencies")
        .or_insert(Item::Table(toml_edit::Table::new()));

    let deps_table = deps.as_table_mut()
        .ok_or_else(|| CliError::cargo_operation("[dependencies] is not a table"))?;

    // Check if already exists
    if deps_table.contains_key(dep) {
        return Ok(());
    }

    // Add dep.workspace = true using dotted key format
    let mut dep_table = toml_edit::Table::new();
    dep_table.set_dotted(true);
    dep_table.insert("workspace", Item::Value(Value::Boolean(toml_edit::Formatted::new(true))));
    deps_table.insert(dep, Item::Table(dep_table));

    // Write back
    fs::write(cargo_toml_path, doc.to_string())?;

    Ok(())
}

/// Fetch the latest version of a crate from crates.io
fn fetch_latest_version(crate_name: &str) -> CliResult<String> {
    // Use cargo search to get the latest version
    let output = std::process::Command::new("cargo")
        .args(["search", crate_name, "--limit", "1"])
        .output()
        .map_err(|_| CliError::cargo_operation("Failed to execute cargo search"))?;

    if !output.status.success() {
        return Err(CliError::cargo_operation(&format!(
            "Failed to search for crate '{crate_name}'"
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Parse output like: serde = "1.0.219"   # A generic serialization/deserialization framework
    for line in stdout.lines() {
        if line.starts_with(crate_name) {
            // Extract version from format: crate_name = "version"
            if let Some(version_part) = line.split('=').nth(1) {
                let version = version_part
                    .trim()
                    .trim_matches('"')
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim_matches('"');

                if !version.is_empty() {
                    return Ok(version.to_string());
                }
            }
        }
    }

    // Fallback: use "*" if we can't determine version
    Ok("*".to_string())
}

/// Fallback: use cargo add command
fn add_dependency_with_cargo(dep: &str, workspace_info: &Option<WorkspaceInfo>) -> CliResult<()> {
    let args = build_cargo_add_args(dep, workspace_info);

    let output = std::process::Command::new("cargo")
        .args(&args)
        .output()
        .map_err(|_| CliError::cargo_operation("Failed to execute cargo add"))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(CliError::cargo_operation(&format!("Failed to add dependency '{dep}': {stderr}")))
    }
}

/// Build cargo add arguments, adding --package flag for workspaces
fn build_cargo_add_args(dep: &str, workspace_info: &Option<WorkspaceInfo>) -> Vec<String> {
    let mut args = vec!["add".to_string(), dep.to_string()];

    if let Some(info) = workspace_info {
        if info.is_workspace {
            if let Some(crate_name) = &info.target_crate {
                args.push("--package".to_string());
                args.push(crate_name.clone());
            }
        }
    }

    args
}

/// Check if a crate is already in Cargo.toml dependencies
fn get_existing_dependencies(workspace_info: &Option<WorkspaceInfo>) -> CliResult<HashSet<String>> {
    // Determine which Cargo.toml to check
    let cargo_toml_path = if let Some(info) = workspace_info {
        if let Some(crate_path) = &info.target_crate_path {
            crate_path.join("Cargo.toml")
        } else {
            Path::new("Cargo.toml").to_path_buf()
        }
    } else {
        Path::new("Cargo.toml").to_path_buf()
    };

    if !cargo_toml_path.exists() {
        return Ok(HashSet::new());
    }

    // Read the file directly to avoid workspace resolution issues
    let contents = fs::read_to_string(&cargo_toml_path)?;
    let manifest = Manifest::from_slice(contents.as_bytes())?;

    let mut existing_deps = HashSet::new();

    // Check [dependencies] section
    for dep_name in manifest.dependencies.keys() {
        existing_deps.insert(dep_name.clone());
    }

    // Check [dev-dependencies] section
    for dep_name in manifest.dev_dependencies.keys() {
        existing_deps.insert(dep_name.clone());
    }

    Ok(existing_deps)
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_build_cargo_add_args_no_workspace() {
        let args = build_cargo_add_args("serde", &None);
        assert_eq!(args, vec!["add", "serde"]);
    }

    #[test]
    fn test_build_cargo_add_args_single_crate() {
        let info = WorkspaceInfo {
            is_workspace: false,
            workspace_root: None,
            target_crate: Some("my-app".to_string()),
            target_crate_path: None,
            components_base_path: "src/components".to_string(),
        };

        let args = build_cargo_add_args("serde", &Some(info));
        assert_eq!(args, vec!["add", "serde"]);
    }

    #[test]
    fn test_build_cargo_add_args_workspace_with_target() {
        let info = WorkspaceInfo {
            is_workspace: true,
            workspace_root: Some(PathBuf::from("/project")),
            target_crate: Some("frontend".to_string()),
            target_crate_path: Some(PathBuf::from("/project/frontend")),
            components_base_path: "frontend/src/components".to_string(),
        };

        let args = build_cargo_add_args("serde", &Some(info));
        assert_eq!(args, vec!["add", "serde", "--package", "frontend"]);
    }

    #[test]
    fn test_build_cargo_add_args_workspace_no_target() {
        let info = WorkspaceInfo {
            is_workspace: true,
            workspace_root: Some(PathBuf::from("/project")),
            target_crate: None,
            target_crate_path: None,
            components_base_path: "src/components".to_string(),
        };

        let args = build_cargo_add_args("serde", &Some(info));
        assert_eq!(args, vec!["add", "serde"]);
    }

    #[test]
    fn test_should_use_workspace_deps_no_workspace() {
        assert!(!should_use_workspace_deps(&None));
    }

    #[test]
    fn test_should_use_workspace_deps_not_workspace() {
        let info = WorkspaceInfo {
            is_workspace: false,
            workspace_root: None,
            target_crate: Some("app".to_string()),
            target_crate_path: None,
            components_base_path: "src/components".to_string(),
        };
        assert!(!should_use_workspace_deps(&Some(info)));
    }

    #[test]
    fn test_should_use_workspace_deps_with_workspace_dependencies() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create workspace Cargo.toml with [workspace.dependencies]
        fs::write(
            root.join("Cargo.toml"),
            r#"
[workspace]
members = ["app"]

[workspace.dependencies]
leptos = "0.7"
"#,
        ).unwrap();

        let info = WorkspaceInfo {
            is_workspace: true,
            workspace_root: Some(root.to_path_buf()),
            target_crate: Some("app".to_string()),
            target_crate_path: Some(root.join("app")),
            components_base_path: "app/src/components".to_string(),
        };

        assert!(should_use_workspace_deps(&Some(info)));
    }

    #[test]
    fn test_should_use_workspace_deps_without_workspace_dependencies() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create workspace Cargo.toml WITHOUT [workspace.dependencies]
        fs::write(
            root.join("Cargo.toml"),
            r#"
[workspace]
members = ["app"]
"#,
        ).unwrap();

        let info = WorkspaceInfo {
            is_workspace: true,
            workspace_root: Some(root.to_path_buf()),
            target_crate: Some("app".to_string()),
            target_crate_path: Some(root.join("app")),
            components_base_path: "app/src/components".to_string(),
        };

        assert!(!should_use_workspace_deps(&Some(info)));
    }

    #[test]
    fn test_add_to_workspace_dependencies() {
        let temp = TempDir::new().unwrap();
        let cargo_toml = temp.path().join("Cargo.toml");

        // Create initial Cargo.toml
        fs::write(
            &cargo_toml,
            r#"[workspace]
members = ["app"]

[workspace.dependencies]
leptos = "0.7"
"#,
        ).unwrap();

        // Add serde
        add_to_workspace_dependencies(&cargo_toml, "serde", "1.0").unwrap();

        // Verify
        let contents = fs::read_to_string(&cargo_toml).unwrap();
        assert!(contents.contains("serde = \"1.0\""), "Should contain serde dependency: {contents}");
        assert!(contents.contains("leptos = \"0.7\""), "Should preserve existing deps: {contents}");
    }

    #[test]
    fn test_add_workspace_ref_to_member() {
        let temp = TempDir::new().unwrap();
        let cargo_toml = temp.path().join("Cargo.toml");

        // Create initial member Cargo.toml
        fs::write(
            &cargo_toml,
            r#"[package]
name = "app"
version = "0.1.0"

[dependencies]
leptos.workspace = true
"#,
        ).unwrap();

        // Add serde.workspace = true
        add_workspace_ref_to_member(&cargo_toml, "serde").unwrap();

        // Verify
        let contents = fs::read_to_string(&cargo_toml).unwrap();
        assert!(contents.contains("serde"), "Should contain serde: {contents}");
        assert!(contents.contains("workspace = true") || contents.contains("workspace=true"),
            "Should have workspace = true: {contents}");
    }

    #[test]
    fn test_add_workspace_ref_uses_dotted_format() {
        let temp = TempDir::new().unwrap();
        let cargo_toml = temp.path().join("Cargo.toml");

        // Create initial member Cargo.toml
        fs::write(
            &cargo_toml,
            r#"[package]
name = "app"
version = "0.1.0"

[dependencies]
"#,
        ).unwrap();

        // Add validator.workspace = true
        add_workspace_ref_to_member(&cargo_toml, "validator").unwrap();

        // Verify it uses dotted format (validator.workspace = true) not inline ({ workspace = true })
        let contents = fs::read_to_string(&cargo_toml).unwrap();
        assert!(
            contents.contains("validator.workspace = true"),
            "Should use dotted format 'validator.workspace = true', got: {contents}"
        );
        assert!(
            !contents.contains("{ workspace = true }"),
            "Should NOT use inline table format, got: {contents}"
        );
    }

    #[test]
    fn test_add_workspace_dependency_full_flow() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create workspace root Cargo.toml
        fs::write(
            root.join("Cargo.toml"),
            r#"[workspace]
members = ["app"]

[workspace.dependencies]
leptos = "0.7"
"#,
        ).unwrap();

        // Create app directory and Cargo.toml
        let app_dir = root.join("app");
        fs::create_dir_all(&app_dir).unwrap();
        fs::write(
            app_dir.join("Cargo.toml"),
            r#"[package]
name = "app"
version = "0.1.0"

[dependencies]
leptos.workspace = true
"#,
        ).unwrap();

        let info = WorkspaceInfo {
            is_workspace: true,
            workspace_root: Some(root.to_path_buf()),
            target_crate: Some("app".to_string()),
            target_crate_path: Some(app_dir.clone()),
            components_base_path: "app/src/components".to_string(),
        };

        // Mock: we'll test the individual functions since fetch_latest_version requires network
        add_to_workspace_dependencies(&root.join("Cargo.toml"), "serde", "1.0").unwrap();
        add_workspace_ref_to_member(&app_dir.join("Cargo.toml"), "serde").unwrap();

        // Verify root Cargo.toml
        let root_contents = fs::read_to_string(root.join("Cargo.toml")).unwrap();
        assert!(root_contents.contains("serde = \"1.0\""), "Root should have serde: {root_contents}");

        // Verify app Cargo.toml
        let app_contents = fs::read_to_string(app_dir.join("Cargo.toml")).unwrap();
        assert!(app_contents.contains("serde"), "App should have serde ref: {app_contents}");
    }
}
