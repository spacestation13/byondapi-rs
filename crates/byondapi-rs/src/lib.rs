mod static_global;

///Plugin init system
pub use inventory;

#[macro_use]
pub mod error;
pub mod map;
pub use error::Error;

pub mod byond_string;
pub mod global_call;
pub mod list;
pub mod prelude;
pub mod typecheck_trait;
pub mod value;

use crate::value::ByondValue;
/// # Safety
/// Don't pass in a null argv pointer please god
/// Just give this what BYOND gives you and pray for the best
pub unsafe fn parse_args(
    argc: byondapi_sys::u4c,
    argv: *mut ByondValue,
) -> &'static mut [ByondValue] {
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
///byondapi::inventory::submit! {InitFunc(func)}
///```
pub struct InitFunc(pub fn() -> ());

///This macro caches string ids and returns it instead of doing a stringid lookup everytime
///The macro will panic if the string doesn't already exist on byond init lib
#[macro_export]
macro_rules! byond_string {
    ($s:literal) => {{
        thread_local! {
            static STRING_ID: ::std::cell::OnceCell<u32> = ::std::cell::OnceCell::new();
        };
        STRING_ID
            .with(|cell| *cell.get_or_init(|| ::byondapi::byond_string::str_id_of($s).unwrap()))
    }};
}
