use crate::shared::shared_fetch_functions::shared_fetch_registry_return_json;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn handle_fetch_from_init(url: &str) -> Result<String, reqwest::Error> {
    let result = shared_fetch_registry_return_json(url).await;

    match result {
        Ok(json) => {
            let pretty_json =
                serde_json::to_string_pretty(&json).unwrap_or_else(|_| "🔸 Failed to convert to pretty JSON".to_string());

            Ok(pretty_json)
        }
        Err(e) => {
            eprintln!("🔸 Error fetching: {}", e);
            Err(e)
        }
    }
}
