use crate::constants::commands::{MyCommand, StartersCommand};
use clap::Command;
use dialoguer::{Select, theme::ColorfulTheme};
use std::process::{Command as ProcessCommand, Stdio};

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

pub async fn process_starters() {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a starter template")
        .items(STARTER_TEMPLATES)
        .default(0)
        .interact()
        .expect("Failed to select a starter template");

    let selected_template = STARTER_TEMPLATES.get(selection).expect("Invalid selection");
    clone_starter_template(selected_template);
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

/// Helper function to clone a starter template repository
fn clone_starter_template(template_name: &str) {
    println!("Installing {template_name} starter...");

    let output = ProcessCommand::new("git")
        .arg("clone")
        .arg(format!("https://github.com/rust-ui/start-{template_name}.git"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("✅ Successfully cloned {template_name} starter template");
            } else {
                eprintln!("🔸 Failed to clone {template_name} starter template");
            }
        }
        Err(err) => {
            eprintln!("🔸 Error executing git clone: {err}");
        }
    }
}
