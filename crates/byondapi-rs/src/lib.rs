mod static_global;

///Macros
pub use byondapi_macros;

pub use byondapi_macros::bind;
pub use byondapi_macros::bind_raw_args;
pub use byondapi_macros::init;

#[macro_use]
pub mod error;
pub mod map;
pub use error::Error;

pub mod byond_string;
pub mod global_call;
pub mod prelude;
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
///byondapi::byondapi_macros::inventory::submit! {InitFunc(func)}
///```
///Or add a #[byondapi::init] attribute to a function
pub struct InitFunc(pub fn() -> ());

///Custom error type for binds, just to implement From for ?
pub struct BindError(pub String);

impl<E> From<E> for BindError
where
    E: std::fmt::Debug,
{
    fn from(value: E) -> Self {
        BindError(format!("{value:#?}"))
    }
}

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
