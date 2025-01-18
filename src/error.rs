use thiserror::Error;

#[derive(Error, Debug)]
pub enum PsMmapError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("This memory map is read-only.")]
    ReadOnly,
}
