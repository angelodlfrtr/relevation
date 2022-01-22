use std::path::PathBuf;
use walkdir::WalkDir;

// check_root validity
pub fn check_root<'a>(root_path: &PathBuf) -> Result<(), &'a str> {
    // Check if path exist
    if !root_path.exists() {
        return Err("Provided data path seems to not existing");
    }

    // Check if path is a directory
    if !root_path.is_dir() {
        return Err("Provided data path seems to not be a directory");
    }

    return Ok(());
}

// list_geotif_files in root_path. Try to find files recursively with .tif extension
pub fn list_geotif_files(root_path: &PathBuf) -> Result<Vec<PathBuf>, &str> {
    let mut tif_paths = Vec::new();

    fn is_tif_or_dir(entry: &walkdir::DirEntry) -> bool {
        if entry.path().is_dir() {
            return true;
        }
        return entry
            .file_name()
            .to_str()
            .unwrap_or_default()
            .ends_with(".tif");
    }

    let walker = WalkDir::new(root_path.to_str().unwrap()).into_iter();
    for entry in walker.filter_entry(|e| is_tif_or_dir(e)) {
        let file = entry.unwrap();
        if file
            .file_name()
            .to_str()
            .unwrap_or_default()
            .ends_with(".tif")
        {
            tif_paths.push(file.path().to_owned());
        }
    }

    return Ok(tif_paths);
}
