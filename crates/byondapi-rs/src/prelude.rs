//! This prelude exposes all of the common types and functions end libraries will end up using.

// We re-export some types from byondapi_sys that end libraries will end up needing.

// Number types
pub use byondapi_sys::s1c;
pub use byondapi_sys::s1cMAX;
pub use byondapi_sys::s1cMIN;
pub use byondapi_sys::s2c;
pub use byondapi_sys::s2cMAX;
pub use byondapi_sys::s2cMIN;
pub use byondapi_sys::s4c;
pub use byondapi_sys::s4cMAX;
pub use byondapi_sys::s4cMIN;
pub use byondapi_sys::u1c;
// pub use byondapi_sys::u1cMAX;
// pub use byondapi_sys::u1cMIN;
pub use byondapi_sys::u2c;
// pub use byondapi_sys::u2cMAX;
// pub use byondapi_sys::u2cMIN;
pub use byondapi_sys::u4c;
// pub use byondapi_sys::u4cMAX;
// pub use byondapi_sys::u4cMIN;
pub use byondapi_sys::u8c;
// pub use byondapi_sys::u8cMAX;
// pub use byondapi_sys::u8cMIN;
pub use byondapi_sys::s8c;
// pub use byondapi_sys::s8cMAX;
// pub use byondapi_sys::s8cMIN;

// Other types
pub use byondapi_sys::u4cOrPointer;
pub use byondapi_sys::ByondValueData as InternalByondValueData;
pub use byondapi_sys::ByondValueType as InternalByondValueType;
pub use byondapi_sys::CByondValue as InternalByondValue;
pub use byondapi_sys::CByondValueList as InternalByondValueList;

// As well as our own types.
pub use crate::value::ByondValue;
