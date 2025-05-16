// use dotenv::dotenv;
use serde_json;
// use std::env;
use std::io;

use crate::command_init::fetch::Fetch;
// use crate::constants::env::ENV;
use crate::constants::url::URL;

const LABEL: &str = "label";

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub struct UserInput {}

impl UserInput {
    pub async fn handle_index_styles() {
        // dotenv().ok();

        // let url_registry_styles_json = env::var(ENV::URL_REGISTRY_STYLES_JSON).unwrap_or_default();

        let url_registry_styles_json = URL::URL_REGISTRY_STYLES_JSON;

        let styles_index_result = Fetch::from_url(&url_registry_styles_json).await;
        // println!("{}", styles_index_result.as_ref().unwrap());

        // Parse the JSON string into Vec<serde_json::Value>
        if let Ok(styles_index) = styles_index_result {
            // Convert the String to a Vec<serde_json::Value>
            let vec_styles: Vec<serde_json::Value> = serde_json::from_str(&styles_index).unwrap();
            ask_user_choose_style(vec_styles);
        }
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

/// Ask user to choose a style
fn ask_user_choose_style(vec_styles: Vec<serde_json::Value>) {
    // Print available styles
    for (index, style) in vec_styles.iter().enumerate() {
        if let Some(label) = style.get(LABEL) {
            println!("\n{}: {}", index + 1, label);
        }
    }

    // Prompt user for choice
    println!("Please choose a style by entering the corresponding number:");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("🔸 Failed to read line");

    // Parse the choice and print the selected style
    if let Ok(index) = user_input.trim().parse::<usize>() {
        if index > 0 && index <= vec_styles.len() {
            if let Some(selected_style) = vec_styles.get(index - 1) {
                if let Some(label) = selected_style.get(LABEL) {
                    println!("You selected: {}", label);
                }
            }
        } else {
            println!(
                "🔸 Invalid choice. Please select a number between 1 and {}.",
                vec_styles.len()
            );
        }
    } else {
        println!("🔸 Invalid input. Please enter a number.");
    }
}
