use super::ByondValue;
use crate::{static_global::byond, Error};

impl ByondValue {
    pub fn builtin_length(&self) -> Result<ByondValue, Error> {
        let mut result = ByondValue::new();
        unsafe {
            map_byond_error!(byond().Byond_Length(&self.0, &mut result.0))?;
        }
        Ok(result)
    }
    pub fn builtin_new(
        value_type: ByondValue,
        arglist: &[ByondValue],
    ) -> Result<ByondValue, Error> {
        let mut result = ByondValue::new();
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

    pub fn builtin_newarglist(
        value_type: ByondValue,
        arglist: ByondValue,
    ) -> Result<ByondValue, Error> {
        if !arglist.is_list() {
            return Err(Error::NotAList(arglist));
        };
        let mut result = ByondValue::new();
        unsafe {
            map_byond_error!(byond().Byond_NewArglist(&value_type.0, &arglist.0, &mut result.0))?;
        }
        Ok(result)
    }
}
