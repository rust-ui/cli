#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("🔸 Component '{name}' not found in registry")]
    ComponentNotFound { name: String },

    #[error("🔸 Circular dependency detected involving component '{name}'")]
    CircularDependency { name: String },

    #[error("🔸 Invalid component name '{name}': {reason}")]
    InvalidComponentName { name: String, reason: String },

    #[error("🔸 Failed to fetch registry data: {message}")]
    RegistryFetch { message: String },

    #[error("🔸 Network request failed: {source}")]
    Network {
        #[from]
        source: reqwest::Error,
    },

    #[error("🔸 File operation failed: {message}")]
    FileOperation { message: String },

    #[error("🔸 IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },

    #[error("🔸 Configuration error: {message}")]
    Config { message: String },

    #[error("🔸 Failed to parse TOML configuration: {source}")]
    TomlParse {
        #[from]
        source: toml::de::Error,
    },

    #[error("🔸 Failed to serialize TOML configuration: {source}")]
    TomlSerialize {
        #[from]
        source: toml::ser::Error,
    },

    #[error("🔸 JSON parsing error: {source}")]
    JsonParse {
        #[from]
        source: serde_json::Error,
    },

    #[error("🔸 Process execution failed: {command} - {message}")]
    ProcessExecution { command: String, message: String },

    #[error("🔸 Git operation failed: {operation} - {message}")]
    GitOperation { operation: String, message: String },

    #[error("🔸 Cargo operation failed: {message}")]
    CargoOperation { message: String },

    #[error("🔸 Path validation error: {path} - {reason}")]
    InvalidPath { path: String, reason: String },

    #[error("🔸 Missing required dependency: {dependency}")]
    MissingDependency { dependency: String },

    #[error("🔸 Validation error: {message}")]
    Validation { message: String },

    #[error("🔸 Template processing error: {message}")]
    Template { message: String },

    #[error("🔸 Registry index is malformed: {reason}")]
    MalformedRegistry { reason: String },

    #[error("🔸 Component dependency resolution failed: {message}")]
    DependencyResolution { message: String },
}

impl CliError {
    pub fn component_not_found(name: &str) -> Self {
        Self::ComponentNotFound { name: name.to_string() }
    }

    pub fn circular_dependency(name: &str) -> Self {
        Self::CircularDependency { name: name.to_string() }
    }

    pub fn invalid_component_name(name: &str, reason: &str) -> Self {
        Self::InvalidComponentName {
            name: name.to_string(),
            reason: reason.to_string(),
        }
    }

    pub fn registry_fetch(message: &str) -> Self {
        Self::RegistryFetch {
            message: message.to_string(),
        }
    }

    pub fn file_operation(message: &str) -> Self {
        Self::FileOperation {
            message: message.to_string(),
        }
    }

    pub fn config(message: &str) -> Self {
        Self::Config {
            message: message.to_string(),
        }
    }

    pub fn process_execution(command: &str, message: &str) -> Self {
        Self::ProcessExecution {
            command: command.to_string(),
            message: message.to_string(),
        }
    }

    pub fn git_operation(operation: &str, message: &str) -> Self {
        Self::GitOperation {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    pub fn cargo_operation(message: &str) -> Self {
        Self::CargoOperation {
            message: message.to_string(),
        }
    }

    pub fn invalid_path(path: &str, reason: &str) -> Self {
        Self::InvalidPath {
            path: path.to_string(),
            reason: reason.to_string(),
        }
    }

    pub fn missing_dependency(dependency: &str) -> Self {
        Self::MissingDependency {
            dependency: dependency.to_string(),
        }
    }

    pub fn validation(message: &str) -> Self {
        Self::Validation {
            message: message.to_string(),
        }
    }

    pub fn template(message: &str) -> Self {
        Self::Template {
            message: message.to_string(),
        }
    }

    pub fn malformed_registry(reason: &str) -> Self {
        Self::MalformedRegistry {
            reason: reason.to_string(),
        }
    }

    pub fn dependency_resolution(message: &str) -> Self {
        Self::DependencyResolution {
            message: message.to_string(),
        }
    }
}

pub type CliResult<T> = std::result::Result<T, CliError>;

impl From<anyhow::Error> for CliError {
    fn from(err: anyhow::Error) -> Self {
        CliError::Validation {
            message: err.to_string(),
        }
    }
}
