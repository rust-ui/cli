#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("🔸 Component '{name}' not found in registry")]
    ComponentNotFound { name: String },

    #[error("🔸 Circular dependency detected involving component '{name}'")]
    CircularDependency { name: String },

    #[error("🔸 Registry request failed")]
    RegistryRequestFailed,


    #[error("🔸 Network request failed: {source}")]
    Network {
        #[from]
        source: reqwest::Error,
    },

    #[error("🔸 File operation failed: {message}")]
    FileOperation { message: String },

    #[error("🔸 File not found")]
    FileNotFound,

    #[error("🔸 Failed to create directory")]
    DirectoryCreateFailed,

    #[error("🔸 Failed to write file")]
    FileWriteFailed,

    #[error("🔸 Failed to read file")]
    FileReadFailed,

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

    #[error("🔸 npm install failed")]
    NpmInstallFailed,

    #[error("🔸 Git clone failed")]
    GitCloneFailed,

    #[error("🔸 Cargo operation failed: {message}")]
    CargoOperation { message: String },

    #[error("🔸 Path validation error: {path} - {reason}")]
    InvalidPath { path: String, reason: String },


    #[error("🔸 Validation error: {message}")]
    Validation { message: String },

    #[error("🔸 Registry index is malformed: {reason}")]
    MalformedRegistry { reason: String },

    #[error("🔸 Registry component missing required fields")]
    RegistryComponentMissing,

    #[error("🔸 Registry has invalid format")]
    RegistryInvalidFormat,
}

impl CliError {
    pub fn component_not_found(name: &str) -> Self {
        Self::ComponentNotFound { name: name.to_string() }
    }

    pub fn circular_dependency(name: &str) -> Self {
        Self::CircularDependency { name: name.to_string() }
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


    pub fn validation(message: &str) -> Self {
        Self::Validation {
            message: message.to_string(),
        }
    }

    pub fn malformed_registry(reason: &str) -> Self {
        Self::MalformedRegistry {
            reason: reason.to_string(),
        }
    }



    pub fn registry_request_failed() -> Self {
        Self::RegistryRequestFailed
    }

    pub fn file_not_found() -> Self {
        Self::FileNotFound
    }

    pub fn directory_create_failed() -> Self {
        Self::DirectoryCreateFailed
    }

    pub fn file_write_failed() -> Self {
        Self::FileWriteFailed
    }

    pub fn file_read_failed() -> Self {
        Self::FileReadFailed
    }

    pub fn npm_install_failed() -> Self {
        Self::NpmInstallFailed
    }

    pub fn git_clone_failed() -> Self {
        Self::GitCloneFailed
    }

    pub fn registry_component_missing() -> Self {
        Self::RegistryComponentMissing
    }

    pub fn registry_invalid_format() -> Self {
        Self::RegistryInvalidFormat
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
