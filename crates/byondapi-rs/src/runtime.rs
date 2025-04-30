use crate::static_global::byond;
use std::ffi::CString;

/// Immediately returns a runtime from this context.
///
/// ## This function will immediately longjump to byond, Drop destructors or catch_unwind will be ignored.
/// ## Make sure you drop everything before you call this.
pub unsafe fn byond_runtime<S: Into<Vec<u8>>>(message: S) -> ! {
    let c_str = CString::new(message.into()).unwrap();
    unsafe { byond().Byond_CRASH(c_str.as_ptr()) };
    unreachable!()
}
