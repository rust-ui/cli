use crate::shared::shared_fetch_functions::shared_fetch_registry_return_json;

pub struct Fetch {}

impl Fetch {
    pub async fn from_url(url: &str) -> Result<String, reqwest::Error> {
        let result = shared_fetch_registry_return_json(url).await;

        match result {
            Ok(json) => {
                let pretty_json = serde_json::to_string_pretty(&json)
                    .unwrap_or_else(|_| "🔸 Failed to convert to pretty JSON".to_string());

                Ok(pretty_json)
            }
            Err(err) => {
                eprintln!("🔸 Error fetching: {err}");
                Err(err)
            }
        }
    }
}
