use std::process::Command;

use crate::constants::others::TAILWIND_DEPENDENCIES;
use crate::shared::task_spinner::TaskSpinner;
use crate::error::{CliError, Result};

pub struct Install {}

impl Install {
    pub async fn tailwind_with_pnpm() -> Result<()> {
        for dep in TAILWIND_DEPENDENCIES {
            let message = format!("Installing dependencies...: {dep}");
            let spinner = TaskSpinner::new(&message);

            let output = Command::new("pnpm").arg("install").arg(dep).output()
                .map_err(|e| CliError::process_execution("pnpm install".to_string(), format!("Failed to execute pnpm install {}: {}", dep, e)))?;

            if output.status.success() {
                spinner.finish_success(&format!("Installed dependency: {dep}"));
            } else {
                return Err(CliError::process_execution(
                    format!("pnpm install {}", dep),
                    String::from_utf8_lossy(&output.stderr).to_string()
                ));
            }
        }
        Ok(())
    }
}
