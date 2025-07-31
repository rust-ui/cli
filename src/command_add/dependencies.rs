use std::collections::{HashMap, HashSet};

use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::task_spinner::TaskSpinner;

use super::components::{MyComponent, ResolvedComponent};

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                   🔍 CIRCULAR DEPENDENCY DETECTOR           */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(Default)]
pub struct CircularDependencyDetector {
    visited: HashSet<String>,
}

impl CircularDependencyDetector {
    pub fn check_and_visit(&mut self, component_name: &str) -> CliResult<()> {
        if !self.visited.insert(component_name.to_string()) {
            return Err(CliError::circular_dependency(component_name));
        }
        Ok(())
    }

    pub fn mark_completed(&mut self, component_name: &str) {
        self.visited.remove(component_name);
    }
}


/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     📦 RESOLUTION CACHE                     */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(Default)]
pub struct ResolutionCache {
    resolved_components: HashMap<String, ResolvedComponent>,
}

impl ResolutionCache {
    pub fn get(&self, component_name: &str) -> Option<&ResolvedComponent> {
        self.resolved_components.get(component_name)
    }

    pub fn insert(&mut self, component_name: String, resolved: ResolvedComponent) {
        self.resolved_components.insert(component_name, resolved);
    }

    pub fn get_dependencies(&self, component_name: &str) -> Option<(HashSet<String>, HashSet<String>)> {
        self.resolved_components.get(component_name).map(|resolved| {
            (
                resolved.resolved_registry_dependencies.clone(),
                resolved.resolved_cargo_dependencies.clone(),
            )
        })
    }

    pub fn get_all_resolved(&self) -> &HashMap<String, ResolvedComponent> {
        &self.resolved_components
    }
}


/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                   🗂️ COMPONENT REGISTRY                     */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub struct ComponentRegistry {
    component_map: HashMap<String, MyComponent>,
}

impl ComponentRegistry {
    pub fn new(components: &[MyComponent]) -> Self {
        let component_map = components
            .iter()
            .map(|c| (c.name.clone(), c.clone()))
            .collect();
        
        Self { component_map }
    }

