use std::path::Path;
// use dotenv::dotenv;
// use std::env;
use std::vec::Vec;

const UI_CONFIG_TOML: &str = "ui_config.toml";

use clap::{Arg, ArgMatches, Command};

use super::components::Components;
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

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

//
pub async fn process_add(matches: &ArgMatches) -> CliResult<()> {
    let user_components: Vec<String> =
        matches.get_many::<String>("components").unwrap_or_default().cloned().collect();

    // Fetch and parse tree.md
    let tree_content = RustUIClient::fetch_tree_md().await?;
    let tree_parser = TreeParser::parse_tree_md(&tree_content)?;

    // Resolve dependencies using the new tree-based system
    let resolved_set = tree_parser.resolve_dependencies(&user_components)?;

    // Convert HashSets to Vecs for compatibility with existing functions
    let all_resolved_components: Vec<String> = resolved_set.components.into_iter().collect();
    let all_resolved_parent_dirs: Vec<String> = resolved_set.parent_dirs.into_iter().collect();
    let all_resolved_cargo_dependencies: Vec<String> = resolved_set.cargo_deps.into_iter().collect();

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
        super::dependencies::add_cargo_dep_to_toml(&all_resolved_cargo_dependencies)?;
    }

    Ok(())
}
