use std::collections::{HashMap, HashSet};

use crate::shared::cli_error::CliResult;

#[derive(Debug, Clone)]
pub struct TreeParser {
    components: HashMap<String, ComponentEntry>,
}

#[derive(Debug, Clone)]
pub struct ComponentEntry {
    pub name: String,
    pub category: String,
    pub dependencies: Vec<String>,
    pub cargo_deps: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ResolvedSet {
    pub components: HashSet<String>,
    pub cargo_deps: HashSet<String>,
    pub parent_dirs: HashSet<String>,
}

impl TreeParser {
    pub fn parse_tree_md(content: &str) -> CliResult<Self> {
        let mut components = HashMap::new();
        let mut current_component: Option<ComponentEntry> = None;
        let mut dependency_stack: Vec<String> = Vec::new();

        for line in content.lines() {
            let line = line.trim();

            // Skip empty lines and code block markers
            if line.is_empty() || line.starts_with("```") {
                continue;
            }

            // Parse component lines (*)
            if let Some(line_content) = line.strip_prefix("* ") {
                // Save previous component if exists
                if let Some(component) = current_component.take() {
                    components.insert(component.name.clone(), component);
                }

                if let Some((name_part, category_part)) = line_content.rsplit_once(" (") {
                    let name = name_part.trim().to_string();
                    let category = category_part.trim_end_matches(')').to_string();

                    current_component = Some(ComponentEntry {
                        name: name.clone(),
                        category,
                        dependencies: Vec::new(),
                        cargo_deps: Vec::new(),
                    });

                    dependency_stack.clear();
                    dependency_stack.push(name);
                }
            }
            // Parse dependency lines (**)
            else if let Some(dep_content) = line.strip_prefix("** ") {
                if let Some(cargo_dep_name) = dep_content.strip_prefix("cargo: ") {
                    // Cargo dependency
                    let cargo_dep = cargo_dep_name.trim().to_string();
                    if let Some(ref mut component) = current_component {
                        component.cargo_deps.push(cargo_dep);
                    }
                } else if let Some((dep_name, _)) = dep_content.rsplit_once(" (") {
                    // Registry dependency
                    let dep_name = dep_name.trim().to_string();
                    if let Some(ref mut component) = current_component {
                        component.dependencies.push(dep_name.clone());
                    }

                    // Update dependency stack for nested dependencies
                    dependency_stack.truncate(1); // Keep only root component
                    dependency_stack.push(dep_name);
                }
            }
            // Parse nested dependency lines (***)
            else if let Some(dep_content) = line.strip_prefix("*** ") {
                if let Some(cargo_dep_name) = dep_content.strip_prefix("cargo: ") {
                    // Nested cargo dependency - add to root component
                    let cargo_dep = cargo_dep_name.trim().to_string();
                    if let Some(ref mut component) = current_component {
                        component.cargo_deps.push(cargo_dep);
                    }
                } else if let Some((dep_name, _)) = dep_content.rsplit_once(" (") {
                    // Nested registry dependency - add to root component
                    let dep_name = dep_name.trim().to_string();
                    if let Some(ref mut component) = current_component {
                        component.dependencies.push(dep_name);
                    }
                }
            }
        }

        // Save last component
        if let Some(component) = current_component {
            components.insert(component.name.clone(), component);
        }

        Ok(TreeParser { components })
    }

    pub fn get_all_component_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.components.keys().cloned().collect();
        names.sort();
        names
    }

    pub fn get_dependencies_map(&self) -> HashMap<String, Vec<String>> {
        self.components
            .iter()
            .map(|(name, entry)| (name.clone(), entry.dependencies.clone()))
            .collect()
    }

    pub fn resolve_dependencies(&self, user_components: &[String]) -> CliResult<ResolvedSet> {
        let mut resolved_components = HashSet::new();
        let mut resolved_cargo_deps = HashSet::new();
        let mut resolved_parent_dirs = HashSet::new();

        // Process each user component
        for component_name in user_components {
            if let Some(component_entry) = self.components.get(component_name) {
                // Add the component itself
                resolved_components.insert(component_name.clone());
                resolved_parent_dirs.insert(component_entry.category.clone());

                // Add its direct dependencies
                for dep in &component_entry.dependencies {
                    resolved_components.insert(dep.clone());

                    // Add parent dir for dependency
                    if let Some(dep_entry) = self.components.get(dep) {
                        resolved_parent_dirs.insert(dep_entry.category.clone());
                    }
                }

                // Add cargo dependencies
                for cargo_dep in &component_entry.cargo_deps {
                    resolved_cargo_deps.insert(cargo_dep.clone());
                }
            } else {
                println!("⚠️  Component '{}' not found in registry. Skipping...", component_name);
            }
        }

        println!("📦 Final set of resolved components: {:?}", resolved_components);
        println!("📦 Final set of cargo dependencies: {:?}", resolved_cargo_deps);

        Ok(ResolvedSet {
            components: resolved_components,
            cargo_deps: resolved_cargo_deps,
            parent_dirs: resolved_parent_dirs,
        })
    }
}
