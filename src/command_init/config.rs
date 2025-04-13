use dotenv::dotenv;
use std::env;
use indicatif::ProgressBar;
use std::fs;
use std::process::Command;
use std::time::Duration;

use crate::constants::others::{CARGO_TOML_FILE, LEPTOS_0_6_13, SPINNER_UPDATE_DURATION};
use crate::{command_init::fetch::Fetch, constants::env::ENV};



pub struct Config {}

impl Config {
    pub async fn handle_config_schema() {
        dotenv().ok();
    
        let url_config_schema_json = env::var(ENV::URL_CONFIG_SCHEMA_JSON).unwrap_or_default();
    
        let _ = Fetch::handle_fetch_from_init(&url_config_schema_json).await;
    }

    pub async fn handle_cargo_toml() {
        ensure_leptos_dependencies_are_0_6_13();
        // add_tailwind_fuse_and_leptos_use();
        // handle_adding_leptos_use_to_ssr_features();
        handle_tailwind_input_file();
    }
    
    
}





/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

fn ensure_leptos_dependencies_are_0_6_13() {
    match fs::read_to_string(CARGO_TOML_FILE) {
        Ok(mut contents) => {
            let dependencies = ["leptos", "leptos_axum", "leptos_meta", "leptos_router"];

            for dep in dependencies.iter() {
                let dep_pattern = format!("{} = {{ version = \"", dep);
                if let Some(start_pos) = contents.find(&dep_pattern) {
                    let version_start = start_pos + dep_pattern.len();
                    if let Some(version_end) = contents[version_start..].find('"') {
                        let current_version = &contents[version_start..version_start + version_end];
                        if current_version != LEPTOS_0_6_13 {
                            contents.replace_range(version_start..version_start + version_end, LEPTOS_0_6_13);
                        }
                    }
                }
            }

            // Write the modified contents back to the file
            if let Err(e) = fs::write(CARGO_TOML_FILE, &contents) {
                eprintln!("🔸 Error writing to file: {}", e);
            }
        }
        Err(e) => {
            eprintln!("🔸 Error reading file: {}", e);
        }
    }
}


fn add_tailwind_fuse_and_leptos_use() {
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Adding crates: rustui_merge and leptos-use");
    spinner.enable_steady_tick(Duration::from_millis(SPINNER_UPDATE_DURATION));

    let output = Command::new("cargo")
        .args([
            "add",
            "rustui_merge@0.3.0",
            "--features",
            "rustui_merge@0.3.0/variant,rustui_merge@0.3.0/debug",
            "leptos-use@0.13.5",
            "--features",
            "leptos-use@0.13.5/storage,leptos-use@0.13.5/docs,leptos-use@0.13.5/math",
        ])
        .output()
        .expect("Failed to execute cargo add command");

    if output.status.success() {
        spinner.finish_with_message("✔️ Crates added successfully.");
    } else {
        spinner.finish_with_message(format!(
            "🔸 Error adding crates: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
}

fn handle_adding_leptos_use_to_ssr_features() {
    match fs::read_to_string(CARGO_TOML_FILE) {
        Ok(mut contents) => {
            if let Some(start_pos) = contents.find("ssr = [") {
                // Find the position to insert the new features
                if let Some(end_pos) = contents[start_pos..].find(']') {
                    let insert_pos = start_pos + end_pos;
                    let new_features = r#"    "leptos-use/ssr",
    "leptos-use/axum",
"#;

                    // Check if the features are already present to avoid duplicates
                    if !contents[start_pos..insert_pos].contains("leptos-use/ssr") {
                        contents.insert_str(insert_pos, new_features);
                    }
                }
            } else {
                println!("'ssr' feature not found.");
            }

            // Write the modified contents back to the file
            if let Err(e) = fs::write(CARGO_TOML_FILE, &contents) {
                eprintln!("Error writing to file: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}

fn handle_tailwind_input_file() {
    match fs::read_to_string(CARGO_TOML_FILE) {
        Ok(mut contents) => {
            // Check if "style-file" exists
            if let Some(start_pos) = contents.find("style-file") {
                // Find the end of the line containing "style-file"
                if let Some(end_pos) = contents[start_pos..].find('\n') {
                    let end_of_line = start_pos + end_pos;
                    // Replace the line with the new entry
                    contents.replace_range(start_pos..end_of_line, "tailwind-input-file = \"style/tailwind.css\"");
                }
            } else if let Some(start_pos) = contents.find("tailwind-input-file") {
                // Find the end of the line containing "tailwind-input-file"
                if let Some(end_pos) = contents[start_pos..].find('\n') {
                    let end_of_line = start_pos + end_pos;
                    // Replace the line with the new entry
                    contents.replace_range(start_pos..end_of_line, "tailwind-input-file = \"style/tailwind.css\"");
                }
            } else {
                println!("🔸 Error. Neither 'style-file' nor 'tailwind-input-file' entry found.");
            }

            // Write the modified contents back to the file
            if let Err(e) = fs::write(CARGO_TOML_FILE, &contents) {
                eprintln!("🔸 Error writing to file: {}", e);
            }
        }
        Err(e) => {
            eprintln!("🔸 Error reading file: {}", e);
        }
    }
}
