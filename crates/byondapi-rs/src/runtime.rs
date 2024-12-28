use crate::static_global::byond;
use std::ffi::CString;

pub fn byond_runtime<S: Into<Vec<u8>>>(message: S) {
    let c_str = CString::new(message.into()).unwrap();
    unsafe { byond().Byond_CRASH(c_str.as_ptr()) }
}
