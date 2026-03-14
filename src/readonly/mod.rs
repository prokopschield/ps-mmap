mod methods;

use std::{fs::File, ops::Deref, sync::Arc};

use memmap2::Mmap;

#[derive(Clone, Debug)]
pub struct ReadonlyMemoryMap {
    inner: Arc<ReadonlyMemoryMapInner>,
}

#[derive(Debug)]
pub struct ReadonlyMemoryMapInner {
    file: File,
    mmap: Mmap,
}

impl AsRef<[u8]> for ReadonlyMemoryMap {
    fn as_ref(&self) -> &[u8] {
        &self.inner.mmap
    }
}

impl AsRef<Mmap> for ReadonlyMemoryMap {
    fn as_ref(&self) -> &Mmap {
        &self.inner.mmap
    }
}

/// Exposes the file handle that owns the shared OS file lock for this mapping.
///
/// Cloning the returned handle (`File::try_clone`) may extend lock lifetime
/// beyond the lifetime of this `ReadableMemoryMap`.
impl AsRef<File> for ReadonlyMemoryMap {
    fn as_ref(&self) -> &File {
        &self.inner.file
    }
}

impl Deref for ReadonlyMemoryMap {
    type Target = Mmap;

    fn deref(&self) -> &Self::Target {
        &self.inner.mmap
    }
}
