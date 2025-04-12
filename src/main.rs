use clap::Command;
use std::process;


mod command_add;
mod command_init;
mod constants;
mod shared;

use constants::commands::{COMMAND_ADD, COMMAND_INIT};

// cargo install ui-cli --force
// ui init
// * ui add button demo_button demo_button_variants demo_button_sizes
// * ui add demo_use_floating_placement

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
        .subcommand(command_init::init_main::command_init())
        .subcommand(command_add::add_main::command_add());

    let matches = mut_program.clone().get_matches();

    // Handle commands
    match matches.subcommand() {
        Some((COMMAND_INIT, _)) => {
            command_init::process_init::process_init().await;
        }
        Some((COMMAND_ADD, sub_matches)) => {
            let _ = command_add::add_main::process_add(sub_matches).await;
        }
        _ => {
            mut_program.print_help().unwrap();
            process::exit(1);
        }
    }
}
