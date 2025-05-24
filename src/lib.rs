#![allow(clippy::module_name_repetitions)]
mod error;
mod guards;
mod readable;
mod writable;

use std::path::Path;

pub use error::{MapError, PsMmapError};
pub use guards::{ReadGuard, WriteGuard};
pub use readable::ReadableMemoryMap;
pub use writable::WritableMemoryMap;

#[derive(Clone, Debug)]
pub enum MemoryMap {
    Readable(ReadableMemoryMap),
    Writable(WritableMemoryMap),
}

impl MemoryMap {
    /// # Errors
    /// Returns [`MapError`] if mapping fails for any reason.
    pub fn map<P: AsRef<Path>>(file_path: P, readonly: bool) -> Result<Self, MapError> {
        if readonly {
            Self::map_readable(file_path)
        } else {
            Self::map_writable(file_path)
        }
    }

    /// # Errors
    /// Returns [`MapError`] if mapping fails for any reason.
    pub fn map_readable<P: AsRef<Path>>(file_path: P) -> Result<Self, MapError> {
        Ok(Self::Readable(ReadableMemoryMap::map(file_path)?))
    }

    /// # Errors
    /// Returns [`MapError`] if mapping fails for any reason.
    pub fn map_writable<P: AsRef<Path>>(file_path: P) -> Result<Self, MapError> {
        Ok(Self::Writable(WritableMemoryMap::map_path(file_path)?))
    }

    #[must_use]
    pub fn read(&self) -> ReadGuard {
        self.into()
    }

    #[must_use]
    pub fn into_read(self) -> ReadGuard {
        self.into()
    }

    /// # Errors
    /// - Returns `ReadOnly` if memory map is read-only.
    pub fn try_write(&self) -> Result<WriteGuard, PsMmapError> {
        self.try_into()
    }

    /// # Errors
    /// - Returns `ReadOnly` if memory map is read-only.
    pub fn try_into_write(self) -> Result<WriteGuard, PsMmapError> {
        self.try_into()
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

    /// # Errors
    /// - Returns `ReadOnly` if memory map is read-only.
    pub fn try_write_with<F, R>(&self, closure: F) -> Result<R, PsMmapError>
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        match self {
            Self::Readable(_) => Err(PsMmapError::ReadOnly),
            Self::Writable(mmap) => Ok(closure(&mut mmap.write())),
        }
    }

    #[must_use]
    pub fn as_ptr(&self) -> *const u8 {
        match self {
            Self::Readable(value) => value.as_ptr(),
            Self::Writable(value) => value.as_ptr(),
        }
    }

    /// # Errors
    /// - Returns `ReadOnly` if memory map is read-only.
    pub fn try_as_mut_ptr(&self) -> Result<*mut u8, PsMmapError> {
        match self {
            Self::Readable(_) => Err(PsMmapError::ReadOnly),
            Self::Writable(value) => Ok(value.as_mut_ptr()),
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Readable(value) => value.len(),
            Self::Writable(value) => value.len(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
