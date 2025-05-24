mod methods;

use std::{
    fs::{File, OpenOptions},
    ops::Deref,
    path::Path,
    sync::Arc,
};

use memmap2::MmapMut;
use parking_lot::{ArcRwLockReadGuard, ArcRwLockWriteGuard, RawRwLock, RwLock};

use crate::MapError;

#[derive(Clone, Debug)]
pub struct WritableMemoryMap {
    inner: Arc<WritableMemoryMapInner>,
}

#[derive(Debug)]
pub struct WritableMemoryMapInner {
    file: File,
    mmap: Arc<RwLock<MmapMut>>,
}

impl WritableMemoryMap {
    /// # Errors
    /// Returns [`MapError`] if mapping fails for any reason.
    pub fn map<P: AsRef<Path>>(file_path: P) -> Result<Self, MapError> {
        let file = OpenOptions::new().read(true).write(true).open(file_path)?;

        Self::map_file(file)
    }

    pub fn read(&self) -> ArcRwLockReadGuard<RawRwLock, MmapMut> {
        self.inner.mmap.read_arc()
    }

    pub fn write(&self) -> ArcRwLockWriteGuard<RawRwLock, MmapMut> {
        self.inner.mmap.write_arc()
    }

    #[must_use]
    pub fn as_mut_ptr(&self) -> *mut u8 {
        let mmap_ptr = self.inner.mmap.data_ptr();
        let mmap = unsafe { &mut *mmap_ptr };

        mmap.as_mut_ptr()
    }

    #[must_use]
    pub fn as_ptr(&self) -> *const u8 {
        self.as_mut_ptr()
    }

    unsafe fn inner_mmap(&self) -> &MmapMut {
        &*self.inner.mmap.data_ptr()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        let mmap = unsafe { self.inner_mmap() };

        mmap.len()
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
