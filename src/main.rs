#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]

use clap::Command;
use std::process;

mod command_add;
mod command_init;
mod constants;
mod shared;

use constants::commands::MyCommand;

// * cargo run --bin ui init
// * cargo run --bin ui add button demo_button demo_button_variants demo_button_sizes
// * cargo run --bin ui add demo_use_floating_placement

// TODO 🐛 add [primitives/dialog]
// └──> 🔸 Write file in primitives/primitives/dialog.tsx

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[tokio::main]
async fn main() {
    let mut mut_program = Command::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(command_init::_init::command_init())
        .subcommand(command_add::_add::command_add());

    let matches = mut_program.clone().get_matches();

    // Handle commands
    match matches.subcommand() {
        Some((MyCommand::INIT, _)) => {
            command_init::_init::init_project().await;
        }
        Some((MyCommand::ADD, sub_matches)) => {
            let _ = command_add::_add::process_add(sub_matches).await;
        }
        _ => {
            if let Err(e) = mut_program.print_help() {
                eprintln!("Error printing help: {}", e);
            }
            process::exit(1);
        }
    }
}
