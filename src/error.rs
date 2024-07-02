use thiserror::Error;

#[derive(Error, Debug)]
pub enum PsMmapError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("This map is read-only.")]
    ReadOnly,
}

pub type Result<T> = std::result::Result<T, PsMmapError>;
