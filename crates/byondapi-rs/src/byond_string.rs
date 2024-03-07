use crate::prelude::*;
use crate::static_global::byond;
use crate::Error;
use std::ffi::{CStr, CString};

pub fn str_id_of<T: Into<Vec<u8>>>(string: T) -> Result<u4c, Error> {
    let c_string = CString::new(string).unwrap();
    str_id_of_cstr(c_string.as_c_str())
}

pub fn str_id_of_cstr(string: &CStr) -> Result<u4c, Error> {
    let res = unsafe { byond().Byond_GetStrId(string.as_ptr()) };
    if res == u2c::MAX as u32 {
        return Err(Error::NonExistentString(string.to_owned()));
    }
    Ok(res)
}
