use std::{fs::OpenOptions, path::Path};

use crate::{MapError, ReadableMemoryMap};

impl ReadableMemoryMap {
    /// Maps the file at a given path into memory.
    ///
    /// # Errors
    ///
    /// - [`MapError`] is returned if mapping the file fails for any reason.
    pub fn map_path<P: AsRef<Path>>(file_path: P) -> Result<Self, MapError> {
        let file = OpenOptions::new().read(true).open(file_path)?;

        Self::map_file(file)
    }
}
