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
    pub fn map(file_path: &str, readonly: bool) -> Result<Self, PsMmapError> {
        if readonly {
            Self::map_readable(file_path)
        } else {
            Self::map_writable(file_path)
        }
    }

    pub fn map_readable(file_path: &str) -> Result<Self, PsMmapError> {
        Ok(Self::Readable(ReadableMemoryMap::map(file_path)?))
    }

    pub fn map_writable(file_path: &str) -> Result<Self, PsMmapError> {
        Ok(Self::Writable(WritableMemoryMap::map(file_path)?))
    }

    pub fn read(&self) -> ReadGuard {
        self.into()
    }

    pub fn into_read(self) -> ReadGuard {
        self.into()
    }

    pub fn try_write(&self) -> Result<WriteGuard, PsMmapError> {
        self.try_into()
    }

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

    pub fn try_write_with<F, R>(&self, closure: F) -> Result<R, PsMmapError>
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        match self {
            Self::Readable(_) => Err(PsMmapError::ReadOnly),
            Self::Writable(mmap) => Ok(closure(&mut mmap.write())),
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        match self {
            Self::Readable(value) => value.as_ptr(),
            Self::Writable(value) => value.as_ptr(),
        }
    }

    pub fn try_as_mut_ptr(&self) -> Result<*mut u8, PsMmapError> {
        match self {
            Self::Readable(_) => Err(PsMmapError::ReadOnly),
            Self::Writable(value) => Ok(value.as_mut_ptr()),
        }
    }
}
