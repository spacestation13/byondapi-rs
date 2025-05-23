mod static_global;

///Macros
pub use byondapi_macros;
pub use byondapi_macros::bind;
pub use byondapi_macros::bind_macro;
pub use byondapi_macros::bind_raw_args;
pub use byondapi_macros::init;

pub use binds::generate_bindings;

pub use inventory;

#[macro_use]
pub mod error;
pub mod map;
#[cfg(feature = "byond-516-1651")]
pub mod pixloc;
#[cfg(feature = "byond-516-1651")]
pub mod runtime;
pub use error::Error;

pub mod binds;
pub mod byond_string;
pub mod global_call;
pub mod prelude;
pub mod threadsync;
pub mod value;

use crate::value::ByondValue;
/// # Safety
/// Don't pass in a null argv pointer please god
/// UPDATE: it does pass in a null argv in cases where there are no args
/// Just give this what BYOND gives you and pray for the best
pub unsafe fn parse_args(
    argc: byondapi_sys::u4c,
    argv: *mut ByondValue,
) -> &'static mut [ByondValue] {
    //oh fuck off
    if argv.is_null() {
        return unsafe {
            std::slice::from_raw_parts_mut(std::ptr::NonNull::<ByondValue>::dangling().as_ptr(), 0)
        };
    }
    unsafe { std::slice::from_raw_parts_mut(argv, argc as usize) }
}

/// Re-export of byondapi_sys for all low level things you may run into.
pub mod sys {
    pub use byondapi_sys::*;
}

inventory::collect!(InitFunc);

///This function will be ran to set up things before the lib is loaded
///The lib is only loaded when any byondapi functions are called from byond
///To submit a function (func) to be ran by byondapi on it's libload, do:
///```
///byondapi::byondapi_macros::inventory::submit! {InitFunc(func)}
///```
///Or add a #[byondapi::init] attribute to a function
pub struct InitFunc(pub fn() -> ());

///This macro caches string ids and returns it instead of doing a stringid lookup everytime
///The macro will panic if the string doesn't already exist on byond init lib
///Example usage:
///```
///byondapi::call_global_id(byond_string!("get_name"),&[]).unwrap()
///```
#[macro_export]
macro_rules! byond_string {
    ($s:literal) => {{
        static STRING_ID: ::std::sync::OnceLock<u32> = ::std::sync::OnceLock::new();
        *STRING_ID.get_or_init(|| $crate::byond_string::str_id_of($s).unwrap())
    }};
}
