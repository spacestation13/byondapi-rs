use crate::prelude::*;
use crate::static_global::byond;
use crate::Error;

use std::ffi::CString;

/// Calls a global proc.
///
/// Implicitly set waitfor=0, will never block unless called from another thread.
pub fn call_global<T: Into<Vec<u8>>>(name: T, args: &[ByondValue]) -> Result<ByondValue, Error> {
    let c_string = CString::new(name).unwrap();

    unsafe {
        let mut new_value = std::mem::MaybeUninit::zeroed();
        map_byond_error!(byond().Byond_CallGlobalProc(
            c_string.as_ptr(),
            args.as_ptr().cast(),
            args.len() as u32,
            new_value.as_mut_ptr()
        ))?;
        Ok(ByondValue(new_value.assume_init()))
    }
}

/// Calls a global proc by its string id.
///
/// Implicitly set waitfor=0, will never block unless called from another thread.
pub fn call_global_id(name: u32, args: &[ByondValue]) -> Result<ByondValue, Error> {
    unsafe {
        let mut new_value = std::mem::MaybeUninit::zeroed();
        map_byond_error!(byond().Byond_CallGlobalProcByStrId(
            name,
            args.as_ptr().cast(),
            args.len() as u32,
            new_value.as_mut_ptr()
        ))?;
        Ok(ByondValue(new_value.assume_init()))
    }
}