    pub fn get_component(&self, name: &str) -> Option<&MyComponent> {
        self.component_map.get(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.component_map.contains_key(name)
    }

    pub fn validate_components(&self, component_names: &[String]) -> Vec<String> {
        component_names
            .iter()
            .filter(|name| !self.contains(name))
            .cloned()
            .collect()
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                   🧩 DEPENDENCY RESOLVER                    */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub struct DependencyResolver {
    registry: ComponentRegistry,
    cache: ResolutionCache,
}

impl DependencyResolver {
    pub fn new(registry: ComponentRegistry) -> Self {
        Self {
            registry,
            cache: ResolutionCache::default(),
        }
    }

    pub fn resolve_user_components(&mut self, user_components: &[String]) -> CliResult<HashMap<String, ResolvedComponent>> {
        let invalid_components = self.registry.validate_components(user_components);
        
        for invalid in &invalid_components {
            println!("⚠️  Skipping component '{invalid}' - not found in registry");
        }

        let mut cycle_detector = CircularDependencyDetector::default();
        
        for component_name in user_components {
            if self.registry.contains(component_name) {
                self.resolve_component_recursive(component_name, &mut cycle_detector)?;
            }
        }

        Ok(self.cache.get_all_resolved().clone())
    }

    fn resolve_component_recursive(
        &mut self,
        component_name: &str,
        cycle_detector: &mut CircularDependencyDetector,
    ) -> CliResult<(HashSet<String>, HashSet<String>)> {
        if let Some(dependencies) = self.cache.get_dependencies(component_name) {
            return Ok(dependencies);
        }

        cycle_detector.check_and_visit(component_name)?;

        let component = self.registry
            .get_component(component_name)
            .ok_or_else(|| CliError::component_not_found(component_name))?
            .clone();

        let mut resolved_registry_dependencies = HashSet::new();
        let mut resolved_cargo_dependencies = HashSet::new();

        for cargo_dep in &component.cargo_dependencies {
            resolved_cargo_dependencies.insert(cargo_dep.clone());
        }

        for dep_name in &component.registry_dependencies {
            resolved_registry_dependencies.insert(dep_name.clone());

            let (transitive_registry_deps, transitive_cargo_deps) =
                self.resolve_component_recursive(dep_name, cycle_detector)?;

            resolved_registry_dependencies.extend(transitive_registry_deps);
            resolved_cargo_dependencies.extend(transitive_cargo_deps);
        }

        cycle_detector.mark_completed(component_name);

        let resolved_component = ResolvedComponent {
            component,
            resolved_registry_dependencies: resolved_registry_dependencies.clone(),
            resolved_cargo_dependencies: resolved_cargo_dependencies.clone(),
        };

        self.cache.insert(component_name.to_string(), resolved_component);

        Ok((resolved_registry_dependencies, resolved_cargo_dependencies))
    }
}

pub fn all_tree_resolved(
    user_components: Vec<String>,
    vec_components_from_index: &[MyComponent],
) -> CliResult<HashMap<String, ResolvedComponent>> {
    let component_registry = ComponentRegistry::new(vec_components_from_index);
    let mut dependency_resolver = DependencyResolver::new(component_registry);
    dependency_resolver.resolve_user_components(&user_components)
}

pub fn get_all_resolved_components(resolved: &HashMap<String, ResolvedComponent>) -> Vec<String> {
    collect_and_sort(resolved, |component| {
        let mut items = vec![component.component.name.clone()];
        items.extend(component.resolved_registry_dependencies.iter().cloned());
        items
    })
}

pub fn get_all_resolved_parent_dirs(resolved: &HashMap<String, ResolvedComponent>) -> Vec<String> {
    collect_and_sort(resolved, |component| vec![component.component.parent_dir.clone()])
}

pub fn get_all_resolved_cargo_dependencies(resolved: &HashMap<String, ResolvedComponent>) -> Vec<String> {
    collect_and_sort(resolved, |component| component.resolved_cargo_dependencies.iter().cloned().collect::<Vec<_>>())
}

pub fn print_dependency_tree(resolved: &HashMap<String, ResolvedComponent>) {
    DependencyTreePrinter::print_tree(resolved);
}

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

// Helper function to collect items from resolved components and return sorted vector
fn collect_and_sort<T, F>(resolved: &HashMap<String, ResolvedComponent>, extractor: F) -> Vec<String>
where
    F: Fn(&ResolvedComponent) -> T,
    T: IntoIterator<Item = String>,
{
    let mut items = HashSet::new();
    
    for component in resolved.values() {
        for item in extractor(component) {
            items.insert(item);
        }
    }
    
    let mut result: Vec<String> = items.into_iter().collect();
    result.sort();
    result
}


/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                   🌳 DEPENDENCY TREE PRINTER               */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub struct DependencyTreePrinter;

impl DependencyTreePrinter {
    pub fn print_tree(resolved: &HashMap<String, ResolvedComponent>) {
        println!("Dependency Tree Resolution:");

        let dependent_components = Self::find_dependent_components(resolved);

        for name in resolved.keys() {
            if !dependent_components.contains(name) {
                Self::print_component_tree(name, resolved, resolved, 0);
            }
        }
    }

    fn find_dependent_components(resolved: &HashMap<String, ResolvedComponent>) -> HashSet<String> {
        let mut dependent_components = HashSet::new();
        for resolved_comp in resolved.values() {
            for dep in &resolved_comp.resolved_registry_dependencies {
                dependent_components.insert(dep.clone());
            }
        }
        dependent_components
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

            let filtered_cargo_deps: Vec<&String> = component
                .component
                .cargo_dependencies
                .iter()
                .filter(|&dep| dep != "std")
                .collect();

            if !filtered_cargo_deps.is_empty() {
                let cargo_indent = "  ".repeat(depth + 1);
                println!("{cargo_indent}└─ Cargo Dependencies:");

                let mut cargo_deps = filtered_cargo_deps;
                cargo_deps.sort();

                for cargo_dep in cargo_deps {
                    let cargo_dep_indent = "  ".repeat(depth + 2);
                    println!("{cargo_dep_indent}└─ {cargo_dep}");
                }
            }

            let mut deps: Vec<&String> = component.component.registry_dependencies.iter().collect();
            deps.sort();

            for dep_name in deps {
                if all_resolved.contains_key(dep_name) {
                    Self::print_component_tree(dep_name, all_resolved, all_resolved, depth + 1);
                } else {
                    let indent = "  ".repeat(depth + 1);
                    println!("{indent}└─ {dep_name} (external)");
                }
            }
        }
    }
}

