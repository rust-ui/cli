use clap::{Arg, ArgMatches, Command};
// use dotenv::dotenv;
// use std::env;
use std::{io::Write, vec::Vec};

// use crate::constants::env::ENV;
use super::components_toml::ComponentsToml;
use super::dependencies::Dependencies;
use super::registry::{Registry, RegistryComponent};
use crate::constants::url::URL;
use crate::{
    command_add::models::MyComponent,
    constants::commands::{ADD, COMMAND},
};

pub fn command_add() -> Command {
    Command::new(COMMAND::ADD)
        .about(ADD::ABOUT)
        .arg(Arg::new(ADD::COMPONENTS).help(ADD::HELP).required(false).num_args(1..))
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

//
pub async fn process_add(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    // dotenv().ok();

    let base_path_components = ComponentsToml::try_extract_base_path_components_from_components_toml();

    if base_path_components.is_err() {
        eprintln!("{}", base_path_components.unwrap_err());
        return Ok(()); // Early return
    }

    // let base_url = env::var(ENV::BASE_URL).unwrap_or_default();
    let url_registry_index_json = URL::URL_REGISTRY_INDEX_JSON;

    let user_components: Vec<String> = matches
        .get_many::<String>(ADD::COMPONENTS)
        .unwrap_or_default()
        .cloned()
        .collect();

    let index_content_from_url = Registry::fetch_index_content(&url_registry_index_json).await?;

    let vec_components_from_index: Vec<MyComponent> = serde_json::from_str(&index_content_from_url).unwrap();

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
    let user_config_path_toml = ComponentsToml::get_base_path_from_Components_toml().unwrap_or_default();
    create_components_mod_if_not_exists_with_pub_mods(user_config_path_toml, all_resolved_parent_dirs.clone());

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

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// TODO. Have instead all_resolved_parent_dirs instead of compomnents.
fn create_components_mod_if_not_exists_with_pub_mods(user_config_path: String, parent_dirs: Vec<String>) {
    let components_mod_path = format!("{}/mod.rs", user_config_path);

    // println!("Parent directories to add to components/mod.rs: {:?}", parent_dirs);

    // Create the directory if it doesn't exist
    let dir = std::path::Path::new(&components_mod_path)
        .parent()
        .expect("Failed to get parent directory");
    std::fs::create_dir_all(dir).expect("Failed to create directories");

    // Initialize mod_content
    let mut mod_content = String::new();

    // Check if the mod.rs file already exists
    if std::path::Path::new(&components_mod_path).exists() {
        mod_content = std::fs::read_to_string(&components_mod_path).expect("Failed to read mod.rs");
    }

    // Create or open the mod.rs file for writing
    let mut mod_rs_file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(components_mod_path)
        .expect("Failed to open mod.rs");

    // Add each parent directory as a module if it doesn't already exist
    for parent_dir in parent_dirs {
        if !mod_content.contains(&format!("pub mod {};", parent_dir)) {
            writeln!(mod_rs_file, "pub mod {};", parent_dir).expect("🔸 Failed to write to mod.rs");
        }
    }
}
