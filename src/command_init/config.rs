use std::fs;
use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};
use toml_edit::{DocumentMut, Item, Value};

use crate::command_init::crates::INIT_CRATES;
use crate::command_init::workspace_utils::{WorkspaceInfo, analyze_workspace, check_leptos_dependency};
use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::task_spinner::TaskSpinner;

///
/// UiConfig - Minimal configuration stored in ui_config.toml
/// Workspace detection is done dynamically via analyze_workspace()
///
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct UiConfig {
    pub base_color: String,
    pub base_path_components: String,
    pub tailwind_input_file: String,
}

impl UiConfig {
    pub fn try_reading_ui_config(toml_path: &str) -> CliResult<UiConfig> {
        if !Path::new(toml_path).exists() {
            return Err(CliError::project_not_initialized());
        }
        let contents = fs::read_to_string(toml_path)?;
        let ui_config: UiConfig = toml::from_str(&contents)?;
        Ok(ui_config)
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        // Detect workspace and set appropriate component path
        let base_path_components = match analyze_workspace() {
            Ok(info) => info.components_base_path,
            Err(_) => "src/components".to_string(),
        };

        UiConfig {
            base_color: "neutral".to_string(),
            base_path_components,
            tailwind_input_file: "style/tailwind.css".to_string(),
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

pub async fn add_init_crates() -> CliResult<()> {
    // Detect workspace dynamically
    let workspace_info = analyze_workspace().ok();

    // Check what crates already exist in workspace.dependencies
    let workspace_crates = get_workspace_dependencies(&workspace_info);

    for my_crate in INIT_CRATES {
        // Skip leptos if it's already installed to preserve user's existing configuration
        if my_crate.name == "leptos" && check_leptos_dependency()? {
            continue;
        }

        let spinner = TaskSpinner::new(&format!("Adding and installing {} crate...", my_crate.name));

        // Check if crate exists in workspace.dependencies
        if workspace_crates.contains(&my_crate.name.to_string()) {
            // Just add workspace reference to member, don't use cargo add
            if let Some(ref info) = workspace_info {
                if info.is_workspace {
                    if let Some(ref member_path) = info.target_crate_path {
                        let member_cargo_toml = member_path.join("Cargo.toml");
                        add_workspace_ref_to_member(&member_cargo_toml, my_crate.name)?;
                        spinner.finish_success(&format!("{} (workspace) added successfully.", my_crate.name));
                        continue;
                    }
                }
            }
        }

        // Use cargo add for non-workspace or new dependencies
        let mut args = vec!["add".to_owned(), my_crate.name.to_owned()];

        // Add --package flag if we're in a workspace with a target crate
        if let Some(ref info) = workspace_info {
            if info.is_workspace {
                if let Some(ref crate_name) = info.target_crate {
                    args.push("--package".to_owned());
                    args.push(crate_name.clone());
                }
            }
        }

        if let Some(features) = my_crate.features
            && !features.is_empty()
        {
            args.push("--features".to_owned());
            args.push(features.join(","));
        }

        let output = Command::new("cargo").args(&args).output().map_err(|e| {
            CliError::cargo_operation(&format!("Failed to execute cargo add {}: {e}", my_crate.name))
        })?;

        if output.status.success() {
            spinner.finish_success(&format!("{} added successfully.", my_crate.name));
        } else {
            return Err(CliError::cargo_operation(&format!(
                "Failed to add crate '{}': {}",
                my_crate.name,
                String::from_utf8_lossy(&output.stderr)
            )));
        }
    }
    Ok(())
}

/* ========================================================== */
/*                     ✨ HELPERS ✨                          */
/* ========================================================== */

/// Get list of crates defined in [workspace.dependencies]
fn get_workspace_dependencies(workspace_info: &Option<WorkspaceInfo>) -> Vec<String> {
    let Some(info) = workspace_info else {
        return Vec::new();
    };

    let Some(workspace_root) = &info.workspace_root else {
        return Vec::new();
    };

    let root_cargo_toml = workspace_root.join("Cargo.toml");
    if !root_cargo_toml.exists() {
        return Vec::new();
    }

    let Ok(contents) = fs::read_to_string(&root_cargo_toml) else {
        return Vec::new();
    };

    let Ok(doc) = contents.parse::<DocumentMut>() else {
        return Vec::new();
    };

    // Get keys from [workspace.dependencies]
    doc.get("workspace")
        .and_then(|w| w.get("dependencies"))
        .and_then(|d| d.as_table())
        .map(|t| t.iter().map(|(k, _)| k.to_string()).collect())
        .unwrap_or_default()
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

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_get_workspace_dependencies_returns_crates() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create workspace Cargo.toml with dependencies
        fs::write(
            root.join("Cargo.toml"),
            r#"[workspace]
members = ["app"]

[workspace.dependencies]
leptos = "0.7"
tw_merge = { version = "0.1", features = ["variant"] }
serde = "1.0"
"#,
        ).unwrap();

        let info = WorkspaceInfo {
            is_workspace: true,
            workspace_root: Some(root.to_path_buf()),
            target_crate: Some("app".to_string()),
            target_crate_path: Some(root.join("app")),
            components_base_path: "app/src/components".to_string(),
        };

        let deps = get_workspace_dependencies(&Some(info));

        assert!(deps.contains(&"leptos".to_string()));
        assert!(deps.contains(&"tw_merge".to_string()));
        assert!(deps.contains(&"serde".to_string()));
        assert_eq!(deps.len(), 3);
    }

    #[test]
    fn test_get_workspace_dependencies_empty_when_no_workspace() {
        let deps = get_workspace_dependencies(&None);
        assert!(deps.is_empty());
    }

    #[test]
    fn test_get_workspace_dependencies_empty_when_not_workspace() {
        let info = WorkspaceInfo {
            is_workspace: false,
            workspace_root: None,
            target_crate: Some("app".to_string()),
            target_crate_path: None,
            components_base_path: "src/components".to_string(),
        };

        let deps = get_workspace_dependencies(&Some(info));
        assert!(deps.is_empty());
    }

    #[test]
    fn test_get_workspace_dependencies_empty_when_no_workspace_deps_section() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create workspace Cargo.toml WITHOUT [workspace.dependencies]
        fs::write(
            root.join("Cargo.toml"),
            r#"[workspace]
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

        let deps = get_workspace_dependencies(&Some(info));
        assert!(deps.is_empty());
    }

    #[test]
    fn test_add_workspace_ref_to_member_uses_dotted_format() {
        let temp = TempDir::new().unwrap();
        let cargo_toml = temp.path().join("Cargo.toml");

        fs::write(
            &cargo_toml,
            r#"[package]
name = "app"
version = "0.1.0"

[dependencies]
leptos.workspace = true
"#,
        ).unwrap();

        add_workspace_ref_to_member(&cargo_toml, "tw_merge").unwrap();

        let contents = fs::read_to_string(&cargo_toml).unwrap();
        assert!(
            contents.contains("tw_merge.workspace = true"),
            "Should use dotted format, got: {contents}"
        );
    }

    #[test]
    fn test_add_workspace_ref_skips_existing_dep() {
        let temp = TempDir::new().unwrap();
        let cargo_toml = temp.path().join("Cargo.toml");

        let original = r#"[package]
name = "app"
version = "0.1.0"

[dependencies]
tw_merge.workspace = true
"#;
        fs::write(&cargo_toml, original).unwrap();

        // Should not error or modify when dep already exists
        add_workspace_ref_to_member(&cargo_toml, "tw_merge").unwrap();

        let contents = fs::read_to_string(&cargo_toml).unwrap();
        // Count occurrences - should still be just one
        assert_eq!(
            contents.matches("tw_merge").count(),
            1,
            "Should not duplicate: {contents}"
        );
    }

    #[test]
    fn test_workspace_crate_detection_for_init() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create workspace with tw_merge already in workspace.dependencies
        fs::write(
            root.join("Cargo.toml"),
            r#"[workspace]
members = ["app"]

[workspace.dependencies]
tw_merge = { version = "0.1", features = ["variant"] }
leptos_ui = "0.3"
"#,
        ).unwrap();

        let info = WorkspaceInfo {
            is_workspace: true,
            workspace_root: Some(root.to_path_buf()),
            target_crate: Some("app".to_string()),
            target_crate_path: Some(root.join("app")),
            components_base_path: "app/src/components".to_string(),
        };

        let workspace_crates = get_workspace_dependencies(&Some(info));

        // These should be detected as workspace crates
        assert!(workspace_crates.contains(&"tw_merge".to_string()));
        assert!(workspace_crates.contains(&"leptos_ui".to_string()));

        // These should NOT be in workspace crates (not defined)
        assert!(!workspace_crates.contains(&"icons".to_string()));
    }
}
