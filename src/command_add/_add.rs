use clap::{Arg, ArgMatches, Command};
use std::path::Path;
// use dotenv::dotenv;
// use std::env;
use std::vec::Vec;

use super::components::{Components, MyComponent};
// use crate::constants::env::ENV;
use super::dependencies;
use super::registry::{Registry, RegistryComponent};
use crate::command_init::config::UiConfig;
use crate::constants::file_name::FileName;
use crate::constants::url::MyUrl;
use crate::shared::cli_error::{CliError, CliResult};

pub fn command_add() -> Command {
    Command::new("add").about("Add components and dependencies to your project").arg(
        Arg::new("components")
            .help("The components to add (space-separated)")
            .required(false)
            .num_args(1..),
    )
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

//
pub async fn process_add(matches: &ArgMatches) -> CliResult<()> {
    // dotenv().ok();

    // let base_url = env::var(ENV::BASE_URL).unwrap_or_default();
    let url_registry_index_json = MyUrl::URL_REGISTRY_INDEX_JSON;

    let user_components: Vec<String> = matches
        .get_many::<String>("components")
        .unwrap_or_default()
        .cloned()
        .collect();

    let index_content_from_url = Registry::fetch_index_content(url_registry_index_json).await?;

    let vec_components_from_index: Vec<MyComponent> = serde_json::from_str(&index_content_from_url)
        .map_err(|e| CliError::malformed_registry(&format!("Failed to parse registry index JSON: {e}")))?;

    let all_tree_resolved = dependencies::all_tree_resolved(user_components, &vec_components_from_index)?;
    dependencies::print_dependency_tree(&all_tree_resolved); // Can be commented out
    let all_resolved_components = dependencies::get_all_resolved_components(&all_tree_resolved);
    let all_resolved_parent_dirs = dependencies::get_all_resolved_parent_dirs(&all_tree_resolved);
    let all_resolved_cargo_dependencies = dependencies::get_all_resolved_cargo_dependencies(&all_tree_resolved);

    // println!("--------------------------------");
    // println!("All resolved components: {:?}", all_resolved_components);
    // println!("All resolved parent dirs: {:?}", all_resolved_parent_dirs);
    // println!("All resolved cargo dependencies: {:?}", all_resolved_cargo_dependencies);

    // Create components/mod.rs if it does not exist
    let components_base_path = UiConfig::try_reading_ui_config(FileName::UI_CONFIG_TOML)?.base_path_components;

    Components::create_components_mod_if_not_exists_with_pub_mods(
        components_base_path.clone(),
        all_resolved_parent_dirs.clone(),
    )?;

    //  Register `components` module
    let components_path = Path::new(&components_base_path);
    let parent_path = components_path.parent()
        .ok_or_else(|| CliError::invalid_path(&components_base_path, "no parent directory"))?;
    
    let entry_file_path = if parent_path.join("lib.rs").exists() {
        parent_path.join("lib.rs")
    } else {
        parent_path.join("main.rs")
    };
    
    let entry_file_path = entry_file_path.to_string_lossy().to_string();

    Components::register_components_in_application_entry(entry_file_path.as_str())?;

    // Components to add
    for component_name_json in all_resolved_components {
        RegistryComponent::fetch_from_registry(component_name_json)
            .await?
            .then_write_to_file()
            .await?;
    }

    // Handle cargo dependencies if any exist
    if !all_resolved_cargo_dependencies.is_empty() {
        dependencies::add_cargo_dep_to_toml(&all_resolved_cargo_dependencies)?;
    }

    Ok(())
}
