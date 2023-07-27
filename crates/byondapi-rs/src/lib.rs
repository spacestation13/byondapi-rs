#[macro_use]
extern crate lazy_static;

mod static_global;

#[macro_use]
pub mod error;
pub use error::Error;

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
