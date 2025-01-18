use std::ops::Deref;

use memmap2::MmapMut;
use parking_lot::{ArcRwLockReadGuard, RawRwLock};

use crate::{MemoryMap, ReadableMemoryMap, WritableMemoryMap};

#[derive(Debug)]
pub enum ReadGuard {
    Readable(ReadableMemoryMap),
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
            MemoryMap::Readable(value) => value.into(),
            MemoryMap::Writable(value) => value.into(),
        }
    }
}

impl From<ReadableMemoryMap> for ReadGuard {
    fn from(value: ReadableMemoryMap) -> Self {
        Self::Readable(value)
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
            ReadGuard::Readable(value) => Self::Readable(value.clone()),
            ReadGuard::Writable(_, value) => Self::Writable(value.clone()),
        }
    }
}

impl From<ReadGuard> for MemoryMap {
    fn from(value: ReadGuard) -> Self {
        match value {
            ReadGuard::Readable(value) => Self::Readable(value),
            ReadGuard::Writable(_, value) => Self::Writable(value),
        }
    }
}

impl AsRef<[u8]> for ReadGuard {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Readable(value) => value,
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
