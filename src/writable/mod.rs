mod implementations;
mod methods;

use std::{fs::File, ops::Deref, sync::Arc};

use memmap2::MmapMut;
use parking_lot::{ArcRwLockReadGuard, ArcRwLockWriteGuard, RawRwLock, RwLock};

#[derive(Clone, Debug)]
pub struct WritableMemoryMap {
    inner: Arc<WritableMemoryMapInner>,
}

#[derive(Debug)]
pub struct WritableMemoryMapInner {
    file: File,
    mmap: Arc<RwLock<MmapMut>>,
    ptr: *mut u8,
    len: usize,
}

// SAFETY: `ptr` is created from `mmap.as_mut_ptr()` at construction and never changes.
// The mapping is never replaced/resized while this value lives, and safe access to mapped
// bytes remains synchronized via `RwLock<MmapMut>`.
unsafe impl Send for WritableMemoryMapInner {}

// SAFETY: same invariants as above; `ptr` is metadata and not dereferenced internally
// without synchronization.
unsafe impl Sync for WritableMemoryMapInner {}

impl WritableMemoryMap {
    pub fn read(&self) -> ArcRwLockReadGuard<RawRwLock, MmapMut> {
        self.inner.mmap.read_arc()
    }

    pub fn write(&self) -> ArcRwLockWriteGuard<RawRwLock, MmapMut> {
        self.inner.mmap.write_arc()
    }

    #[must_use]
    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.inner.ptr
    }

    #[must_use]
    pub fn as_ptr(&self) -> *const u8 {
        self.inner.ptr.cast_const()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl AsRef<Arc<RwLock<MmapMut>>> for WritableMemoryMap {
    fn as_ref(&self) -> &Arc<RwLock<MmapMut>> {
        &self.inner.mmap
    }
}

impl AsRef<RwLock<MmapMut>> for WritableMemoryMap {
    fn as_ref(&self) -> &RwLock<MmapMut> {
        &self.inner.mmap
    }
}

impl AsRef<File> for WritableMemoryMap {
    fn as_ref(&self) -> &File {
        &self.inner.file
    }
}

impl Deref for WritableMemoryMap {
    type Target = RwLock<MmapMut>;

    fn deref(&self) -> &Self::Target {
        &self.inner.mmap
    }
}
