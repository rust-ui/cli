use clap::{Arg, Command};

use crate::{
    command_init::process_init::process_init,
    constants::commands::{COMMAND, INIT},
};

pub fn command_init() -> Command {
    Command::new(COMMAND::INIT)
        .about(INIT::ABOUT)
        .arg(Arg::new(INIT::PROJECT_NAME).help(INIT::HELP).required(false))
        .subcommand(Command::new("run").about("Run the initialization logic"))
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[allow(dead_code)]
pub async fn init_project() {
    process_init().await;
}
