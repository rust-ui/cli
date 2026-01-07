use std::collections::HashSet;
use std::path::Path;

/// Scan the components directory and return a set of installed component names
pub fn get_installed_components(base_path: &str) -> HashSet<String> {
    let mut installed = HashSet::new();
    let base = Path::new(base_path);

    if !base.exists() {
        return installed;
    }

    // Scan subdirectories: ui/, demos/, hooks/, extensions/
    let subdirs = ["ui", "demos", "hooks", "extensions"];

    for subdir in subdirs {
        let dir_path = base.join(subdir);
        if let Ok(entries) = std::fs::read_dir(&dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file()
                    && path.extension().is_some_and(|ext| ext == "rs")
                    && let Some(stem) = path.file_stem()
                {
                    let name = stem.to_string_lossy().to_string();
                    // Skip mod.rs
                    if name != "mod" {
                        installed.insert(name);
                    }
                }
            }
        }
    }

    installed
}
