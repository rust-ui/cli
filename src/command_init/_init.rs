use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use clap::{Arg, Command};
use dialoguer::Confirm;
use dialoguer::theme::ColorfulTheme;

const UI_CONFIG_TOML: &str = "ui_config.toml";
const PACKAGE_JSON: &str = "package.json";

use super::config::{UiConfig, add_init_crates};
use super::install::InstallType;
use super::user_input::UserInput;
use super::workspace_utils::{check_leptos_dependency, get_tailwind_input_file};
use crate::command_init::install::install_dependencies;
use crate::command_init::template::MyTemplate;
use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::task_spinner::TaskSpinner;

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

pub fn command_init() -> Command {
    Command::new("init")
        .about("Initialize the project")
        .arg(Arg::new("project_name").help("The name of the project to initialize").required(false))
        .subcommand(Command::new("run").about("Run the initialization logic"))
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

pub async fn process_init() -> CliResult<()> {
    // Check if Leptos is installed before proceeding
    if !check_leptos_dependency()? {
        return Err(CliError::config(
            "Leptos dependency not found in Cargo.toml. Please install Leptos first.",
        ));
    }

    // Get tailwind input file from Cargo.toml metadata
    let tailwind_input_file = get_tailwind_input_file()?;

    let ui_config = UiConfig::default();
    let ui_config_toml = toml::to_string_pretty(&ui_config)?;

    // ui_config.toml - always write (config file)
    write_template_file(UI_CONFIG_TOML, &ui_config_toml).await?;

    // package.json - merge with existing to preserve user dependencies
    merge_package_json(PACKAGE_JSON, MyTemplate::PACKAGE_JSON).await?;

    // tailwind.css - ask before overwriting if exists
    write_template_with_confirmation(&tailwind_input_file, MyTemplate::STYLE_TAILWIND_CSS).await?;

    add_init_crates().await?;

    UserInput::handle_index_styles().await?;

    install_dependencies(&[InstallType::Tailwind]).await?;
    Ok(())
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Write template file (always writes, no confirmation)
async fn write_template_file(file_name: &str, template: &str) -> CliResult<()> {
    let file_path = Path::new(".").join(file_name);
    let spinner = TaskSpinner::new(&format!("Writing {file_name}..."));

    write_file_content(&file_path, template)?;

    spinner.finish_success(&format!("{file_name} written."));
    Ok(())
}

/// Merge package.json with existing file to preserve user dependencies
async fn merge_package_json(file_name: &str, template: &str) -> CliResult<()> {
    let file_path = Path::new(".").join(file_name);
    let file_exists = file_path.exists();
    let spinner = TaskSpinner::new(&format!("Writing {file_name}..."));

    let content = if file_exists {
        let existing_content = fs::read_to_string(&file_path)?;
        merge_json_objects(&existing_content, template)?
    } else {
        template.to_string()
    };

    write_file_content(&file_path, &content)?;

    let action = if file_exists { "merged" } else { "written" };
    spinner.finish_success(&format!("{file_name} {action}."));
    Ok(())
}

/// Write template file with confirmation if file already exists
async fn write_template_with_confirmation(file_name: &str, template: &str) -> CliResult<()> {
    let file_path = Path::new(".").join(file_name);

    if file_path.exists() {
        let should_overwrite = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("{file_name} already exists. Overwrite?"))
            .default(false)
            .interact()
            .map_err(|err| CliError::validation(&format!("Failed to get user input: {err}")))?;

        if !should_overwrite {
            println!("⏭️  Skipping {file_name}");
            return Ok(());
        }
    }

    let spinner = TaskSpinner::new(&format!("Writing {file_name}..."));
    write_file_content(&file_path, template)?;
    spinner.finish_success(&format!("{file_name} written."));
    Ok(())
}

/* ========================================================== */
/*                     ✨ HELPERS ✨                          */
/* ========================================================== */

/// Write content to a file, creating parent directories if needed
fn write_file_content(file_path: &Path, content: &str) -> io::Result<()> {
    // Create the directory if it doesn't exist
    if let Some(dir) = file_path.parent() {
        fs::create_dir_all(dir)?;
    }

    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Merge JSON objects: template values are added to existing, preserving existing fields
fn merge_json_objects(existing: &str, template: &str) -> CliResult<String> {
    let mut existing_json: serde_json::Value = serde_json::from_str(existing)
        .map_err(|err| CliError::file_operation(&format!("Failed to parse existing JSON: {err}")))?;

    let template_json: serde_json::Value = serde_json::from_str(template)
        .map_err(|err| CliError::file_operation(&format!("Failed to parse template JSON: {err}")))?;

    if let (Some(existing_obj), Some(template_obj)) =
        (existing_json.as_object_mut(), template_json.as_object())
    {
        for (key, value) in template_obj {
            existing_obj.insert(key.clone(), value.clone());
        }
    }

    serde_json::to_string_pretty(&existing_json)
        .map_err(|err| CliError::file_operation(&format!("Failed to serialize JSON: {err}")))
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_merge_json_preserves_existing_dependencies() {
        let existing = r#"{
  "name": "my-app",
  "dependencies": {
    "axios": "^1.0.0",
    "react": "^18.0.0"
  }
}"#;
        let template = r#"{"type": "module"}"#;

        let result = merge_json_objects(existing, template).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

        // Template field added
        assert_eq!(parsed["type"], "module");
        // Existing fields preserved
        assert_eq!(parsed["name"], "my-app");
        assert_eq!(parsed["dependencies"]["axios"], "^1.0.0");
        assert_eq!(parsed["dependencies"]["react"], "^18.0.0");
    }

    #[test]
    fn test_merge_json_template_takes_precedence() {
        let existing = r#"{"type": "commonjs", "name": "app"}"#;
        let template = r#"{"type": "module"}"#;

        let result = merge_json_objects(existing, template).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

        // Template value overwrites existing
        assert_eq!(parsed["type"], "module");
        // Other existing fields preserved
        assert_eq!(parsed["name"], "app");
    }

    #[test]
    fn test_merge_json_empty_existing() {
        let existing = r#"{}"#;
        let template = r#"{"type": "module"}"#;

        let result = merge_json_objects(existing, template).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

        assert_eq!(parsed["type"], "module");
    }

    #[test]
    fn test_merge_json_complex_existing() {
        let existing = r#"{
  "name": "my-leptos-app",
  "private": true,
  "scripts": {
    "dev": "trunk serve"
  },
  "devDependencies": {
    "tailwindcss": "^4.0.0",
    "tw-animate-css": "^1.0.0"
  }
}"#;
        let template = r#"{"type": "module"}"#;

        let result = merge_json_objects(existing, template).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

        // Template field added
        assert_eq!(parsed["type"], "module");
        // All existing fields preserved
        assert_eq!(parsed["name"], "my-leptos-app");
        assert_eq!(parsed["private"], true);
        assert_eq!(parsed["scripts"]["dev"], "trunk serve");
        assert_eq!(parsed["devDependencies"]["tailwindcss"], "^4.0.0");
    }

    #[test]
    fn test_write_file_content_creates_directories() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("nested").join("dir").join("file.txt");

        write_file_content(&file_path, "test content").unwrap();

        assert!(file_path.exists());
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "test content");
    }

    #[test]
    fn test_write_file_content_overwrites() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("file.txt");

        write_file_content(&file_path, "first").unwrap();
        write_file_content(&file_path, "second").unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "second");
    }
}
