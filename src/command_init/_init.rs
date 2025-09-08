use std::fs::{self, File};
use std::io::{self, Write};

use clap::{Arg, Command};

const UI_CONFIG_TOML: &str = "ui_config.toml";
const PACKAGE_JSON: &str = "package.json";

use super::config::{UiConfig, add_init_crates};
use super::install::InstallType;
use super::user_input::UserInput;
use super::workspace_utils::check_leptos_dependency;
use crate::command_init::install::install_dependencies;
use crate::command_init::template::MyTemplate;
use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::task_spinner::TaskSpinner;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub fn command_init() -> Command {
    Command::new("init")
        .about("Initialize the project")
        .arg(Arg::new("project_name").help("The name of the project to initialize").required(false))
        .subcommand(Command::new("run").about("Run the initialization logic"))
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn process_init() -> CliResult<()> {
    // Check if Leptos is installed before proceeding
    if !check_leptos_dependency()? {
        return Err(CliError::config(
            "Leptos dependency not found in Cargo.toml. Please install Leptos first.",
        ));
    }

    let ui_config = UiConfig::default();

    let ui_config_toml = toml::to_string_pretty(&ui_config)
        .map_err(|e| CliError::config(&format!("Failed to serialize UiConfig: {e}")))?;

    INIT_TEMPLATE_FILE(UI_CONFIG_TOML, &ui_config_toml).await?;
    INIT_TEMPLATE_FILE(PACKAGE_JSON, MyTemplate::PACKAGE_JSON).await?;
    INIT_TEMPLATE_FILE(&ui_config.tailwind_input_file, MyTemplate::STYLE_TAILWIND_CSS).await?;

    add_init_crates().await?;

    UserInput::handle_index_styles().await?;

    install_dependencies(&[InstallType::Tailwind]).await?;
    Ok(())
}

//
/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

/// INIT TEMPLATE FILE
#[allow(non_snake_case)]
async fn INIT_TEMPLATE_FILE(file_name: &str, template: &str) -> CliResult<()> {
    let file_path = std::path::Path::new(".").join(file_name);

    // if !shared_check_file_exist_and_ask_overwrite(&file_path, file_name_ext).await {
    //     return;
    // }

    let spinner = TaskSpinner::new("Writing to file...");

    shared_write_template_file(&file_path.to_string_lossy(), template).await?;

    let finish_message = format!("Writing {file_name} complete.");
    spinner.finish_success(&finish_message);
    Ok(())
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

async fn shared_write_template_file(file_path: &str, template: &str) -> io::Result<()> {
    // Create the directory if it doesn't exist
    if let Some(dir) = std::path::Path::new(file_path).parent() {
        fs::create_dir_all(dir)?;
    }

    match File::create(file_path) {
        Ok(mut file) => {
            file.write_all(template.as_bytes())?;
            Ok(())
        }
        Err(err) => {
            eprintln!("🔸 Error: {err}");
            Err(err)
        }
    }
}
