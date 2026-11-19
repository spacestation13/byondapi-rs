use crate::prelude::*;
use crate::static_global::byond;
use crate::Error;
use std::ffi::CString;

/// Gets stringid of a string, fails if this string doesn't exist
pub fn str_id_of<T: Into<Vec<u8>>>(string: T) -> Result<u32, Error> {
    let c_string = CString::new(string).unwrap();
    let res = unsafe { byond().Byond_GetStrId(c_string.as_ptr()) };
    if res == u2c::MAX as u32 {
        return Err(Error::NonExistentString(c_string.to_owned()));
    }
    Ok(res)
}

/// Gets stringid of a string or adds it if it doesn't exist, fails if this string can't be added
pub fn addget_str_id_of<T: Into<Vec<u8>>>(string: T) -> Result<u32, Error> {
    let c_string = CString::new(string).unwrap();
    let res = unsafe { byond().Byond_AddGetStrId(c_string.as_ptr()) };
    if res == u2c::MAX as u32 {
        return Err(Error::UnableToCreateString(c_string.to_owned()));
    }
    Ok(res)
}
