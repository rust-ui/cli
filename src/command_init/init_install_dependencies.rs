use indicatif::ProgressBar;
use std::{process::Command, time::Duration};

use crate::constants::others::{INIT_DEPENDENCIES, SPINNER_UPDATE_DURATION};

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn init_pnpm_install_dependencies() {
    let spinner = ProgressBar::new_spinner();

    for dep in INIT_DEPENDENCIES {
        let message = format!("Installing dependencies...: {}", dep);
        spinner.set_message(message);
        spinner.enable_steady_tick(Duration::from_millis(SPINNER_UPDATE_DURATION));

        let output = Command::new("pnpm").arg("install").arg(dep).output();

        match output {
            Ok(_) => spinner.finish_with_message(format!("✔️ Installed dependency: {}", dep)),
            Err(_) => spinner.finish_with_message(format!("🔸 Failed to install: {}", dep)),
        }
    }
    spinner.finish();
}
