mod methods;

use std::{
    fs::{File, OpenOptions},
    ops::Deref,
    path::Path,
    sync::Arc,
};

use memmap2::Mmap;

use crate::MapError;

#[derive(Clone, Debug)]
pub struct ReadableMemoryMap {
    inner: Arc<ReadableMemoryMapInner>,
}

#[derive(Debug)]
pub struct ReadableMemoryMapInner {
    file: File,
    mmap: Mmap,
}

impl ReadableMemoryMap {
    /// # Errors
    /// Returns `IoError` if mapping fails for any reason.
    pub fn map<P: AsRef<Path>>(file_path: P) -> Result<Self, MapError> {
        let file = OpenOptions::new().read(true).write(false).open(file_path)?;

        Self::map_file(file)
    }
}

impl AsRef<[u8]> for ReadableMemoryMap {
    fn as_ref(&self) -> &[u8] {
        &self.inner.mmap
    }
}

impl AsRef<Mmap> for ReadableMemoryMap {
    fn as_ref(&self) -> &Mmap {
        &self.inner.mmap
    }
}

impl AsRef<File> for ReadableMemoryMap {
    fn as_ref(&self) -> &File {
        &self.inner.file
    }
}

impl Deref for ReadableMemoryMap {
    type Target = Mmap;

    fn deref(&self) -> &Self::Target {
        &self.inner.mmap
    }
}
