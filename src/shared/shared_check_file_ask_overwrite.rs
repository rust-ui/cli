use colored::*;
use std::io::{self};
use std::path::Path;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[allow(dead_code)]
pub async fn shared_check_file_exist_and_ask_overwrite(file_path: &str, file_name: &str) -> anyhow::Result<bool> {
    if Path::new(file_path).exists() {
        println!(
            "⚠️ {} {} {} {}",
            file_name.yellow().bold(),
            "already exists.".yellow().bold(),
            "Do you want to overwrite it?".yellow(),
            "(y/n)".yellow().underline()
        );

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return Ok(true), // User confirmed overwrite
            _ => {
                println!(
                    "{} {}",
                    "🚧 Operation canceled.".blue().bold(),
                    "The file will not be overwritten".blue()
                );
                return Ok(false); // User declined overwrite
            }
        }
    }
    Ok(true) // File does not exist, proceed
}
