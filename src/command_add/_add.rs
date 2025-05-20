use clap::{Arg, ArgMatches, Command};
use std::path::Path;
// use dotenv::dotenv;
// use std::env;
use std::vec::Vec;

use super::components::{Components, MyComponent};
// use crate::constants::env::ENV;
use super::dependencies::Dependencies;
use super::registry::{Registry, RegistryComponent};
use crate::command_init::config::UiConfig;
use crate::constants::commands::{AddCommand, MyCommand};
use crate::constants::file_name::FILE_NAME;
use crate::constants::url::MyUrl;

pub fn command_add() -> Command {
    Command::new(MyCommand::ADD).about(AddCommand::ABOUT).arg(
        Arg::new(AddCommand::COMPONENTS)
            .help(AddCommand::HELP)
            .required(false)
            .num_args(1..),
    )
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

//
pub async fn process_add(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    // dotenv().ok();

    // let base_url = env::var(ENV::BASE_URL).unwrap_or_default();
    let url_registry_index_json = MyUrl::URL_REGISTRY_INDEX_JSON;

    let user_components: Vec<String> = matches
        .get_many::<String>(AddCommand::COMPONENTS)
        .unwrap_or_default()
        .cloned()
        .collect();

    let index_content_from_url = Registry::fetch_index_content(url_registry_index_json).await?;

    let vec_components_from_index: Vec<MyComponent> = serde_json::from_str(&index_content_from_url)
        .map_err(|e| format!("Failed to parse registry index JSON: {e}"))?;

    let all_tree_resolved = Dependencies::all_tree_resolved(user_components, &vec_components_from_index);
    Dependencies::print_dependency_tree(&all_tree_resolved); // Can be commented out
    let all_resolved_components = Dependencies::get_all_resolved_components(&all_tree_resolved);
    let all_resolved_parent_dirs = Dependencies::get_all_resolved_parent_dirs(&all_tree_resolved);
    let all_resolved_cargo_dependencies = Dependencies::get_all_resolved_cargo_dependencies(&all_tree_resolved);

    // println!("--------------------------------");
    // println!("All resolved components: {:?}", all_resolved_components);
    // println!("All resolved parent dirs: {:?}", all_resolved_parent_dirs);
    // println!("All resolved cargo dependencies: {:?}", all_resolved_cargo_dependencies);

    // Create components/mod.rs if it does not exist
    let components_base_path = UiConfig::try_reading_ui_config(FILE_NAME::UI_CONFIG_TOML)?.base_path_components;

    Components::create_components_mod_if_not_exists_with_pub_mods(
        components_base_path.clone(),
        all_resolved_parent_dirs.clone(),
    );

    //  Register `components` module
    let mut file_path = components_base_path.split("/").collect::<Vec<&str>>();
    assert_eq!(file_path.pop(), Some("components"));

    let file_path = file_path.join("/");
    let entry_file_path = if Path::new(&format!("{file_path}/lib.rs")).exists() {
        format!("{file_path}/lib.rs")
    } else {
        format!("{file_path}/main.rs")
    };

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
        Dependencies::add_cargo_dep_to_toml(&all_resolved_cargo_dependencies)?;
    }

    Ok(())
}
