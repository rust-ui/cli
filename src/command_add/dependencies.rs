use std::collections::HashSet;
use std::path::Path;

use cargo_toml::Manifest;

use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::task_spinner::TaskSpinner;

pub fn process_cargo_deps(cargo_deps: &[String]) -> CliResult<()> {
    let spinner = TaskSpinner::new("Checking dependencies...");

    // Get existing dependencies from Cargo.toml
    let existing_deps = get_existing_dependencies()?;

    // Filter out dependencies that already exist
    let (new_deps, existing_deps_found): (Vec<_>, Vec<_>) =
        cargo_deps.iter().partition(|dep| !existing_deps.contains(*dep));

    if !existing_deps_found.is_empty() {
        let existing_str = existing_deps_found.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
        spinner.set_message(&format!("⏭️  Skipping existing dependencies: [{existing_str}]"));
    }

    if new_deps.is_empty() {
        spinner.finish_with_message("All dependencies already exist in Cargo.toml");
        return Ok(());
    }

    spinner.set_message("Adding new crates to Cargo.toml...");
    let mut added_deps = Vec::new();

    for dep in &new_deps {
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

    let dependencies_str = added_deps.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
    let finish_message = format!("Successfully added to Cargo.toml: [{dependencies_str}] !");
    spinner.finish_success(&finish_message);

    Ok(())
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

/// Check if a crate is already in Cargo.toml dependencies
fn get_existing_dependencies() -> CliResult<HashSet<String>> {
    let cargo_toml_path = Path::new("Cargo.toml");

    if !cargo_toml_path.exists() {
        return Ok(HashSet::new());
    }

    let manifest = Manifest::from_path(cargo_toml_path)?;

    let mut existing_deps = HashSet::new();

    // Check [dependencies] section
    for dep_name in manifest.dependencies.keys() {
        existing_deps.insert(dep_name.clone());
    }

    // Check [dev-dependencies] section
    for dep_name in manifest.dev_dependencies.keys() {
        existing_deps.insert(dep_name.clone());
    }

    Ok(existing_deps)
}
