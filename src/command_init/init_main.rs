use clap::{Arg, Command};

use crate::command_init::process_init::process_init;
use crate::constants::commands::COMMAND_INIT;

pub fn command_init() -> Command {
    Command::new(COMMAND_INIT)
        .about("Initialize the project")
        .arg(
            Arg::new("project_name")
                .help("The name of the project to initialize")
                .required(false),
        )
        .subcommand(Command::new("run").about("Run the initialization logic"))
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[allow(dead_code)]
pub async fn init_project() {
    process_init().await;
}
