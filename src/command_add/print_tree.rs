use std::collections::{HashMap, HashSet};

use crate::command_add::models::ResolvedComponent;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

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
