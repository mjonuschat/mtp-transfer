use anyhow::Result;
use indicatif::ProgressBar;
use libmtp_rs::device::MtpDevice;
use libmtp_rs::object::{filetypes::Filetype, Object};
use libmtp_rs::storage::{Parent, Storage};
use ptree::item::StringItem;

use crate::helpers;
use crate::mtp::{get_device, DeviceSelector};

pub fn filetree(selector: DeviceSelector, verbose: bool) -> Result<()> {
    let device = get_device(&selector)?;

    for (id, storage) in device.storage_pool().iter() {
        let name = storage
            .description()
            .map_or_else(|| id.to_string(), |v| v.to_owned());

        let spinner = helpers::create_spinner(&format!("Scanning {}", &name));

        let result = recursive_file_tree(
            &device,
            storage,
            Parent::Root,
            format!("Storage: {}", &name),
            verbose,
            &spinner,
        );

        spinner.finish_and_clear();

        match result {
            Some(tree) => ptree::print_tree(&tree)?,
            None => println!("Storage: {} - no activity files found", &name),
        }
    }

    Ok(())
}

fn recursive_file_tree<'a>(
    device: &MtpDevice,
    storage: &'a Storage,
    parent: Parent,
    text: String,
    verbose: bool,
    spinner: &ProgressBar,
) -> Option<StringItem> {
    let files = storage.files_and_folders(parent);
    let mut children: Vec<StringItem> = Vec::new();

    for file in files {
        spinner.tick();
        if matches!(file.ftype(), Filetype::Folder) {
            let result = recursive_file_tree(
                device,
                storage,
                Parent::Folder(file.id()),
                file.name().to_string(),
                verbose,
                spinner,
            );

            if let Some(item) = result {
                children.push(item)
            }
        } else if verbose || helpers::is_activity_file(file.name()) {
            children.push(StringItem {
                text: file.name().to_string(),
                children: Vec::new(),
            })
        }
    }

    if verbose || !children.is_empty() {
        return Some(StringItem { text, children });
    }

    None
}
