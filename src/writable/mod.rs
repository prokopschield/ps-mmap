use std::{
    fs::{File, OpenOptions},
    ops::Deref,
    sync::Arc,
};

use memmap2::MmapMut;
use parking_lot::{ArcRwLockReadGuard, ArcRwLockWriteGuard, RawRwLock, RwLock};

use crate::PsMmapError;

#[derive(Clone, Debug)]
pub struct WritableMemoryMap {
    inner: Arc<(Arc<RwLock<MmapMut>>, File)>,
}

impl WritableMemoryMap {
    pub fn map(file_path: &str) -> Result<Self, PsMmapError> {
        let file = OpenOptions::new().read(true).write(true).open(file_path)?;
        let mmap = unsafe { MmapMut::map_mut(&file)? };

        let mmap = Self {
            inner: Arc::from((Arc::from(RwLock::from(mmap)), file)),
        };

        Ok(mmap)
    }

    pub fn read(&self) -> ArcRwLockReadGuard<RawRwLock, MmapMut> {
        self.inner.0.read_arc()
    }

    pub fn write(&self) -> ArcRwLockWriteGuard<RawRwLock, MmapMut> {
        self.inner.0.write_arc()
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        let mmap_ptr = self.inner.0.data_ptr();
        let mmap = unsafe { &mut *mmap_ptr };

        mmap.as_mut_ptr()
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.as_mut_ptr()
    }

    unsafe fn inner_mmap(&self) -> &MmapMut {
        &*self.inner.0.data_ptr()
    }

    pub fn len(&self) -> usize {
        let mmap = unsafe { self.inner_mmap() };

        mmap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl AsRef<Arc<RwLock<MmapMut>>> for WritableMemoryMap {
    fn as_ref(&self) -> &Arc<RwLock<MmapMut>> {
        &self.inner.0
    }
}

impl AsRef<RwLock<MmapMut>> for WritableMemoryMap {
    fn as_ref(&self) -> &RwLock<MmapMut> {
        &self.inner.0
    }
}

impl AsRef<File> for WritableMemoryMap {
    fn as_ref(&self) -> &File {
        &self.inner.1
    }
}

impl Deref for WritableMemoryMap {
    type Target = RwLock<MmapMut>;

    fn deref(&self) -> &Self::Target {
        &self.inner.0
    }
}
