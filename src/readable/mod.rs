mod methods;

use std::{fs::File, ops::Deref, sync::Arc};

use memmap2::Mmap;

#[derive(Clone, Debug)]
pub struct ReadableMemoryMap {
    inner: Arc<ReadableMemoryMapInner>,
}

#[derive(Debug)]
pub struct ReadableMemoryMapInner {
    file: File,
    mmap: Mmap,
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
