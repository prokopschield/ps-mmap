use std::{
    fs::{File, OpenOptions},
    path::Path,
};

use crate::{MapError, WritableMemoryMap};

impl WritableMemoryMap {
    /// Opens a [`File`] and maps it into mutable memory.
    ///
    /// # Errors
    ///
    /// Returns a [`MapError`] if mapping fails for any reason.
    pub fn map_path<P: AsRef<Path>>(file_path: P) -> Result<Self, MapError> {
        let file: File = OpenOptions::new().read(true).write(true).open(file_path)?;

        Self::map_file(file)
    }
}
