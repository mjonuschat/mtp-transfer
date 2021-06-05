use crate::helpers;
use libmtp_rs::object::filetypes::Filetype;
use libmtp_rs::storage::files::File;
use libmtp_rs::storage::{Parent, Storage};

pub fn get_activity_files<'a>(storage: &'a Storage, parent: Parent) -> Vec<File<'a>> {
    storage
        .files_and_folders(parent)
        .into_iter()
        .filter(|item| !matches!(item.ftype(), Filetype::Folder))
        .filter(|item| helpers::is_activity_file(item.name()))
        .collect()
}
