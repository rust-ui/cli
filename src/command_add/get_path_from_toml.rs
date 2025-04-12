/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[allow(non_snake_case)]
pub fn get_base_path_from_Components_toml() -> Result<String, String> {
    // Read the Components.toml file
    let config_str = match std::fs::read_to_string("Components.toml") {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading Components.toml: {}", e);
            return Ok("components".to_string()); // Default to "components"
        }
    };

    let mut base_path = "components".to_string(); // Default value

    // Split the lines and find the base_path
    for line in config_str.lines() {
        if line.starts_with("path = ") {
            // Extract the path value
            let path_value = line.split('=').nth(1).unwrap_or("").trim().trim_matches('"');
            base_path = path_value.to_string();
            break; // Exit the loop once we find the path
        }
    }

    Ok(base_path)
}
