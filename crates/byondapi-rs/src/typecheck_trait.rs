use byondapi_sys::ByondValueType;

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
}
