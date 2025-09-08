use crate::shared::cli_error::CliResult;
use crate::shared::rust_ui_client::RustUIClient;

pub struct Fetch {}

impl Fetch {
    pub async fn from_url(_url: &str) -> CliResult<String> {
        // Note: The URL parameter is ignored since we're now using the dedicated method
        RustUIClient::fetch_styles_index().await
    }
}
