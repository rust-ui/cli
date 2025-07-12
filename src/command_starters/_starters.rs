use crate::constants::commands::{MyCommand, StartersCommand};
use clap::Command;
use dialoguer::{Select, theme::ColorfulTheme};
use std::process::{Command as ProcessCommand, Stdio};
use crate::shared::error::{CliError, Result};

// TODO. Use cargo-generate later for more customization.

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub fn command_starters() -> Command {
    Command::new(MyCommand::STARTERS).about(StartersCommand::ABOUT)
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

const TRUNK: &str = "trunk";
const LEPTOS_SSR: &str = "leptos-ssr";
const LEPTOS_SSR_WORKSPACE: &str = "leptos-ssr-workspace";
const STARTER_TEMPLATES: &[&str] = &[TRUNK, LEPTOS_SSR, LEPTOS_SSR_WORKSPACE];

pub async fn process_starters() -> Result<()> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a starter template")
        .items(STARTER_TEMPLATES)
        .default(0)
        .interact()
        .map_err(|e| CliError::validation(format!("Failed to get user selection: {e}")))?;

    let selected_template = STARTER_TEMPLATES.get(selection)
        .ok_or_else(|| CliError::validation(format!("Invalid selection: {selection}")))?;
    clone_starter_template(selected_template)?;
    Ok(())
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

/// Helper function to clone a starter template repository
fn clone_starter_template(template_name: &str) -> Result<()> {
    println!("Installing {template_name} starter...");

    let output = ProcessCommand::new("git")
        .arg("clone")
        .arg(format!("https://github.com/rust-ui/start-{template_name}.git"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| CliError::git_operation("clone".to_string(), format!("Failed to execute git clone: {e}")))?;

    if output.status.success() {
        println!("✅ Successfully cloned {template_name} starter template");
    } else {
        return Err(CliError::git_operation(
            "clone".to_string(),
            format!("Failed to clone {template_name} starter template")
        ));
    }
    Ok(())
}
