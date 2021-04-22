use crate::arguments::PathError;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub fn create_output_dir(path: &Path) -> Result<(), PathError> {
    match std::fs::metadata(&path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok(())
            } else {
                Err(PathError::Inaccessible(path.to_string_lossy().to_string()))
            }
        }
        Err(_e) => Ok(std::fs::create_dir_all(&path)?),
    }
}

pub fn read_existing_activities(path: &Path, extension: &str) -> Vec<String> {
    fn is_dir_or_activity(entry: &DirEntry, pattern: &str) -> bool {
        if entry.path().is_dir() {
            return true;
        }

        entry
            .file_name()
            .to_str()
            .map(|s| s.ends_with(pattern))
            .unwrap_or(false)
    }

    WalkDir::new(path)
        .into_iter()
        .filter_entry(|entry| is_dir_or_activity(entry, extension))
        .filter_map(|entry| entry.ok())
        .filter(|entry| !entry.path().is_dir())
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect::<Vec<String>>()
}
