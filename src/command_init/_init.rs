use clap::{Arg, Command};
use indicatif::ProgressBar;
use std::time::Duration;

use super::config::{UiConfig, add_init_dependencies};
use super::{install::Install, user_input::UserInput};
use crate::constants::commands::{COMMAND, INIT};
use crate::constants::file_name::FILE_NAME;
use crate::constants::template::TEMPLATE;
use crate::constants::{others::SPINNER_UPDATE_DURATION, paths::RELATIVE_PATH_PROJECT_DIR};
use crate::shared::shared_write_template_file::shared_write_template_file;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub fn command_init() -> Command {
    Command::new(COMMAND::INIT)
        .about(INIT::ABOUT)
        .arg(Arg::new(INIT::PROJECT_NAME).help(INIT::HELP).required(false))
        .subcommand(Command::new("run").about("Run the initialization logic"))
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[allow(dead_code)]
pub async fn init_project() {
    process_init().await;
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn process_init() {
    // Create ui_config.toml file with default values in it
    let ui_config = UiConfig::default();

    INIT_TEMPLATE_FILE(FILE_NAME::APP_CONFIG_TOML, &toml::to_string_pretty(&ui_config).unwrap()).await;
    INIT_TEMPLATE_FILE(FILE_NAME::PACKAGE_JSON, TEMPLATE::PACKAGE_JSON).await;
    INIT_TEMPLATE_FILE(&ui_config.tailwind_input_file, TEMPLATE::STYLE_TAILWIND_CSS).await;
    INIT_TEMPLATE_FILE(FILE_NAME::TAILWIND_CONFIG_JS, TEMPLATE::TAILWIND_CONFIG).await;

    add_init_dependencies().await;

    UserInput::handle_index_styles().await;

    Install::tailwind_with_pnpm().await;
}

//
/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

/// INIT TEMPLATE FILE
#[allow(non_snake_case)]
async fn INIT_TEMPLATE_FILE(file_name: &str, template: &str) {
    let file_path = format!("{}/{}", RELATIVE_PATH_PROJECT_DIR, file_name);

    // if !shared_check_file_exist_and_ask_overwrite(&file_path, file_name_ext).await {
    //     return;
    // }

    let spinner: ProgressBar = ProgressBar::new_spinner();
    spinner.set_message("Writing to file...");
    spinner.enable_steady_tick(Duration::from_millis(SPINNER_UPDATE_DURATION));

    let _ = shared_write_template_file(&file_path, &spinner, template).await;

    let finish_message = format!("✔️ Writing {} complete.", file_name);
    spinner.finish_with_message(finish_message);
}
