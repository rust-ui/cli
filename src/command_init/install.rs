use std::process::Command;

use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::task_spinner::TaskSpinner;

const TAILWIND_DEPENDENCIES: [&str; 3] = ["@tailwindcss/cli", "tailwindcss", "tw-animate-css"];

pub struct Install {}

impl Install {
    pub async fn tailwind_with_pnpm() -> CliResult<()> {
        let deps_list = TAILWIND_DEPENDENCIES.join(" ");
        let message = format!("Installing TailwindCSS dependencies: {deps_list}");
        let spinner = TaskSpinner::new(&message);

        let mut cmd = Command::new("pnpm");
        cmd.arg("install");

        for dep in TAILWIND_DEPENDENCIES {
            cmd.arg(dep);
        }

        let output = cmd.output().map_err(|_| CliError::npm_install_failed())?;

        if output.status.success() {
            spinner.finish_success("All TailwindCSS dependencies installed successfully");
        } else {
            return Err(CliError::npm_install_failed());
        }

        Ok(())
    }
}
