use std::fs::{self, File};
use std::io::{self, Write};

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn shared_write_template_file(file_path: &str, template: &str) -> io::Result<()> {
    // Create the directory if it doesn't exist
    if let Some(dir) = std::path::Path::new(file_path).parent() {
        fs::create_dir_all(dir)?;
    }

    match File::create(file_path) {
        Ok(mut file) => {
            file.write_all(template.as_bytes())?;
            Ok(())
        }
        Err(err) => {
            eprintln!("🔸 Error: {err}");
            Err(err)
        }
    }
}
