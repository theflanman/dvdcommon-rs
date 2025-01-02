use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::{mem, slice};
use std::os::fd::{AsRawFd, FromRawFd, RawFd};
use std::os::raw::{c_int, c_void};
use crate::DvdInterface;

pub struct FileInterface {
    file: File,
}

impl FileInterface {
    pub fn new(file: File) -> FileInterface {FileInterface{file}}
}

impl DvdInterface for FileInterface {
    fn get_handle(&self) -> *mut c_void {
        let handle: *mut RawFd = &mut self.file.as_raw_fd();
        handle as *mut c_void
    }

    fn get_seek_callback(&self) -> unsafe extern "C" fn(handle: *mut c_void, u64) -> c_int {
        unsafe extern "C" fn seek(handle: *mut c_void, pos: u64) -> c_int {
            println!("seek to {}", pos);
            let handle = handle as *mut RawFd;
            let mut file = File::from_raw_fd(handle.read());
            let result = file.seek(SeekFrom::Start(pos));
            mem::forget(file);

            match result {
                Ok(_) => {0}
                Err(e) => {
                    eprintln!("Error seeking file: {}", e);
                    -1
                }
            }
        }
        seek
    }

    fn get_read_callback(&self) -> unsafe extern "C" fn(*mut c_void, *mut c_void, c_int) -> c_int {
        unsafe extern "C" fn read(handle: *mut c_void, buffer: *mut c_void, n_bytes: c_int) -> c_int {
            println!("read {} bytes", n_bytes);
            let handle = handle as *mut RawFd;
            let mut file = File::from_raw_fd(handle.read());

            let buffer = slice::from_raw_parts_mut(buffer as *mut u8, n_bytes as usize);
            let result = file.read(buffer);

            mem::forget(file);

            match result {
                Ok(bytes_read) => {bytes_read as c_int}
                Err(e) => {
                    eprintln!("{:?}", e);
                    -1
                }
            }
        }
        read
    }

    fn get_readv_callback(&self) -> unsafe extern "C" fn(*mut c_void, *mut c_void, c_int) -> c_int {
        unsafe extern "C" fn readv(handle: *mut c_void, buffer: *mut c_void, n_bytes: c_int) -> c_int {
            let handle = handle as *mut RawFd;
            let mut file = File::from_raw_fd(handle.read());
            todo!();
            // mem::forget(file);
            //
            // match result {
            //     Ok(_) => {0}
            //     Err(_) => {-1}
            // }
        }
        readv
    }
}