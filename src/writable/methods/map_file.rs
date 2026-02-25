use std::{fs::File, sync::Arc};

use memmap2::MmapMut;
use parking_lot::RwLock;

use crate::{error::MapError, writable::WritableMemoryMapInner, WritableMemoryMap};

impl WritableMemoryMap {
    /// Maps a writable [`File`] into mutable memory.
    ///
    /// # Errors
    ///
    /// - [`MapError`] is returned if memory mapping fails, e.g. if the file inaccessible or read-only.
    pub fn map_file(file: File) -> Result<Self, MapError> {
        // Lock the file to prevent others from mutably mapping it.
        file.try_lock()?;
        // This lock is released by WritableMemoryMapInner's Drop implementation.

        let mmap = unsafe { MmapMut::map_mut(&file) }?;

        let mmap = Self {
            inner: Arc::new(WritableMemoryMapInner {
                file,
                mmap: Arc::new(RwLock::new(mmap)),
            }),
        };

        Ok(mmap)
    }
}
