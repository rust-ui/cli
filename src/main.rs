#![cfg_attr(
    not(test),
    deny(
        // clippy::expect_used, // TODO later.
        clippy::unwrap_used,
        clippy::panic,
        clippy::todo,
        clippy::indexing_slicing,
    )
)]
#![deny(irrefutable_let_patterns)]

use clap::Command;
use std::process;

mod command_add;
mod command_init;
mod command_starters;
mod constants;
mod shared;

use constants::commands::MyCommand;

// * cargo run --bin ui init
// * cargo run --bin ui add button demo_button demo_button_variants demo_button_sizes
// * cargo run --bin ui add demo_use_floating_placement
// * cargo run --bin ui starters

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
        .subcommand(command_add::_add::command_add())
        .subcommand(command_starters::_starters::command_starters());

    let matches = mut_program.clone().get_matches();

    // Handle commands
    match matches.subcommand() {
        Some((MyCommand::INIT, _)) => {
            command_init::_init::process_init().await;
        }
        Some((MyCommand::ADD, sub_matches)) => {
            let _ = command_add::_add::process_add(sub_matches).await;
        }
        Some((MyCommand::STARTERS, _)) => {
            command_starters::_starters::process_starters().await;
        }
        _ => {
            if let Err(err) = mut_program.print_help() {
                eprintln!("Error printing help: {err}");
            }
            process::exit(1);
        }
    }
}
