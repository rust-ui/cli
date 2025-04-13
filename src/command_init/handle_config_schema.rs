use std::env;
use dotenv::dotenv;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

use crate::{command_init::fetch::Fetch, constants::env::ENV};

pub async fn handle_config_schema() {
    dotenv().ok();

    let url_config_schema_json = env::var(ENV::URL_CONFIG_SCHEMA_JSON).unwrap_or_default();

    let _ = Fetch::handle_fetch_from_init(&url_config_schema_json).await;
}
