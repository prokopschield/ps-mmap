use std::sync::PoisonError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PsMmapError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("This map is read-only.")]
    ReadOnly,
    #[error("The mutex protecting this map has been poisoned.")]
    MutexPoison,
}

pub type Result<T> = std::result::Result<T, PsMmapError>;

impl<T> From<PoisonError<T>> for PsMmapError {
    fn from(_: PoisonError<T>) -> Self {
        Self::MutexPoison
    }
}
