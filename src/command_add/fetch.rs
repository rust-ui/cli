

// use dotenv::dotenv;
// use std::env;
use std::{io::Write, path::Path};

// use crate::constants::env::ENV;
use crate::constants::url::URL;

use super::components_toml::ComponentsToml;

pub struct Fetch {}

impl Fetch {
    pub async fn fetch_index_content(url: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Attempt to fetch the content from the URL
        let response = reqwest::get(url).await;
    
        // Check if the request was successful
        let index_content_from_url = match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.text().await?
                } else {
                    let error_message = format!("🔸 Failed to fetch data: Server returned status {}", resp.status());
                    println!("{}", error_message); // Print the error message
                    return Err(error_message.into());
                }
            }
            Err(e) => {
                let error_message = format!("🔸 Failed to fetch data: {}", e);
                println!("{}", error_message); // Print the error message
                return Err(error_message.into());
            }
        };
    
        // Check if the fetched content is empty
        if index_content_from_url.is_empty() {
            let error_message = "🔸 Failed to fetch data: The server returned an empty response.";
            println!("{}", error_message); // Print the error message
            return Err(error_message.into());
        }
    
        Ok(index_content_from_url)
    }

    pub async fn from_registry_component_name_json_and_write_to_file(component_to_add: String) {
        // dotenv().ok();
        // let base_url = env::var(ENV::BASE_URL_STYLES_DEFAULT).unwrap_or_default();
        let base_url_styles_default = URL::BASE_URL_STYLES_DEFAULT;
    
        let formatted_url_json = format!("{}/{}.json", base_url_styles_default, component_to_add);
    
        let response = reqwest::get(&formatted_url_json).await.expect("Failed to fetch JSON");
        let json_content: serde_json::Value = response.json().await.expect("Failed to parse JSON");
    
        let registry_json_path = json_content["path"].as_str().expect("Path not found");
        let registry_json_content = json_content["files"][0]["content"].as_str().expect("Content not found");
    
        let user_config_path = ComponentsToml::get_base_path_from_Components_toml().unwrap_or_default();
        let full_path_component = format!("{}/{}", user_config_path, registry_json_path);
    
        // * Converts from "src/components/ui/button.rs" to "src/components/ui"
        let full_path_component_without_name_rs = std::path::Path::new(&full_path_component)
            .parent()
            .expect("Failed to get parent directory")
            .to_str()
            .expect("Failed to convert path to string")
            .to_string();
    
        //
        write_component_name_in_mod_rs_if_not_exists(component_to_add, full_path_component_without_name_rs);
    
        //
        // Create the directory if it doesn't exist
        let dir = Path::new(&full_path_component)
            .parent()
            .expect("Failed to get parent directory");
        std::fs::create_dir_all(dir).expect("Failed to create directories");
    
        // Write the content to the specified file
        std::fs::write(full_path_component, registry_json_content).expect("Failed to write to file");
    }
    
}



/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

fn write_component_name_in_mod_rs_if_not_exists(component_name: String, full_path_component_without_name_rs: String) {
    let mod_rs_path = format!("{}/mod.rs", full_path_component_without_name_rs);

    // Create the directory if it doesn't exist
    let dir = std::path::Path::new(&mod_rs_path)
        .parent()
        .expect("Failed to get parent directory");
    std::fs::create_dir_all(dir).expect("Failed to create directories");

    // Check if the mod.rs file already exists
    let mut mod_rs_content = String::new();
    if std::path::Path::new(&mod_rs_path).exists() {
        mod_rs_content = std::fs::read_to_string(&mod_rs_path).expect("Failed to read mod.rs");
    }

    // Check if the component already exists
    if mod_rs_content.contains(&component_name) {
        println!("Component {} already exists in mod.rs", component_name);
        return; // Exit the function if the component already exists
    }

    // Append the component name to mod.rs
    let mut mod_rs_file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(mod_rs_path)
        .expect("Failed to open mod.rs");

    // Write the new component name
    writeln!(mod_rs_file, "pub mod {};", component_name).expect("Failed to write to mod.rs");
}
