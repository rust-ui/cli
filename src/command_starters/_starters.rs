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
        .unwrap();

    match STARTER_TEMPLATES[selection] {
        TRUNK => {
            println!("Installing {TRUNK} starter...");
            let output = ProcessCommand::new("git")
                .arg("clone")
                .arg(format!("https://github.com/rust-ui/start-{TRUNK}.git"))
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output();

            match output {
                Ok(output) => {
                    if output.status.success() {
                        println!("✅ Successfully cloned {TRUNK} starter template");
                    } else {
                        eprintln!("🔸 Failed to clone {TRUNK} starter template");
                    }
                }
                Err(err) => {
                    eprintln!("🔸 Error executing git clone: {err}");
                }
            }
        }
        LEPTOS_SSR => {
            println!("Installing {LEPTOS_SSR} starter...");
            let output = ProcessCommand::new("git")
                .arg("clone")
                .arg(format!("https://github.com/rust-ui/start-{LEPTOS_SSR}.git"))
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output();

            match output {
                Ok(output) => {
                    if output.status.success() {
                        println!("✅ Successfully cloned {LEPTOS_SSR} starter template");
                    } else {
                        eprintln!("🔸 Failed to clone {LEPTOS_SSR} starter template");
                    }
                }
                Err(err) => {
                    eprintln!("🔸 Error executing git clone: {err}");
                }
            }
        }
        LEPTOS_SSR_WORKSPACE => {
            println!("Installing {LEPTOS_SSR_WORKSPACE} starter...");
            let output = ProcessCommand::new("git")
                .arg("clone")
                .arg(format!("https://github.com/rust-ui/start-{LEPTOS_SSR_WORKSPACE}.git"))
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output();

            match output {
                Ok(output) => {
                    if output.status.success() {
                        println!("✅ Successfully cloned {LEPTOS_SSR_WORKSPACE} starter template");
                    } else {
                        eprintln!("🔸 Failed to clone {LEPTOS_SSR_WORKSPACE} starter template");
                    }
                }
                Err(err) => {
                    eprintln!("🔸 Error executing git clone: {err}");
                }
            }
        }
        _ => {
            println!("Unknown starter template");
        }
    }
}
