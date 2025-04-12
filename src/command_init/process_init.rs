use indicatif::ProgressBar;
use std::time::Duration;

use super::{
    handle_cargo_toml::handle_cargo_toml, handle_config_schema::handle_config_schema,
    handle_index_styles::handle_index_styles, init_install_dependencies::init_pnpm_install_dependencies,
};
use crate::constants::{
    file_names::{
        COMPONENTS_JSON, SRC_COMPONENTS_MOD_RS, SRC_COMPONENTS_UI_MOD_RS, SRC_LIB_RS, STYLE_SLASH_TAILWIND_CSS,
        TAILWIND_CONFIG_JS,
    },
    others::SPINNER_UPDATE_DURATION,
    paths::RELATIVE_PATH_PROJECT_DIR,
    templates_init::{TEMPLATE_COMPONENTS_JSON, TEMPLATE_GLOBAL_CSS, TEMPLATE_LIB_RS, TEMPLATE_TAILWIND_CONFIG},
};
use crate::shared::shared_write_template_file::shared_write_template_file;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn process_init() {
    handle_cargo_toml().await;
    handle_config_schema().await;
    handle_index_styles().await;
    init_pnpm_install_dependencies().await;

    // 1. CREATE TEMPLATES
    INIT_TEMPLATE_FILE(SRC_LIB_RS, TEMPLATE_LIB_RS).await;
    INIT_TEMPLATE_FILE(COMPONENTS_JSON, TEMPLATE_COMPONENTS_JSON).await;
    INIT_TEMPLATE_FILE(STYLE_SLASH_TAILWIND_CSS, TEMPLATE_GLOBAL_CSS).await;
    INIT_TEMPLATE_FILE(TAILWIND_CONFIG_JS, TEMPLATE_TAILWIND_CONFIG).await;

    // 2. CREATE COMPONENTS
    INIT_TEMPLATE_FILE(SRC_COMPONENTS_MOD_RS, "pub mod ui;").await;
    // TODO. └──> Handle "pub mod hooks;" etc.
    INIT_TEMPLATE_FILE(SRC_COMPONENTS_UI_MOD_RS, "").await;
}

//
/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

/// INIT TEMPLATE FILE
#[allow(non_snake_case)]
async fn INIT_TEMPLATE_FILE(file_name_ext: &str, template: &str) {
    let file_path = format!("{}/{}", RELATIVE_PATH_PROJECT_DIR, file_name_ext);

    // if !shared_check_file_exist_and_ask_overwrite(&file_path, file_name_ext).await {
    //     return;
    // }

    let spinner: ProgressBar = ProgressBar::new_spinner();
    spinner.set_message("Writing to file...");
    spinner.enable_steady_tick(Duration::from_millis(SPINNER_UPDATE_DURATION));

    let _ = shared_write_template_file(&file_path, &spinner, template).await;

    let finish_message = format!("✔️ Writing {} complete.", file_name_ext);
    spinner.finish_with_message(finish_message);
}
