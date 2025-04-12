use std::{fs, time::Duration};

use indicatif::ProgressBar;

use crate::constants::others::SPINNER_UPDATE_DURATION;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub fn add_cargo_dep_to_toml(cargo_deps: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    // Find Cargo.toml file in the current directory or parent directories
    let cargo_toml_path = find_cargo_toml()?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Adding crates to Cargo.toml...");
    spinner.enable_steady_tick(Duration::from_millis(SPINNER_UPDATE_DURATION));

    // Read the current Cargo.toml content
    let mut cargo_toml_content = fs::read_to_string(&cargo_toml_path)?;

    // Check if dependencies section exists
    if !cargo_toml_content.contains("[dependencies]") {
        cargo_toml_content.push_str("\n[dependencies]\n");
    }

    // Add each dependency using the CLI command
    let mut added_deps = Vec::new();
    for dep in cargo_deps {
        // Skip "std" as it's a standard library and not a dependency to add
        if dep == "std" {
            continue;
        }

        // Update the spinner message to show the current crate being installed
        spinner.set_message(format!("📦 Adding crate: {}", dep));

        // Execute the CLI command to add the dependency
        let output = std::process::Command::new("cargo").arg("add").arg(dep).output()?;

        if output.status.success() {
            added_deps.push(dep);
        } else {
            eprintln!(
                "Failed to add dependency {}: {}",
                dep,
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    // Only write to the file if we've added new dependencies
    if !added_deps.is_empty() {
        let dependencies_str = added_deps
            .iter()
            .map(|dep| dep.as_str())
            .collect::<Vec<&str>>()
            .join(", ");
        let finish_message = format!("✔️ Successfully added to Cargo.toml: [{}] !", dependencies_str);
        spinner.finish_with_message(finish_message);
    } else {
        spinner.finish_with_message("No new crates to add");
    }

    Ok(())
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

fn find_cargo_toml() -> Result<String, Box<dyn std::error::Error>> {
    // Start with the current directory
    let mut current_dir = std::env::current_dir()?;

    loop {
        let cargo_toml_path = current_dir.join("Cargo.toml");

        if cargo_toml_path.exists() {
            return Ok(cargo_toml_path.to_string_lossy().to_string());
        }

        // Move to the parent directory
        if !current_dir.pop() {
            // No parent directory (we're at the root)
            break;
        }
    }

    Err("Could not find Cargo.toml in the current directory or any parent directories".into())
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🧪 TESTS 🧪                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn make_sure_we_do_not_add_registry_dependencies_twice() {
        // Setup: Use the existing FakeCargo.toml file for testing
        let test_cargo_toml_path = "FakeCargo.toml";

        // Read the initial content of FakeCargo.toml to check existing dependencies
        let initial_content = fs::read_to_string(test_cargo_toml_path).expect("Unable to read test Cargo.toml");
        let existing_dependencies: Vec<&str> = initial_content
            .lines()
            .filter_map(|line| {
                if line.trim().starts_with('[') || line.trim().is_empty() {
                    None
                } else {
                    Some(line.trim().split_whitespace().next().unwrap())
                }
            })
            .collect();

        // Test: Add a new dependency
        let new_dependencies = vec!["serde".to_string(), "reqwest".to_string()];
        add_cargo_dep_to_toml(&new_dependencies).expect("Failed to update Cargo.toml");

        // Verify: Check if the dependencies were added
        let updated_content = fs::read_to_string(test_cargo_toml_path).expect("Unable to read test Cargo.toml");

        // Assert that new dependencies are added and not duplicated
        for dep in new_dependencies {
            assert!(updated_content.contains(&dep), "Dependency {} was not added", dep);
        }

        // Assert that existing dependencies are not duplicated
        for dep in existing_dependencies {
            assert!(
                updated_content.matches(dep).count() == 1,
                "Dependency {} was added twice",
                dep
            );
        }
    }
}
