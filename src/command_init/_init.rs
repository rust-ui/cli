use clap::{Arg, Command};

use super::config::{UiConfig, add_init_crates};
use super::install::InstallType;
use super::user_input::UserInput;
use crate::command_init::install::install_dependencies;
use crate::command_init::template::MyTemplate;
use crate::constants::file_name::FileName;
use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::shared_write_template_file::shared_write_template_file;
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
    let ui_config = UiConfig::default();

    let ui_config_toml = toml::to_string_pretty(&ui_config)
        .map_err(|e| CliError::config(&format!("Failed to serialize UiConfig: {e}")))?;
    INIT_TEMPLATE_FILE(FileName::UI_CONFIG_TOML, &ui_config_toml).await?;
    INIT_TEMPLATE_FILE(FileName::PACKAGE_JSON, MyTemplate::PACKAGE_JSON).await?;
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
