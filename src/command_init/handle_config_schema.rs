use std::env;
use dotenv::dotenv;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

use crate::{command_init::init_fetch_functions::handle_fetch_from_init, constants::env::ENV};

pub async fn handle_config_schema() {
    dotenv().ok();

    let url_config_schema_json = env::var(ENV::URL_CONFIG_SCHEMA_JSON).unwrap_or_default();

    let _ = handle_fetch_from_init(&url_config_schema_json).await;
    // println!("{}", registry_schema.unwrap());
    println!("🦀 handle_config_schema OK 🦀");
}
