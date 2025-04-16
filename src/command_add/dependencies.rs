use indicatif::ProgressBar;
use std::collections::{HashMap, HashSet};
use std::{fs, time::Duration};

use crate::constants::others::SPINNER_UPDATE_DURATION;

use super::components::{MyComponent, ResolvedComponent};

// TODO. Should distinguish clearly between cargo dependencies and registry dependencies.

pub struct Dependencies {}

impl Dependencies {
    pub fn all_tree_resolved(
        user_components: Vec<String>,
        vec_components_from_index: &[MyComponent],
    ) -> HashMap<String, ResolvedComponent> {
        let component_map: HashMap<String, MyComponent> = vec_components_from_index
            .iter()
            .map(|c| (c.name.clone(), c.clone()))
            .collect();

        let resolved = resolve_all_dependencies(&component_map, &user_components).unwrap();

        resolved
    }

    pub fn get_all_resolved_components(resolved: &HashMap<String, ResolvedComponent>) -> Vec<String> {
        let mut all_components = HashSet::new();

        // Add all the resolved components
        for name in resolved.keys() {
            all_components.insert(name.clone());
        }

        // Add all their dependencies
        for (_, component) in resolved {
            for dep in &component.resolved_registry_dependencies {
                all_components.insert(dep.clone());
            }
        }

        // Convert to sorted vector for consistent output
        let mut result: Vec<String> = all_components.into_iter().collect();
        result.sort();
        result
    }

    pub fn get_all_resolved_parent_dirs(resolved: &HashMap<String, ResolvedComponent>) -> Vec<String> {
        let mut all_parent_dirs = HashSet::new();

        // Add all the resolved component types
        for (_, component) in resolved {
            all_parent_dirs.insert(component.component.parent_dir.clone());
        }

        // Convert to sorted vector for consistent output
        let mut result: Vec<String> = all_parent_dirs.into_iter().collect();
        result.sort();
        result
    }

    pub fn get_all_resolved_cargo_dependencies(resolved: &HashMap<String, ResolvedComponent>) -> Vec<String> {
        let mut all_cargo_deps = HashSet::new();

        // Add all cargo dependencies from all components
        for (_, component) in resolved {
            for dep in &component.resolved_cargo_dependencies {
                all_cargo_deps.insert(dep.clone());
            }
        }

        // Convert to sorted vector for consistent output
        let mut result: Vec<String> = all_cargo_deps.into_iter().collect();
        result.sort();
        result
    }

    //

    pub fn print_dependency_tree(resolved: &HashMap<String, ResolvedComponent>) {
        println!("Dependency Tree Resolution:");

        // Find components that are direct targets (not dependencies of other resolved components)
        let mut dependent_components = HashSet::new();
        for (_, resolved_comp) in resolved {
            for dep in &resolved_comp.resolved_registry_dependencies {
                dependent_components.insert(dep.clone());
            }
        }

        // Print each target component's tree
        for (name, _) in resolved {
            // Only print the top-level components (not dependencies of other resolved components)
            // Or, remove this condition to print all resolved components at top level
            if !dependent_components.contains(name) {
                print_component_tree(name, resolved, resolved, 0);
            }
        }
    }

