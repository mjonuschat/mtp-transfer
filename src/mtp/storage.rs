use std::path::Path;

use libmtp_rs::storage::StoragePool;
use libmtp_rs::{
    object::{filetypes::Filetype, Object},
    storage::{files::File, Parent, Storage},
};

use crate::error::StorageError;

pub struct ActivityFolder<'a> {
    /// ID of the storage device
    pub storage: &'a Storage<'a>,
    /// Parent object of activity folder
    pub folder: Parent,
}

fn find_folder_recursive<'a>(
    path: &Path,
    storage: &'a Storage,
    folder: Option<File<'a>>,
) -> Result<Option<File<'a>>, StorageError> {
    let parent = folder
        .as_ref()
        .map_or(Parent::Root, |f| Parent::Folder(f.id()));
    let mut components = path.components();

    match components.next() {
        Some(component) => {
            let mut targets = storage
                .files_and_folders(parent)
                .into_iter()
                .filter(|entry| {
                    matches!(entry.ftype(), Filetype::Folder)
                        && entry.name() == component.as_os_str()
                });

            match targets.next() {
                Some(target) => find_folder_recursive(
                    &components.as_path().to_path_buf(),
                    storage,
                    Some(target),
                ),
                None => Err(StorageError::FolderNotFound(
                    component.as_os_str().to_string_lossy().to_string(),
                )),
            }
        }
        None => Ok(folder),
    }
}

// TODO: Handle multiple storages with identical folders
pub fn find_activity_folder<'a>(
    storage_pool: &'a StoragePool<'a>,
    path: &Path,
) -> Result<ActivityFolder<'a>, StorageError> {
    let mut activity_folder: Option<ActivityFolder> = None;

    for (i, (_id, storage)) in storage_pool.iter().enumerate() {
        // Find activity folder
        if let Some(folder) = find_folder_recursive(path, storage, None)? {
            println!(
                "Found {} folder on Storage {}:",
                path.to_string_lossy(),
                i + 1
            );
            println!(
                "  Description: {}",
                storage.description().unwrap_or("Unknown")
            );
            println!(
                "  Max. capacity: {}",
                bytefmt::format(storage.maximum_capacity())
            );
            println!(
                "  Free space: {}",
                bytefmt::format(storage.free_space_in_bytes())
            );

            activity_folder.replace(ActivityFolder {
                storage,
                folder: Parent::Folder(folder.id()),
            });
            break;
        }
    }

    activity_folder
        .ok_or_else(|| StorageError::FolderNotFound("Activity folder not found".to_string()))
}
