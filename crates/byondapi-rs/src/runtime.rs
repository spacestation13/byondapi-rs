use crate::static_global::byond;
use std::ffi::CString;

/// Immediately returns a runtime from this context
///
/// # WARNING, THIS WILL NOT CALL DROP ON THINGS, YOU MUST DROP EVERYTHING BEFORE YOU CALL THIS FUNCTION
pub unsafe fn byond_runtime<S: Into<Vec<u8>>>(message: S) -> ! {
    let c_str = CString::new(message.into()).unwrap();
    unsafe { byond().Byond_CRASH(c_str.as_ptr()) };
    panic!()
}
