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
