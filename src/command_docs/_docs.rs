use std::process::Command as ProcessCommand;

use clap::Command;

use crate::shared::cli_error::{CliError, CliResult};

const DOCS_URL: &str = "https://rust-ui.com";

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

pub fn command_docs() -> Command {
    Command::new("docs").about("Open the rust-ui documentation in your browser")
}

pub fn process_docs() -> CliResult<()> {
    println!("Opening {DOCS_URL} ...");
    open_url(DOCS_URL)
}

/* ========================================================== */
/*                     ✨ HELPERS ✨                          */
/* ========================================================== */

/// Opens the given URL in the system default browser.
/// Returns the platform-specific command and arguments used — extracted for testability.
pub fn open_url(url: &str) -> CliResult<()> {
    let (program, args) = browser_command(url);

    let status = ProcessCommand::new(program)
        .args(&args)
        .status()
        .map_err(|_| CliError::validation("Failed to launch browser"))?;

    if status.success() {
        Ok(())
    } else {
        Err(CliError::validation("Browser command exited with a non-zero status"))
    }
}

/// Returns the platform command and args needed to open `url`.
/// Separated from `open_url` so it can be unit-tested without side effects.
pub fn browser_command(url: &str) -> (&'static str, Vec<String>) {
    #[cfg(target_os = "macos")]
    return ("open", vec![url.to_string()]);

    #[cfg(target_os = "linux")]
    return ("xdg-open", vec![url.to_string()]);

    #[cfg(target_os = "windows")]
    return ("cmd", vec!["/c".to_string(), "start".to_string(), url.to_string()]);

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    return ("xdg-open", vec![url.to_string()]);
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn docs_url_is_https() {
        assert!(DOCS_URL.starts_with("https://"));
    }

    #[test]
    fn docs_url_has_no_trailing_slash() {
        assert!(!DOCS_URL.ends_with('/'));
    }

    #[test]
    fn browser_command_includes_url() {
        let (_, args) = browser_command("https://example.com");
        assert!(args.iter().any(|a| a.contains("https://example.com")));
    }

    #[test]
    fn browser_command_program_is_non_empty() {
        let (program, _) = browser_command(DOCS_URL);
        assert!(!program.is_empty());
    }

    #[test]
    fn browser_command_args_are_non_empty() {
        let (_, args) = browser_command(DOCS_URL);
        assert!(!args.is_empty());
    }
}
