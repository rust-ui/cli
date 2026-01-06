use std::collections::HashSet;
use std::path::Path;

use cargo_toml::Manifest;

use crate::command_init::workspace_utils::{WorkspaceInfo, analyze_workspace};
use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::task_spinner::TaskSpinner;

pub fn process_cargo_deps(cargo_deps: &[String]) -> CliResult<()> {
    let spinner = TaskSpinner::new("Checking dependencies...");

    // Detect workspace to determine if we need --package flag
    let workspace_info = analyze_workspace().ok();

    // Get existing dependencies from Cargo.toml
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
    let mut added_deps = Vec::new();

    for dep in &new_deps {
        spinner.set_message(&format!("📦 Adding crate: {dep}"));

        let args = build_cargo_add_args(dep, &workspace_info);
        let output = std::process::Command::new("cargo")
            .args(&args)
            .output()
            .map_err(|_| CliError::cargo_operation("Failed to execute cargo add"))?;

        if output.status.success() {
            added_deps.push(dep);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CliError::cargo_operation(&format!("Failed to add dependency '{dep}': {stderr}")));
        }
    }

    let dependencies_str = added_deps.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
    let finish_message = format!("Successfully added to Cargo.toml: [{dependencies_str}] !");
    spinner.finish_success(&finish_message);

    Ok(())
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

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
    let contents = std::fs::read_to_string(&cargo_toml_path)?;
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
        // No --package for non-workspace
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
            target_crate: None, // Edge case: workspace but no target found
            target_crate_path: None,
            components_base_path: "src/components".to_string(),
        };

        let args = build_cargo_add_args("serde", &Some(info));
        // No --package if no target crate
        assert_eq!(args, vec!["add", "serde"]);
    }
}
