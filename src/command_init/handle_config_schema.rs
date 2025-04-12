/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

use crate::{command_init::init_fetch_functions::handle_fetch_from_init, constants::urls::URL_CONFIG_SCHEMA_JSON};

pub async fn handle_config_schema() {
    let _ = handle_fetch_from_init(URL_CONFIG_SCHEMA_JSON).await;
    // println!("{}", registry_schema.unwrap());
    println!("🦀 handle_config_schema OK 🦀");
}
