use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::task_spinner::TaskSpinner;

pub fn process_cargo_deps(cargo_deps: &[String]) -> CliResult<()> {
    // TODO. Check if cargo deps are not already in Cargo.toml before adding.

    let spinner = TaskSpinner::new("Adding crates to Cargo.toml...");

    let mut added_deps = Vec::new();
    for dep in cargo_deps {
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

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// TODO. Add a function to check if a crate is already in Cargo.toml before adding.
