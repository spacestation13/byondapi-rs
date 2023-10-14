use crate::prelude::*;
use crate::static_global::byond;
use std::ffi::CString;

pub fn str_id_of<T: Into<Vec<u8>>>(string: T) -> u4c {
    let c_string = CString::new(string).unwrap();
    let c_str = c_string.as_c_str();

    unsafe { byond().Byond_GetStrId(c_str.as_ptr()) }
}
