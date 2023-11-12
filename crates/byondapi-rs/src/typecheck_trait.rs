//! This trait allows checking what a [`crate::value::ByondValue`] actually represents before doing something with it
use byondapi_sys::ByondValueType;

/// This trait allows checking what a [`crate::value::ByondValue`] actually represents before doing something with it
pub trait ByondTypeCheck {
    /// This gets the actual type.
    fn get_type(&self) -> ByondValueType;

    /// Check if this is null.
    fn is_null(&self) -> bool;
    /// Check if this is a number.
    fn is_num(&self) -> bool;
    /// Check if this is a string.
    fn is_str(&self) -> bool;
    /// Check if this is a list.
    fn is_list(&self) -> bool;
    /// Check if this is a pointer.
    fn is_ptr(&self) -> bool;
    #[cfg(any(feature = "byond-515-1620"))]
    /// Check if this is true-ish from byond's view.
    fn is_true(&self) -> bool;
}
