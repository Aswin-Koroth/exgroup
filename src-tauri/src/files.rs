use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

use crate::APP_DATA_DIR;

pub fn get_files_dir() -> Result<PathBuf, String> {
    let app_data_dir = APP_DATA_DIR
        .get()
        .ok_or_else(|| "Database path is not initialized yet".to_string())?;

    let file_path = app_data_dir.join("files");

    fs::create_dir_all(&file_path).map_err(|e| format!("Failed to create files directory: {e}"))?;

    Ok(file_path)
}

pub fn get_profile_image_dir() -> Result<PathBuf, String> {
    let files_directory = get_files_dir()?.join("profiles");
    fs::create_dir_all(&files_directory)
        .map_err(|e| format!("Failed to create profiles directory: {e}"))?;
    Ok(files_directory)
}

pub fn save_profile_image(source_path: &Path, old_path: Option<&str>) -> Result<PathBuf, String> {
    if !source_path.exists() {
        return Err(format!("Photo file not found: {}", source_path.display()));
    }
    if let Some(old_path) = old_path {
        delete_image(Path::new(old_path))?
    }
    let extension = source_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg");

    let filename = format!("{}.{extension}", Uuid::new_v4());

    let destination_path = get_profile_image_dir()?.join(filename);

    fs::copy(source_path, &destination_path)
        .map_err(|e| format!("Failed to save profile image: {e}"))?;

    Ok(destination_path)
}

pub fn delete_image(image_path: &Path) -> Result<(), String> {
    if !image_path.exists() {
        return Err(format!("Image file not found: {}", image_path.display()));
    }

    fs::remove_file(image_path).map_err(|_| "Failed to delete image".to_string())?;

    Ok(())
}
