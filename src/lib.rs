pub mod file_interface;

use std::os::raw::{c_int, c_void};

pub trait DvdInterface {
    fn get_handle(&self) -> *mut c_void;

    fn get_seek_callback(&self) -> unsafe extern "C" fn(
        p_stream: *mut c_void,
        i_pos: u64,
    ) -> c_int;

    fn get_read_callback(&self) -> unsafe extern "C" fn(
        p_stream: *mut c_void,
        buffer: *mut c_void,
        i_read: c_int,
    ) -> c_int;

    fn get_readv_callback(&self) -> unsafe extern "C" fn(
        p_stream: *mut c_void,
        p_iovec: *mut c_void,
        i_blocks: c_int,
    ) -> c_int;

}