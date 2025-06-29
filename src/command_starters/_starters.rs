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
const STARTER_TEMPLATES: &[&str] = &[TRUNK];

pub async fn process_starters() {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a starter template")
        .items(STARTER_TEMPLATES)
        .default(0)
        .interact()
        .unwrap();

    match STARTER_TEMPLATES[selection] {
        TRUNK => {
            println!("Installing trunk starter...");
            let output = ProcessCommand::new("git")
                .arg("clone")
                .arg(format!("https://github.com/rust-ui/start-{TRUNK}.git"))
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output();

            match output {
                Ok(output) => {
                    if output.status.success() {
                        println!("✅ Successfully cloned trunk starter template");
                    } else {
                        eprintln!("🔸 Failed to clone trunk starter template");
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
