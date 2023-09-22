mod static_global;

#[macro_use]
pub mod error;
#[cfg(feature = "byond-515-1611")]
pub mod map;
pub use error::Error;

pub mod list;
pub mod prelude;
pub mod typecheck_trait;
pub mod value;

use crate::value::ByondValue;
/// # Safety
/// Don't pass in a null argv pointer please god
/// Just give this what BYOND gives you and pray for the best
pub unsafe fn parse_args(argc: byondapi_sys::u4c, argv: *mut ByondValue) -> Vec<ByondValue> {
    unsafe { std::slice::from_raw_parts_mut(argv, argc as usize).to_vec() }
}

/// Re-export of byondapi_sys for all low level things you may run into.
pub mod sys {
    pub use byondapi_sys::*;
}
