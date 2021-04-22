use crate::mtp::Device;
use libmtp_rs::{
    device::StorageSort,
    object::{filetypes::Filetype, Object},
    storage::{files::File, Parent, Storage},
};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Folder {0} could not be found")]
    FolderNotFound(String),
}

fn find_folder<'a>(
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
                Some(target) => {
                    find_folder(&components.as_path().to_path_buf(), storage, Some(target))
                }
                None => Err(StorageError::FolderNotFound(
                    component.as_os_str().to_string_lossy().to_string(),
                )),
            }
        }
        None => Ok(folder),
    }
}

pub fn select_storage(mut device: Device, path: &Path) -> Result<Device, StorageError> {
    device
        .handle
        .update_storage(StorageSort::ByMaximumSpace)
        .unwrap();

    let storage_pool = device.handle.storage_pool();
    let mut found = false;
    for (i, (id, storage)) in storage_pool.iter().enumerate() {
        // Find garmin folder
        if let Some(folder) = find_folder(path, storage, None)? {
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

            device.storage = id;
            device.activity_folder = Parent::Folder(folder.id());
            found = true;
            break;
        }
    }

    if !found {
        return Err(StorageError::FolderNotFound(
            "Activity folder not found".to_string(),
        ));
    }

    Ok(device)
}