    //

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
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

fn resolve_all_dependencies(
    component_map: &HashMap<String, MyComponent>,
    user_components: &[String],
) -> Result<HashMap<String, ResolvedComponent>, Box<dyn std::error::Error>> {
    // Map to store resolved components
    let mut resolved_components: HashMap<String, ResolvedComponent> = HashMap::new();

    // Process only the selected components
    for component_name in user_components {
        if !component_map.contains_key(component_name) {
            return Err(format!("Target component '{}' not found in index", component_name).into());
        }

        resolve_component_recursive(
            component_name,
            component_map,
            &mut resolved_components,
            &mut HashSet::new(),
        )?;
    }

    Ok(resolved_components)
}

fn resolve_component_recursive(
    component_name: &str,
    component_map: &HashMap<String, MyComponent>,
    resolved_components: &mut HashMap<String, ResolvedComponent>,
    visited: &mut HashSet<String>,
) -> Result<(HashSet<String>, HashSet<String>), Box<dyn std::error::Error>> {
    // Return cached result if already processed
    if let Some(resolved) = resolved_components.get(component_name) {
        return Ok((
            resolved.resolved_registry_dependencies.clone(),
            resolved.resolved_cargo_dependencies.clone(),
        ));
    }

    // Prevent infinite recursion
    if !visited.insert(component_name.to_string()) {
        return Err(format!("Circular dependency detected involving '{}'", component_name).into());
    }

    // Get component or return error if not found
    let component = match component_map.get(component_name) {
        Some(c) => c,
        None => return Err(format!("Component '{}' not found", component_name).into()),
    };

    // Collect all dependencies recursively
    let mut resolved_registry_dependencies = HashSet::new();
    let mut resolved_cargo_dependencies = HashSet::new();

    // Add direct cargo dependencies
    for cargo_dep in &component.cargo_dependencies {
        resolved_cargo_dependencies.insert(cargo_dep.clone());
    }

    // Add direct registry dependencies and their transitive dependencies
    for dep_name in &component.registry_dependencies {
        resolved_registry_dependencies.insert(dep_name.clone());

        // Add transitive dependencies (both registry and cargo)
        let (transitive_registry_deps, transitive_cargo_deps) =
            resolve_component_recursive(dep_name, component_map, resolved_components, visited)?;

        for trans_dep in transitive_registry_deps {
            resolved_registry_dependencies.insert(trans_dep);
        }

        for cargo_dep in transitive_cargo_deps {
            resolved_cargo_dependencies.insert(cargo_dep);
        }
    }

    // Remove component from visited set as we're done with it
    visited.remove(component_name);

    // Store the resolved component
    resolved_components.insert(
        component_name.to_string(),
        ResolvedComponent {
            component: component.clone(),
            resolved_registry_dependencies: resolved_registry_dependencies.clone(),
            resolved_cargo_dependencies: resolved_cargo_dependencies.clone(),
        },
    );

    Ok((resolved_registry_dependencies, resolved_cargo_dependencies))
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

fn print_component_tree(
    component_name: &str,
    all_resolved: &HashMap<String, ResolvedComponent>,
    current_branch: &HashMap<String, ResolvedComponent>,
    depth: usize,
) {
    if let Some(component) = current_branch.get(component_name) {
        let indent = "  ".repeat(depth);
        println!("{}└─ {} ({})", indent, component_name, component.component.parent_dir);

        // TODO. Shortfix to remove std. I don't know where it comes from.
        let filtered_cargo_deps: Vec<&String> = component
            .component
            .cargo_dependencies
            .iter()
            .filter(|&dep| dep != "std")
            .collect();

        if !filtered_cargo_deps.is_empty() {
            let cargo_indent = "  ".repeat(depth + 1);
            println!("{}└─ Cargo Dependencies:", cargo_indent);

            // Sort cargo dependencies for consistent output
            let mut cargo_deps = filtered_cargo_deps;
            cargo_deps.sort();

            for cargo_dep in cargo_deps {
                let cargo_dep_indent = "  ".repeat(depth + 2);
                println!("{}└─ {}", cargo_dep_indent, cargo_dep);
            }
        }

        // Sort registry dependencies for consistent output
        let mut deps: Vec<&String> = component.component.registry_dependencies.iter().collect();
        deps.sort();

        for dep_name in deps {
            // Only print dependency if it's in our resolved set
            if all_resolved.contains_key(dep_name) {
                print_component_tree(dep_name, all_resolved, all_resolved, depth + 1);
            } else {
                // This is a dependency that wasn't fully resolved (part of another branch)
                let indent = "  ".repeat(depth + 1);
                println!("{}└─ {} (external)", indent, dep_name);
            }
        }
    }
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fs;

//     #[test]
//     fn make_sure_we_do_not_add_registry_dependencies_twice() {
//         // Setup: Use the existing FakeCargo.toml file for testing
//         let test_cargo_toml_path = "FakeCargo.toml";

//         // Read the initial content of FakeCargo.toml to check existing dependencies
//         let initial_content = fs::read_to_string(test_cargo_toml_path).expect("Unable to read test Cargo.toml");
//         let existing_dependencies: Vec<&str> = initial_content
//             .lines()
//             .filter_map(|line| {
//                 if line.trim().starts_with('[') || line.trim().is_empty() {
//                     None
//                 } else {
//                     Some(line.trim().split_whitespace().next().unwrap())
//                 }
//             })
//             .collect();

//         // Test: Add a new dependency
//         let new_dependencies = vec!["serde".to_string(), "reqwest".to_string()];
//         add_cargo_dep_to_toml(&new_dependencies).expect("Failed to update Cargo.toml");

//         // Verify: Check if the dependencies were added
//         let updated_content = fs::read_to_string(test_cargo_toml_path).expect("Unable to read test Cargo.toml");

//         // Assert that new dependencies are added and not duplicated
//         for dep in new_dependencies {
//             assert!(updated_content.contains(&dep), "Dependency {} was not added", dep);
//         }

//         // Assert that existing dependencies are not duplicated
//         for dep in existing_dependencies {
//             assert!(
//                 updated_content.matches(dep).count() == 1,
//                 "Dependency {} was added twice",
//                 dep
//             );
//         }
//     }
// }
