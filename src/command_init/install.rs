use std::process::Command;

use strum::AsRefStr;

use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::task_spinner::TaskSpinner;

const TAILWIND_DEPENDENCIES: [&str; 3] = ["@tailwindcss/cli", "tailwindcss", "tw-animate-css"];

#[derive(Debug, Clone, AsRefStr)]
#[strum(serialize_all = "lowercase")]
enum PackageManager {
    Pnpm,
    Npm,
}

impl PackageManager {
    fn command(&self) -> &str {
        self.as_ref()
    }
}

pub struct Install {}

impl Install {
    pub async fn tailwind_dependencies() -> CliResult<()> {
        let package_manager = Self::detect_package_manager();
        Self::install_with_package_manager(package_manager)
    }

        fn is_pnpm_available() -> bool {
        Command::new("pnpm")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
    
    fn detect_package_manager() -> PackageManager {
        if Self::is_pnpm_available() {
            PackageManager::Pnpm
        } else {
            PackageManager::Npm
        }
    }
    
    fn install_with_package_manager(package_manager: PackageManager) -> CliResult<()> {
        let deps_list = TAILWIND_DEPENDENCIES.join(" ");
        let pm_name = package_manager.command();
        let message = format!("Installing TailwindCSS dependencies with {pm_name}: {deps_list}");
        let spinner = TaskSpinner::new(&message);

        let mut cmd = Command::new(package_manager.command());
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
