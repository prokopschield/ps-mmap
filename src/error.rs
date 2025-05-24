use thiserror::Error;

#[derive(Error, Debug)]
pub enum DerefError {
    #[error("Cannot mutably dereference a read-only MemoryMap.")]
    ReadOnly,
}

/// This error occurs when mapping a file into memory fails.
#[derive(Error, Debug)]
pub enum MapError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
