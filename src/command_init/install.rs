use std::process::Command;

use crate::constants::others::TAILWIND_DEPENDENCIES;
use crate::shared::task_spinner::TaskSpinner;
use crate::shared::error::{CliError, Result};

pub struct Install {}

impl Install {
    pub async fn tailwind_with_pnpm() -> Result<()> {
        let deps_list = TAILWIND_DEPENDENCIES.join(" ");
        let message = format!("Installing TailwindCSS dependencies: {deps_list}");
        let spinner = TaskSpinner::new(&message);

        let mut cmd = Command::new("pnpm");
        cmd.arg("install");
        
        for dep in TAILWIND_DEPENDENCIES {
            cmd.arg(dep);
        }

        let output = cmd.output()
            .map_err(|e| CliError::process_execution("pnpm install".to_string(), format!("Failed to execute pnpm install: {e}")))?;

        if output.status.success() {
            spinner.finish_success("All TailwindCSS dependencies installed successfully");
        } else {
            return Err(CliError::process_execution(
                format!("pnpm install {deps_list}"),
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }
        
        Ok(())
    }
}
