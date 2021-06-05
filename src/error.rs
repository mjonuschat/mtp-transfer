use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Folder {0} could not be found")]
    FolderNotFound(String),
}

#[derive(Debug, Error)]
pub enum PathError {
    #[error("Path `{0}` could not be resolved")]
    Canonicalize(#[from] std::io::Error),
    #[error("File or directory `{0}` is not accessible")]
    Inaccessible(String),
}
