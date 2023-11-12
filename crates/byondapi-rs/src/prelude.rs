//! This prelude exposes all of the common types and functions end libraries will end up using.

// We re-export some types from byondapi_sys that end libraries will end up needing.

pub use crate::sys;

// Number types
pub use crate::sys::s1c;
pub use crate::sys::s1cMAX;
pub use crate::sys::s1cMIN;
pub use crate::sys::s2c;
pub use crate::sys::s2cMAX;
pub use crate::sys::s2cMIN;
pub use crate::sys::s4c;
pub use crate::sys::s4cMAX;
pub use crate::sys::s4cMIN;
pub use crate::sys::u1c;
// pub use crate::sys::u1cMAX;
// pub use crate::sys::u1cMIN;
pub use crate::sys::u2c;
// pub use crate::sys::u2cMAX;
// pub use crate::sys::u2cMIN;
pub use crate::sys::u4c;
// pub use crate::sys::u4cMAX;
// pub use crate::sys::u4cMIN;
pub use crate::sys::u8c;
// pub use crate::sys::u8cMAX;
// pub use crate::sys::u8cMIN;
pub use crate::sys::s8c;
// pub use crate::sys::s8cMAX;
// pub use crate::sys::s8cMIN;

// Other types
pub use byondapi_sys::u4cOrPointer;
pub use byondapi_sys::ByondValueData as InternalByondValueData;
pub use byondapi_sys::ByondValueType as InternalByondValueType;
pub use byondapi_sys::CByondValue as InternalByondValue;

// As well as our own types.
pub use crate::value::ByondValue;
