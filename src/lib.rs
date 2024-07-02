pub mod error;
pub use error::PsMmapError;
use error::Result;
pub use memmap2::MmapOptions;
use std::fs::OpenOptions;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;

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
    pub roref: &'lt [u8],
    pub rwref: Result<Arc<Mutex<&'lt mut [u8]>>>,
}

impl<'lt> MemoryMap<'lt> {
    fn new(owner: Arc<MemoryMapOwner>, readonly: bool) -> Result<Self> {
        let pointer = owner.as_ptr() as *mut u8;
        let length = owner.len();

        let roref: &'lt [u8] = unsafe { std::slice::from_raw_parts(pointer, length) };

        let rwref: Result<Arc<Mutex<&'lt mut [u8]>>> = match readonly {
            false => Ok(Arc::from(Mutex::from(unsafe {
                std::slice::from_raw_parts_mut(pointer, length)
            }))),
            true => Err(PsMmapError::ReadOnly),
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
}

impl<'lt> Clone for MemoryMap<'lt> {
    fn clone(&self) -> Self {
        Self {
            owner: self.owner.clone(),
            roref: self.roref,
            rwref: match &self.rwref {
                Ok(arc) => Ok(arc.clone()),
                _ => Err(PsMmapError::ReadOnly),
            },
        }
    }
}

impl<'lt> Deref for MemoryMap<'lt> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.roref
    }
}
