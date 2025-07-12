
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
    pub fn component_not_found(name: impl Into<String>) -> Self {
        Self::ComponentNotFound { name: name.into() }
    }

    pub fn circular_dependency(name: impl Into<String>) -> Self {
        Self::CircularDependency { name: name.into() }
    }

    pub fn invalid_component_name(name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidComponentName {
            name: name.into(),
            reason: reason.into(),
        }
    }

    pub fn registry_fetch(message: impl Into<String>) -> Self {
        Self::RegistryFetch {
            message: message.into(),
        }
    }

    pub fn file_operation(message: impl Into<String>) -> Self {
        Self::FileOperation {
            message: message.into(),
        }
    }

    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    pub fn process_execution(command: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ProcessExecution {
            command: command.into(),
            message: message.into(),
        }
    }

    pub fn git_operation(operation: impl Into<String>, message: impl Into<String>) -> Self {
        Self::GitOperation {
            operation: operation.into(),
            message: message.into(),
        }
    }

    pub fn cargo_operation(message: impl Into<String>) -> Self {
        Self::CargoOperation {
            message: message.into(),
        }
    }

    pub fn invalid_path(path: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidPath {
            path: path.into(),
            reason: reason.into(),
        }
    }

    pub fn missing_dependency(dependency: impl Into<String>) -> Self {
        Self::MissingDependency {
            dependency: dependency.into(),
        }
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }

    pub fn template(message: impl Into<String>) -> Self {
        Self::Template {
            message: message.into(),
        }
    }

    pub fn malformed_registry(reason: impl Into<String>) -> Self {
        Self::MalformedRegistry {
            reason: reason.into(),
        }
    }

    pub fn dependency_resolution(message: impl Into<String>) -> Self {
        Self::DependencyResolution {
            message: message.into(),
        }
    }
}

pub type Result<T> = std::result::Result<T, CliError>;

impl From<anyhow::Error> for CliError {
    fn from(err: anyhow::Error) -> Self {
        CliError::Validation {
            message: err.to_string(),
        }
    }
}