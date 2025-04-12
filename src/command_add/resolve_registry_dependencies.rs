use std::collections::{HashMap, HashSet};

use crate::command_add::models::{MyComponent, ResolvedComponent};

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

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
