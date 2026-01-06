use heck::ToSnakeCase;
use strum::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, EnumString, EnumIter)]
#[strum(serialize_all = "lowercase")]
pub enum ComponentType {
    Ui,
    Demos,
    Hooks,
    Extensions,
}

impl ComponentType {
    /// Get the directory path for this component type
    pub fn to_path(&self) -> String {
        self.to_string().to_snake_case()
    }

    /// Determine component type from component name patterns
    pub fn from_component_name(component_name: &str) -> Self {
        if component_name.starts_with("demo_") {
            Self::Demos
        } else if component_name.starts_with("use_") {
            Self::Hooks
        } else if component_name.contains("extension") {
            Self::Extensions
        } else {
            Self::Ui
        }
    }
}
