use std::{fs::File, io::Read};

use crate::MemoryMap;

#[test]
fn as_ptr() {
    let mmap = MemoryMap::map_readable("Cargo.toml").expect("Failed to map Cargo.toml");
    let ptr = mmap.as_ptr();

    let equals = unsafe { *ptr.add(15) };

    assert_eq!(equals, b'=', "as_ptr() returned an incorrect pointer!");

    let pkg = unsafe { std::slice::from_raw_parts(ptr, 9) };

    assert_eq!(
        pkg, b"[package]",
        "Failed to read [package] from Cargo.toml"
    );
}

#[test]
fn try_as_mut_ptr() {
    let mmap = MemoryMap::map_writable(".gitignore").expect("Failed to map .gitignore");
    let ptr = mmap
        .try_as_mut_ptr()
        .expect("Couldn't get mut ptr to .gitignore");

    unsafe { *ptr.add(7) = b'L' };

    let mut buf = [0u8; 11];
    let mut gitignore = File::open(".gitignore").expect("Could not open .gitignore");

    File::read_exact(&mut gitignore, &mut buf).expect("Could not read bytes from .gitignore");

    assert_eq!(buf, *b"/Cargo.Lock");

    unsafe { *ptr.add(7) = b'l' };
}
