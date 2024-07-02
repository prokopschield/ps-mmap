use crate::error::Result;
use crate::MemoryMapping;
use crate::PsMmapError;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct MutableMemoryMapping<'lt> {
    mapping: MemoryMapping<'lt>,
    arc: Arc<Mutex<&'lt mut [u8]>>,
}

impl<'lt> MutableMemoryMapping<'lt> {
    pub fn from(mapping: MemoryMapping<'lt>, arc: Arc<Mutex<&'lt mut [u8]>>) -> Self {
        Self { mapping, arc }
    }

    pub fn done(self) -> MemoryMapping<'lt> {
        self.mapping
    }
}

impl<'lt> From<MutableMemoryMapping<'lt>> for MemoryMapping<'lt> {
    fn from(mapping: MutableMemoryMapping<'lt>) -> Self {
        mapping.done()
    }
}

impl<'lt> TryFrom<MemoryMapping<'lt>> for MutableMemoryMapping<'lt> {
    type Error = PsMmapError;

    fn try_from(mapping: MemoryMapping<'lt>) -> Result<Self> {
        mapping.rw()
    }
}

impl<'lt> Deref for MutableMemoryMapping<'lt> {
    type Target = Arc<Mutex<&'lt mut [u8]>>;

    fn deref(&self) -> &Self::Target {
        &self.arc
    }
}
