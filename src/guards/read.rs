use std::ops::Deref;

use memmap2::MmapMut;
use parking_lot::{ArcRwLockReadGuard, RawRwLock};

use crate::{MemoryMap, ReadonlyMemoryMap, WritableMemoryMap};

#[derive(Debug)]
pub enum ReadGuard {
    Readonly(ReadonlyMemoryMap),
    Writable(ArcRwLockReadGuard<RawRwLock, MmapMut>, WritableMemoryMap),
}

impl From<&MemoryMap> for ReadGuard {
    fn from(value: &MemoryMap) -> Self {
        value.clone().into()
    }
}

impl From<MemoryMap> for ReadGuard {
    fn from(value: MemoryMap) -> Self {
        match value {
            MemoryMap::Readonly(value) => value.into(),
            MemoryMap::Writable(value) => value.into(),
        }
    }
}

impl From<ReadonlyMemoryMap> for ReadGuard {
    fn from(value: ReadonlyMemoryMap) -> Self {
        Self::Readonly(value)
    }
}

impl From<WritableMemoryMap> for ReadGuard {
    fn from(value: WritableMemoryMap) -> Self {
        Self::Writable(value.read(), value)
    }
}

impl From<&ReadGuard> for MemoryMap {
    fn from(value: &ReadGuard) -> Self {
        match value {
            ReadGuard::Readonly(value) => Self::Readonly(value.clone()),
            ReadGuard::Writable(_, value) => Self::Writable(value.clone()),
        }
    }
}

impl From<ReadGuard> for MemoryMap {
    fn from(value: ReadGuard) -> Self {
        match value {
            ReadGuard::Readonly(value) => Self::Readonly(value),
            ReadGuard::Writable(_, value) => Self::Writable(value),
        }
    }
}

impl AsRef<[u8]> for ReadGuard {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Readonly(value) => value,
            Self::Writable(value, _) => value,
        }
    }
}

impl Deref for ReadGuard {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
