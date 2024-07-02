pub mod error;
pub use error::PsMmapError;
use error::Result;
pub use memmap2::MmapOptions;
use std::fs::OpenOptions;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

pub enum MemoryMapOwner {
    Ro(memmap2::Mmap),
    Rw(memmap2::MmapMut),
}

impl Deref for MemoryMapOwner {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ro(ro) => ro,
            Self::Rw(rw) => rw,
        }
    }
}

pub struct MemoryMap<'lt> {
    owner: Arc<MemoryMapOwner>,
    roref: &'lt [u8],
    rwref: Option<Arc<Mutex<&'lt mut [u8]>>>,
}

impl<'lt> MemoryMap<'lt> {
    fn new(owner: Arc<MemoryMapOwner>, readonly: bool) -> Result<Self> {
        let pointer = owner.as_ptr() as *mut u8;
        let length = owner.len();

        let roref: &'lt [u8] = unsafe { std::slice::from_raw_parts(pointer, length) };

        let rwref: Option<Arc<Mutex<&'lt mut [u8]>>> = match readonly {
            false => Some(Arc::from(Mutex::from(unsafe {
                std::slice::from_raw_parts_mut(pointer, length)
            }))),
            true => None,
        };

        let map = Self {
            owner,
            roref,
            rwref,
        };

        Ok(map)
    }

    pub fn new_backed(options: &MmapOptions, file_name: &str, readonly: bool) -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(!readonly)
            .open(file_name)?;

        let owner = Arc::from(match readonly {
            true => MemoryMapOwner::Ro(unsafe { options.map(&file) }?),
            false => MemoryMapOwner::Rw(unsafe { options.map_mut(&file) }?),
        });

        Self::new(owner, readonly)
    }

    pub fn new_blank(options: &MmapOptions) -> Result<Self> {
        let owner = MemoryMapOwner::Rw(options.map_anon()?);

        Self::new(owner.into(), false)
    }

    pub fn ro(&self) -> &[u8] {
        &self.roref
    }

    pub fn rw(&'lt self) -> Result<MutexGuard<&mut [u8]>> {
        match &self.rwref {
            Some(arc) => Ok(arc.lock()?),
            None => Err(PsMmapError::ReadOnly),
        }
    }
}

impl<'lt> Clone for MemoryMap<'lt> {
    fn clone(&self) -> Self {
        Self {
            owner: self.owner.clone(),
            roref: self.roref,
            rwref: self.rwref.clone(),
        }
    }
}

impl<'lt> Deref for MemoryMap<'lt> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.roref
    }
}
