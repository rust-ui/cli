use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MyComponent {
    pub name: String,
    pub registry_dependencies: Vec<String>,
    pub cargo_dependencies: Vec<String>,
    #[serde(rename = "type")]
    pub component_type: String,
    #[serde(rename = "parent_dir")]
    pub parent_dir: String,
}

#[derive(Debug)]
pub struct ResolvedComponent {
    pub component: MyComponent,
    pub resolved_registry_dependencies: HashSet<String>, // All dependencies including transitive ones
    pub resolved_cargo_dependencies: HashSet<String>, // All cargo dependencies including those from transitive dependencies
}
