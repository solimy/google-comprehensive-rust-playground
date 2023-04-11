// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

mod ffi {
    use std::os::raw::{c_char, c_int, c_long, c_ulong, c_ushort};

    // Opaque type. See https://doc.rust-lang.org/nomicon/ffi.html.
    #[repr(C)]
    pub struct DIR {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    // Layout as per dirent(1) on osx.
    #[repr(C)]
    #[derive(Debug)]
    pub struct dirent {
        pub d_ino: c_ulong,
        pub d_seekoff: c_long,
        pub d_reclen: c_ushort,
        pub d_namlen: c_ushort,
        pub d_type: c_ushort,
        pub d_name: [c_char; 1024],
    }

    extern "C" {
        pub fn opendir(s: *const c_char) -> *mut DIR;
        pub fn readdir(s: *mut DIR) -> *const dirent;
        pub fn closedir(s: *mut DIR) -> c_int;
    }
}

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        // Call opendir and return a Ok value if that worked,
        // otherwise return Err with a message.
        unsafe {
            let path = CString::new(path).unwrap();
            let dir = ffi::opendir(path.as_ptr());
            if dir.is_null() {
                Err("opendir failed".to_string())
            } else {
                Ok(DirectoryIterator {
                    path: path,
                    dir: dir,
                })
            }
        }
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        // Keep calling readdir until we get a NULL pointer back.
        unsafe {
            let entry = ffi::readdir(self.dir);
            if entry.is_null() {
                println!("readdir returned null");
                None
            } else {
                // println!("readdir returned {:?}", (*entry).d_name);
                let name = CStr::from_ptr((*entry).d_name.as_ptr());
                Some(OsStr::from_bytes(name.to_bytes()).to_os_string())
            }
        }
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        unsafe {
            ffi::closedir(self.dir);
        }
    }
}

pub fn main() -> Result<(), String> {
    let iter = DirectoryIterator::new(".")?;
    println!("files: {:#?}", iter.collect::<Vec<_>>());
    Ok(())
}