mod error;
mod guards;
mod readable;
mod writable;

pub use error::PsMmapError;
pub use guards::{ReadGuard, WriteGuard};
pub use readable::ReadableMemoryMap;
pub use writable::WritableMemoryMap;

#[derive(Clone, Debug)]
pub enum MemoryMap {
    Readable(ReadableMemoryMap),
    Writable(WritableMemoryMap),
}

impl MemoryMap {
    pub fn map_readable(file_name: &str) -> Result<Self, PsMmapError> {
        Ok(Self::Readable(ReadableMemoryMap::map(file_name)?))
    }

    pub fn map_writable(file_name: &str) -> Result<Self, PsMmapError> {
        Ok(Self::Writable(WritableMemoryMap::map(file_name)?))
    }

    pub fn read_with<F, R>(&self, closure: F) -> R
    where
        F: FnOnce(&[u8]) -> R,
    {
        match self {
            Self::Readable(mmap) => closure(mmap),
            Self::Writable(mmap) => closure(&mmap.read()),
        }
    }

    pub fn try_write_with<F, R>(&self, closure: F) -> Result<R, PsMmapError>
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        match self {
            Self::Readable(_) => Err(PsMmapError::ReadOnly),
            Self::Writable(mmap) => Ok(closure(&mut mmap.write())),
        }
    }
}
