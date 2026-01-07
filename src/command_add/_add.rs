use std::collections::HashSet;
use std::path::Path;
use std::vec::Vec;

const UI_CONFIG_TOML: &str = "ui_config.toml";

use clap::{Arg, ArgMatches, Command};

use super::components::Components;
use super::installed::get_installed_components;
use super::registry::RegistryComponent;
use super::tree_parser::TreeParser;
use crate::command_init::config::UiConfig;
use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::rust_ui_client::RustUIClient;

pub fn command_add() -> Command {
    Command::new("add").about("Add components and dependencies to your project").arg(
        Arg::new("components").help("The components to add (space-separated)").required(false).num_args(1..),
    )
}

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

//
pub async fn process_add(matches: &ArgMatches) -> CliResult<()> {
    let user_components: Vec<String> =
        matches.get_many::<String>("components").unwrap_or_default().cloned().collect();

    // Fetch and parse tree.md
    let tree_content = RustUIClient::fetch_tree_md().await?;
    let tree_parser = TreeParser::parse_tree_md(&tree_content)?;

    // Get base path for components (try reading config, fallback to default)
    let base_path = UiConfig::try_reading_ui_config(UI_CONFIG_TOML)
        .map(|c| c.base_path_components)
        .unwrap_or_else(|_| "src/components".to_string());

    // Detect already installed components
    let installed = get_installed_components(&base_path);

    // If no components provided, launch TUI
    let user_components = if user_components.is_empty() {
        let component_names: Vec<String> = tree_parser.get_all_component_names();
        let dependencies = tree_parser.get_dependencies_map();
        let selected = super::ratatui::run_tui(component_names, installed, dependencies)?;
        if selected.is_empty() {
            println!("No components selected.");
            return Ok(());
        }
        selected
    } else {
        user_components
    };

    // Resolve dependencies using the new tree-based system
    let resolved_set = tree_parser.resolve_dependencies(&user_components)?;

    // Convert HashSets to Vecs for compatibility with existing functions
    let all_resolved_components: Vec<String> = resolved_set.components.into_iter().collect();
    let all_resolved_parent_dirs: Vec<String> = resolved_set.parent_dirs.into_iter().collect();
    let all_resolved_cargo_dependencies: Vec<String> = resolved_set.cargo_deps.into_iter().collect();
    let all_resolved_js_files: HashSet<String> = resolved_set.js_files;

    // Create components/mod.rs if it does not exist
    let components_base_path = UiConfig::try_reading_ui_config(UI_CONFIG_TOML)?.base_path_components;

    Components::create_components_mod_if_not_exists_with_pub_mods(
        components_base_path.clone(),
        all_resolved_parent_dirs,
    )?;

    // Register `components` module
    let components_path = Path::new(&components_base_path);
    let parent_path = components_path
        .parent()
        .ok_or_else(|| CliError::invalid_path(&components_base_path, "no parent directory"))?;

    let entry_file_path = if parent_path.join("lib.rs").exists() {
        parent_path.join("lib.rs")
    } else {
        parent_path.join("main.rs")
    };

    let entry_file_path = entry_file_path.to_string_lossy().to_string();

    Components::register_components_in_application_entry(entry_file_path.as_str())?;

    // Components to add
    for component_name in all_resolved_components {
        RegistryComponent::fetch_from_registry(component_name).await?.then_write_to_file().await?;
    }

    // Handle cargo dependencies if any exist
    if !all_resolved_cargo_dependencies.is_empty() {
        super::dependencies::process_cargo_deps(&all_resolved_cargo_dependencies)?;
    }

    // Handle JS file dependencies if any exist
    if !all_resolved_js_files.is_empty() {
        process_js_files(&all_resolved_js_files).await?;
    }

    Ok(())
}

/// Download and install JS files to the user's public directory
async fn process_js_files(js_files: &HashSet<String>) -> CliResult<()> {
    use crate::shared::task_spinner::TaskSpinner;

    let spinner = TaskSpinner::new("Installing JS files...");

    for js_path in js_files {
        spinner.set_message(&format!("📜 Downloading {js_path}"));

        // Fetch the JS file content
        let content = RustUIClient::fetch_js_file(js_path).await?;

        // Determine the output path (public/ + js_path)
        let output_path = Path::new("public").join(js_path.trim_start_matches('/'));

        // Create parent directories if they don't exist
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent).map_err(|_| CliError::directory_create_failed())?;
        }

        // Check if file already exists
        if output_path.exists() {
            spinner.set_message(&format!("⏭️  Skipping {js_path} (already exists)"));
            continue;
        }

        // Write the file
        std::fs::write(&output_path, content).map_err(|_| CliError::file_write_failed())?;
    }

    let files_str = js_files.iter().cloned().collect::<Vec<_>>().join(", ");
    spinner.finish_success(&format!("JS files installed: [{files_str}]"));

    Ok(())
}
