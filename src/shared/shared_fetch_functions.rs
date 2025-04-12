use reqwest;

// ADD + INIT
pub async fn shared_fetch_registry_return_json(url: &str) -> Result<serde_json::Value, reqwest::Error> {
    let response = reqwest::get(url).await?;
    response.json::<serde_json::Value>().await
}
