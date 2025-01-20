use std::ops::{Deref, DerefMut};

use memmap2::MmapMut;
use parking_lot::{ArcRwLockWriteGuard, RawRwLock};

use crate::{MemoryMap, PsMmapError, WritableMemoryMap};

#[derive(Debug)]
pub enum WriteGuard {
    Writable(ArcRwLockWriteGuard<RawRwLock, MmapMut>, WritableMemoryMap),
}

impl TryFrom<&MemoryMap> for WriteGuard {
    type Error = PsMmapError;

    fn try_from(value: &MemoryMap) -> Result<Self, Self::Error> {
        match value {
            MemoryMap::Readable(_) => Err(PsMmapError::ReadOnly),
            MemoryMap::Writable(value) => Ok(value.clone().into()),
        }
    }
}

impl TryFrom<MemoryMap> for WriteGuard {
    type Error = PsMmapError;

    fn try_from(value: MemoryMap) -> Result<Self, Self::Error> {
        match value {
            MemoryMap::Readable(_) => Err(PsMmapError::ReadOnly),
            MemoryMap::Writable(value) => Ok(value.into()),
        }
    }
}

impl From<WritableMemoryMap> for WriteGuard {
    fn from(value: WritableMemoryMap) -> Self {
        Self::Writable(value.write(), value)
    }
}

impl From<&WriteGuard> for WritableMemoryMap {
    fn from(value: &WriteGuard) -> Self {
        match value {
            WriteGuard::Writable(_, value) => value.clone(),
        }
    }
}

impl From<WriteGuard> for WritableMemoryMap {
    fn from(value: WriteGuard) -> Self {
        match value {
            WriteGuard::Writable(_, value) => value,
        }
    }
}

impl From<&WriteGuard> for MemoryMap {
    fn from(value: &WriteGuard) -> Self {
        MemoryMap::Writable(value.into())
    }
}

impl From<WriteGuard> for MemoryMap {
    fn from(value: WriteGuard) -> Self {
        MemoryMap::Writable(value.into())
    }
}

impl AsRef<[u8]> for WriteGuard {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Writable(value, _) => value,
        }
    }
}

impl Deref for WriteGuard {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl DerefMut for WriteGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Writable(value, _) => value,
        }
    }
}
