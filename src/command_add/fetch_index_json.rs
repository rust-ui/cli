/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn fetch_index_content(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Attempt to fetch the content from the URL
    let response = reqwest::get(url).await;

    // Check if the request was successful
    let index_content_from_url = match response {
        Ok(resp) => {
            if resp.status().is_success() {
                resp.text().await?
            } else {
                let error_message = format!("🔸 Failed to fetch data: Server returned status {}", resp.status());
                println!("{}", error_message); // Print the error message
                return Err(error_message.into());
            }
        }
        Err(e) => {
            let error_message = format!("🔸 Failed to fetch data: {}", e);
            println!("{}", error_message); // Print the error message
            return Err(error_message.into());
        }
    };

    // Check if the fetched content is empty
    if index_content_from_url.is_empty() {
        let error_message = "🔸 Failed to fetch data: The server returned an empty response.";
        println!("{}", error_message); // Print the error message
        return Err(error_message.into());
    }

    Ok(index_content_from_url)
}
