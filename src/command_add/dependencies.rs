use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::task_spinner::TaskSpinner;

pub fn add_cargo_dep_to_toml(cargo_deps: &[String]) -> CliResult<()> {
    let spinner = TaskSpinner::new("Adding crates to Cargo.toml...");

    let mut added_deps = Vec::new();
    for dep in cargo_deps {
        // Skip "std" as it's a standard library and not a dependency to add
        if dep == "std" {
            continue;
        }

        // Update the spinner message to show the current crate being installed
        spinner.set_message(&format!("📦 Adding crate: {dep}"));

        // Execute the CLI command to add the dependency
        let output = std::process::Command::new("cargo")
            .arg("add")
            .arg(dep)
            .output()
            .map_err(|_| CliError::cargo_operation("Failed to execute cargo add"))?;

        if output.status.success() {
            added_deps.push(dep);
        } else {
            return Err(CliError::cargo_operation("Failed to add dependency"));
        }
    }

    if !added_deps.is_empty() {
        let dependencies_str = added_deps.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
        let finish_message = format!("Successfully added to Cargo.toml: [{dependencies_str}] !");
        spinner.finish_success(&finish_message);
    } else {
        spinner.finish_with_message("No new crates to add");
    }

    Ok(())
}
