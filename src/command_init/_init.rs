use clap::{Arg, Command};
use indicatif::ProgressBar;
use std::time::Duration;

use super::{config::Config, install::Install, user_input::UserInput};
use crate::constants::commands::{COMMAND, INIT};
use crate::constants::{
    file_names::{COMPONENTS_TOML, PACKAGE_JSON, STYLE_SLASH_TAILWIND_CSS, TAILWIND_CONFIG_JS},
    others::SPINNER_UPDATE_DURATION,
    paths::RELATIVE_PATH_PROJECT_DIR,
    templates_init::{
        TEMPLATE_COMPONENTS_TOML, TEMPLATE_PACKAGE_JSON, TEMPLATE_STYLE_TAILWIND_CSS, TEMPLATE_TAILWIND_CONFIG,
    },
};
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
    INIT_TEMPLATE_FILE(PACKAGE_JSON, TEMPLATE_PACKAGE_JSON).await;
    INIT_TEMPLATE_FILE(COMPONENTS_TOML, TEMPLATE_COMPONENTS_TOML).await;
    INIT_TEMPLATE_FILE(STYLE_SLASH_TAILWIND_CSS, TEMPLATE_STYLE_TAILWIND_CSS).await;
    INIT_TEMPLATE_FILE(TAILWIND_CONFIG_JS, TEMPLATE_TAILWIND_CONFIG).await;

    Config::handle_cargo_toml().await;
    Config::handle_config_schema().await;
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
