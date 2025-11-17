use ui_cli::shared::rust_ui_client::RustUIClient;

#[tokio::test]
async fn test_fetch_tree_md() {
    let result = RustUIClient::fetch_tree_md().await;

    assert!(result.is_ok(), "Failed to fetch tree.md: {:?}", result.err());

    let content = result.unwrap();
    assert!(!content.is_empty(), "tree.md content should not be empty");
}

#[tokio::test]
async fn test_fetch_styles_default_alert() {
    let result = RustUIClient::fetch_styles_default("alert").await;

    assert!(result.is_ok(), "Failed to fetch alert.md: {:?}", result.err());

    let rust_code = result.unwrap();
    assert!(!rust_code.is_empty(), "Extracted Rust code from alert.md should not be empty");
    // Basic sanity check that it contains Rust code
    assert!(rust_code.contains("fn") || rust_code.contains("use") || rust_code.contains("pub"),
            "Content should contain Rust code");
}

#[tokio::test]
async fn test_fetch_styles_default_button() {
    let result = RustUIClient::fetch_styles_default("button").await;

    assert!(result.is_ok(), "Failed to fetch button.md: {:?}", result.err());

    let rust_code = result.unwrap();
    assert!(!rust_code.is_empty(), "Extracted Rust code from button.md should not be empty");
}

#[tokio::test]
async fn test_fetch_styles_index() {
    let result = RustUIClient::fetch_styles_index().await;

    assert!(result.is_ok(), "Failed to fetch styles/index.json: {:?}", result.err());

    let json_content = result.unwrap();
    assert!(!json_content.is_empty(), "styles/index.json content should not be empty");

    // Verify it's valid JSON by parsing it
    let parsed: serde_json::Value = serde_json::from_str(&json_content)
        .expect("Response should be valid JSON");

    // Basic structure check - should be an object or array
    assert!(parsed.is_object() || parsed.is_array(),
            "JSON should be an object or array");
}

#[tokio::test]
async fn test_fetch_nonexistent_component() {
    let result = RustUIClient::fetch_styles_default("nonexistent_component_xyz").await;

    // Should fail for nonexistent components
    assert!(result.is_err(), "Should fail when fetching nonexistent component");
}

#[tokio::test]
async fn test_styles_index_url_format() {
    let url = RustUIClient::styles_index_url();

    assert_eq!(url, "https://www.rust-ui.com/registry/styles/index.json");
    assert!(url.starts_with("https://"));
    assert!(url.ends_with(".json"));
}
