use std::{fs::File, sync::Arc};

use memmap2::Mmap;

use crate::{readable::ReadableMemoryMapInner, MapError, ReadableMemoryMap};

impl ReadableMemoryMap {
    /// Maps a readonly [`File`] into memory.
    /// Acquires a shared file lock.
    ///
    /// Lock lifetime follows OS handle lifetime: the lock is released when all
    /// duplicated handles for this file description are closed.
    ///
    /// # Errors
    ///
    /// - [`MapError`] is returned if memory-mapping the file fails for any reason.
    pub fn map_file(file: File) -> Result<Self, MapError> {
        // Lock the file to prevent others from mutably mapping it.
        file.lock_shared()?;

        let mmap = unsafe { Mmap::map(&file)? };

        let mmap = Self {
            inner: Arc::new(ReadableMemoryMapInner { file, mmap }),
        };

        Ok(mmap)
    }
}
