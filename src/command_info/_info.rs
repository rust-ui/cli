use clap::Command;

use crate::command_add::installed::get_installed_components;
use crate::command_init::config::UiConfig;
use crate::command_init::workspace_utils::analyze_workspace;
use crate::shared::cli_error::CliResult;

const UI_CONFIG_TOML: &str = "ui_config.toml";

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

pub fn command_info() -> Command {
    Command::new("info").about("Show project configuration and installed components")
}

pub fn process_info() -> CliResult<()> {
    let config = UiConfig::try_reading_ui_config(UI_CONFIG_TOML)?;
    let installed = get_installed_components(&config.base_path_components);
    let workspace = analyze_workspace().ok();

    let output = format_info(&config.base_color, &config.base_path_components, &installed, workspace.as_ref());
    println!("{output}");
    Ok(())
}

/* ========================================================== */
/*                     ✨ HELPERS ✨                          */
/* ========================================================== */

/// Pure formatter — takes plain data, returns the info string. Fully testable.
pub fn format_info(
    base_color: &str,
    base_path: &str,
    installed: &std::collections::HashSet<String>,
    workspace: Option<&crate::command_init::workspace_utils::WorkspaceInfo>,
) -> String {
    let mut lines: Vec<String> = Vec::new();

    lines.push(format!("  Config file   ui_config.toml"));
    lines.push(format!("  Base color    {base_color}"));
    lines.push(format!("  Base path     {base_path}"));

    if let Some(ws) = workspace {
        let workspace_label = if ws.is_workspace { "yes" } else { "no" };
        lines.push(format!("  Workspace     {workspace_label}"));
        if let Some(ref crate_name) = ws.target_crate {
            lines.push(format!("  Target crate  {crate_name}"));
        }
    }

    let count = installed.len();
    if count == 0 {
        lines.push("  Installed     none".to_string());
    } else {
        let mut sorted: Vec<&String> = installed.iter().collect();
        sorted.sort();
        lines.push(format!("  Installed ({count})  {}", sorted.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")));
    }

    lines.join("\n")
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::command_init::workspace_utils::WorkspaceInfo;

    fn installed(names: &[&str]) -> HashSet<String> {
        names.iter().map(|s| s.to_string()).collect()
    }

    fn no_workspace() -> Option<WorkspaceInfo> {
        None
    }

    fn single_crate_workspace() -> Option<WorkspaceInfo> {
        Some(WorkspaceInfo {
            is_workspace: false,
            workspace_root: None,
            target_crate: Some("my-app".to_string()),
            target_crate_path: None,
            components_base_path: "src/components".to_string(),
        })
    }

    fn full_workspace() -> Option<WorkspaceInfo> {
        Some(WorkspaceInfo {
            is_workspace: true,
            workspace_root: Some(std::path::PathBuf::from("/project")),
            target_crate: Some("frontend".to_string()),
            target_crate_path: None,
            components_base_path: "frontend/src/components".to_string(),
        })
    }

    #[test]
    fn shows_config_fields() {
        let result = format_info("neutral", "src/components", &installed(&[]), no_workspace().as_ref());
        assert!(result.contains("ui_config.toml"));
        assert!(result.contains("neutral"));
        assert!(result.contains("src/components"));
    }

    #[test]
    fn shows_none_when_no_components_installed() {
        let result = format_info("neutral", "src/components", &installed(&[]), no_workspace().as_ref());
        assert!(result.contains("none"));
    }

    #[test]
    fn shows_installed_components_sorted() {
        let result = format_info("neutral", "src/components", &installed(&["card", "button", "badge"]), no_workspace().as_ref());
        assert!(result.contains("badge, button, card"));
    }

    #[test]
    fn shows_installed_count() {
        let result = format_info("neutral", "src/components", &installed(&["button", "badge"]), no_workspace().as_ref());
        assert!(result.contains("(2)"));
    }

    #[test]
    fn shows_workspace_no_when_single_crate() {
        let result = format_info("neutral", "src/components", &installed(&[]), single_crate_workspace().as_ref());
        assert!(result.contains("Workspace     no") || result.contains("no"));
    }

    #[test]
    fn shows_workspace_yes_when_in_workspace() {
        let result = format_info("neutral", "src/components", &installed(&[]), full_workspace().as_ref());
        assert!(result.contains("yes"));
        assert!(result.contains("frontend"));
    }

    #[test]
    fn shows_target_crate_when_available() {
        let result = format_info("neutral", "src/components", &installed(&[]), single_crate_workspace().as_ref());
        assert!(result.contains("my-app"));
    }

    #[test]
    fn no_workspace_info_omits_workspace_line() {
        let result = format_info("neutral", "src/components", &installed(&[]), no_workspace().as_ref());
        assert!(!result.contains("Workspace"));
        assert!(!result.contains("Target crate"));
    }

    #[test]
    fn single_installed_component() {
        let result = format_info("neutral", "src/components", &installed(&["button"]), no_workspace().as_ref());
        assert!(result.contains("(1)"));
        assert!(result.contains("button"));
    }
}
