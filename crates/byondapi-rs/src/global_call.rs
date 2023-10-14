use crate::prelude::*;
use crate::static_global::byond;
use crate::Error;

use std::ffi::CString;

/// Calls a global proc.
///
/// Implicitly set waitfor=0, will never block.
pub fn call_global<T: Into<Vec<u8>>>(name: T, args: &[ByondValue]) -> Result<ByondValue, Error> {
    let c_string = CString::new(name).unwrap();
    let c_str = c_string.as_c_str();

    let str_id = unsafe { byond().Byond_GetStrId(c_str.as_ptr()) };
    if str_id == 0 {
        return Err(Error::InvalidProc);
    }
    let ptr = args.as_ptr();
    let mut new_value = ByondValue::new();
    unsafe {
        map_byond_error!(byond().Byond_CallGlobalProcByStrId(
            str_id,
            ptr.cast(),
            args.len() as u32,
            &mut new_value.0
        ))?;
    }
    Ok(new_value)
}

/// Calls a global proc by its string id.
///
/// Implicitly set waitfor=0, will never block.
pub fn call_global_id(name: u4c, args: &[ByondValue]) -> Result<ByondValue, Error> {
    let ptr = args.as_ptr();
    let mut new_value = ByondValue::new();
    unsafe {
        map_byond_error!(byond().Byond_CallGlobalProcByStrId(
            name,
            ptr.cast(),
            args.len() as u32,
            &mut new_value.0
        ))?;
    }
    Ok(new_value)
}
