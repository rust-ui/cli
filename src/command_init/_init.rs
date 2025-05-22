use clap::{Arg, Command};
use indicatif::ProgressBar;
use std::time::Duration;

use super::config::{UiConfig, add_init_crates};
use super::{install::Install, user_input::UserInput};
use crate::constants::commands::{InitCommand, MyCommand};
use crate::constants::file_name::FILE_NAME;
use crate::constants::template::MyTemplate;
use crate::constants::{others::SPINNER_UPDATE_DURATION, paths::RELATIVE_PATH_PROJECT_DIR};
use crate::shared::shared_write_template_file::shared_write_template_file;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub fn command_init() -> Command {
    Command::new(MyCommand::INIT)
        .about(InitCommand::ABOUT)
        .arg(
            Arg::new(InitCommand::PROJECT_NAME)
                .help(InitCommand::HELP)
                .required(false),
        )
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
    let ui_config = UiConfig::default();

    let ui_config_toml = match toml::to_string_pretty(&ui_config) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error serializing UiConfig: {}", e);
            return;
        }
    };
    INIT_TEMPLATE_FILE(FILE_NAME::UI_CONFIG_TOML, &ui_config_toml).await;
    INIT_TEMPLATE_FILE(FILE_NAME::PACKAGE_JSON, MyTemplate::PACKAGE_JSON).await;
    INIT_TEMPLATE_FILE(&ui_config.tailwind_input_file, MyTemplate::STYLE_TAILWIND_CSS).await;
    INIT_TEMPLATE_FILE(FILE_NAME::TAILWIND_CONFIG_JS, MyTemplate::TAILWIND_CONFIG).await;

    add_init_crates().await;

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
    let file_path = format!("{RELATIVE_PATH_PROJECT_DIR}/{file_name}");

    // if !shared_check_file_exist_and_ask_overwrite(&file_path, file_name_ext).await {
    //     return;
    // }

    let spinner: ProgressBar = ProgressBar::new_spinner();
    spinner.set_message("Writing to file...");
    spinner.enable_steady_tick(Duration::from_millis(SPINNER_UPDATE_DURATION));

    let _ = shared_write_template_file(&file_path, &spinner, template).await;

    let finish_message = format!("✔️ Writing {file_name} complete.");
    spinner.finish_with_message(finish_message);
}
