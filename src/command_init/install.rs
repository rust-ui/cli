use std::process::Command;

use crate::constants::others::TAILWIND_DEPENDENCIES;
use crate::shared::task_spinner::TaskSpinner;

pub struct Install {}

impl Install {
    pub async fn tailwind_with_pnpm() -> anyhow::Result<()> {
        for dep in TAILWIND_DEPENDENCIES {
            let message = format!("Installing dependencies...: {dep}");
            let spinner = TaskSpinner::new(&message);

            let output = Command::new("pnpm").arg("install").arg(dep).output();

            match output {
                Ok(_) => spinner.finish_success(&format!("Installed dependency: {dep}")),
                Err(_) => spinner.finish_info(&format!("Failed to install: {dep}")),
            }
        }
        Ok(())
    }
}
