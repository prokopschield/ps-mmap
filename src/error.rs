use thiserror::Error;

/// This error occurs when mapping a file into memory fails.
#[derive(Error, Debug)]
pub enum MapError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum PsMmapError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("This memory map is read-only.")]
    ReadOnly,
}
