use std::process::Command;

use strum::AsRefStr;

use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::task_spinner::TaskSpinner;

#[derive(Debug, Clone, AsRefStr)]
enum InstallType {
    Tailwind,
}

impl InstallType {
    fn dependencies(&self) -> &'static [&'static str] {
        match self {
            Self::Tailwind => &["@tailwindcss/cli", "tailwindcss", "tw-animate-css"],
        }
    }
    
    fn name(&self) -> &str {
        self.as_ref()
    }
    
}

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
        let install_type = InstallType::Tailwind;
        let package_manager = Self::detect_package_manager();
        Self::install_with_package_manager(install_type, package_manager)
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
    
    fn install_with_package_manager(install_type: InstallType, package_manager: PackageManager) -> CliResult<()> {
        let dependencies = install_type.dependencies();
        let deps_list = dependencies.join(" ");
        let pm_name = package_manager.command();
        let type_name = install_type.name();
        let message = format!("Installing {type_name} dependencies with {pm_name}: {deps_list}");
        let spinner = TaskSpinner::new(&message);

        let mut cmd = Command::new(package_manager.command());
        cmd.arg("install");

        for dep in dependencies {
            cmd.arg(dep);
        }

        let output = cmd.output().map_err(|_| CliError::npm_install_failed())?;

        if output.status.success() {
            let success_message = format!("All {} dependencies installed successfully", install_type.name());
            spinner.finish_success(&success_message);
        } else {
            return Err(CliError::npm_install_failed());
        }

        Ok(())
    }
}
