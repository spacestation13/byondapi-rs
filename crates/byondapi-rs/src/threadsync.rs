use crate::static_global::byond;
use crate::value::ByondValue;
use byondapi_sys::CByondValue;
use std::os::raw::c_void;

extern "C-unwind" fn trampoline<F: FnOnce() -> ByondValue + Send>(
    data: *mut c_void,
) -> CByondValue {
    let data = unsafe { Box::from_raw(data as *mut F) };
    unsafe { data().into_inner() }
}

/// Calls the function `callback` on the main thread, this just calls it and does nothing if its
/// already on the main thread.
///
/// If not, then the callback gets added to a queue to be called later by byond, And :
/// If `block` is true then `thread_sync` will block the calling thread until the callback returns and returns the callbacks value,
/// If `block` is false then `thread_sync` will just returns null and wont block you.
///
/// I am unsure where the return value for callback goes in the `block` is false case
pub fn thread_sync<F>(callback: F, block: bool) -> ByondValue
where
    F: FnOnce() -> ByondValue + Send + 'static,
{
    let data = Box::new(callback);
    let data_ptr = Box::into_raw(data);

    ByondValue(unsafe { byond().Byond_ThreadSync(Some(trampoline::<F>), data_ptr.cast(), block) })
}
