use indicatif::ProgressBar;
use std::fs::{self, File};
use std::io::{self, Write};
use std::time::Duration;

use crate::constants::others::SPINNER_UPDATE_DURATION;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn shared_write_template_file(file_path: &str, spinner: &ProgressBar, template: &str) -> io::Result<()> {
    // Create the directory if it doesn't exist
    if let Some(dir) = std::path::Path::new(file_path).parent() {
        fs::create_dir_all(dir)?;
    }

    match File::create(file_path) {
        Ok(mut file) => {
            // Start the spinner
            spinner.enable_steady_tick(Duration::from_millis(SPINNER_UPDATE_DURATION));

            file.write_all(template.as_bytes())?;
            Ok(())
        }
        Err(e) => {
            eprintln!("🔸 Error: {}", e);
            Err(e)
        }
    }
}
