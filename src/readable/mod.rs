use std::{
    fs::{File, OpenOptions},
    ops::Deref,
    sync::Arc,
};

use memmap2::Mmap;

use crate::PsMmapError;

#[derive(Clone, Debug)]
pub struct ReadableMemoryMap {
    inner: Arc<(Mmap, File)>,
}

impl ReadableMemoryMap {
    pub fn map(file_name: &str) -> Result<Self, PsMmapError> {
        let file = OpenOptions::new().read(true).write(false).open(file_name)?;
        let mmap = unsafe { Mmap::map(&file)? };

        let mmap = Self {
            inner: Arc::from((mmap, file)),
        };

        Ok(mmap)
    }
}

impl AsRef<[u8]> for ReadableMemoryMap {
    fn as_ref(&self) -> &[u8] {
        &self.inner.0
    }
}

impl AsRef<Mmap> for ReadableMemoryMap {
    fn as_ref(&self) -> &Mmap {
        &self.inner.0
    }
}

impl AsRef<File> for ReadableMemoryMap {
    fn as_ref(&self) -> &File {
        &self.inner.1
    }
}

impl Deref for ReadableMemoryMap {
    type Target = Mmap;

    fn deref(&self) -> &Self::Target {
        &self.inner.0
    }
}
