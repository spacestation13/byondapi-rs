use super::ByondValue;
use crate::{static_global::byond, Error};

impl ByondValue {
    /// Try to get a length of a string in bytes, lists in number of assoc elements probably, will fail if it's neither a list or string
    pub fn builtin_length(&self) -> Result<ByondValue, Error> {
        let mut result = ByondValue::default();
        unsafe {
            map_byond_error!(byond().Byond_Length(&self.0, &mut result.0))?;
        }
        Ok(result)
    }
    /// Try to create a new byond object, equivalent to byond's new
    pub fn builtin_new(
        value_type: ByondValue,
        arglist: &[ByondValue],
    ) -> Result<ByondValue, Error> {
        let mut result = ByondValue::default();
        unsafe {
            map_byond_error!(byond().Byond_New(
                &value_type.0,
                arglist.as_ptr().cast(),
                arglist.len() as u32,
                &mut result.0
            ))?;
        }
        Ok(result)
    }
    /// Try to create a new byond object, equivalent to byond's new, but takes a list as arguments instead
    pub fn builtin_newarglist(
        value_type: ByondValue,
        arglist: ByondValue,
    ) -> Result<ByondValue, Error> {
        if !arglist.is_list() {
            return Err(Error::NotAList(arglist));
        };
        let mut result = ByondValue::default();
        unsafe {
            map_byond_error!(byond().Byond_NewArglist(&value_type.0, &arglist.0, &mut result.0))?;
        }
        Ok(result)
    }
}
