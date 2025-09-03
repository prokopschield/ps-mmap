use crate::writable::WritableMemoryMapInner;

impl Drop for WritableMemoryMapInner {
    fn drop(&mut self) {
        match self.file.unlock() {
            Ok(()) => {}
            Err(err) => {
                eprintln!("Failed to release lock on memory-mapped file: {err}",);
            }
        }
    }
}
