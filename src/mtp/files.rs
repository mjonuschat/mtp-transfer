use libmtp_rs::object::filetypes::Filetype;
use libmtp_rs::storage::files::File;
use libmtp_rs::storage::{Parent, Storage};

pub fn get_files<'a>(storage: &'a Storage, parent: Parent, extension: &'a str) -> Vec<File<'a>> {
    storage
        .files_and_folders(parent)
        .into_iter()
        .filter(|item| !matches!(item.ftype(), Filetype::Folder))
        .filter(|item| item.name().ends_with(extension))
        // .take(5)
        .collect()
}
