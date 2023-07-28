use super::*;

/// # Compatibility with the C++ API
impl ByondValue {
    /// Maybe don't use this, questionable
    pub fn into_inner(mut self) -> CByondValue {
        std::mem::replace(&mut self.0, unsafe { std::mem::zeroed() })
    }

    pub fn get_number(&self) -> Result<f32, Error> {
        self.try_into()
    }

    pub fn get_cstring(&self) -> Result<CString, Error> {
        self.try_into()
    }

    pub fn get_string(&self) -> Result<String, Error> {
        self.try_into()
    }

    pub fn get_ref(&self) -> Result<u4c, Error> {
        let ref_ = unsafe { BYOND.ByondValue_GetRef(&self.0) };
        if ref_ != 0 {
            Ok(ref_ as u4c)
        } else {
            Err(Error::InvalidConversion)
        }
    }
}

/// # In-place modifiers
impl ByondValue {
    pub fn set_number(&mut self, f: f32) {
        unsafe { BYOND.ByondValue_SetNum(&mut self.0, f) }
    }

    pub fn set_str(&mut self, f: &str) -> Result<(), Error> {
        let c_string = CString::new(f).unwrap();
        let c_str = c_string.as_c_str();
        unsafe { map_byond_error!(BYOND.ByondValue_SetStr(&mut self.0, c_str.as_ptr())) }
    }

    pub fn set_ref(&mut self, type_: ByondValueType, ref_: u4c) {
        unsafe { BYOND.ByondValue_SetRef(&mut self.0, type_, ref_) }
    }
}

/// # Accessors
impl ByondValue {
    pub fn read_var(&self, name: &str) -> Result<ByondValue, Error> {
        let c_string = CString::new(name).unwrap();
        let c_str = c_string.as_c_str();

        let mut new_value = ByondValue::new();

        unsafe {
            map_byond_error!(BYOND.Byond_ReadVar(&self.0, c_str.as_ptr(), &mut new_value.0))?;
        }

        Ok(new_value)
    }

    pub fn write_var(&mut self, name: &str, other: &ByondValue) -> Result<(), Error> {
        let c_string = CString::new(name).unwrap();
        let c_str = c_string.as_c_str();

        unsafe { map_byond_error!(BYOND.Byond_WriteVar(&self.0, c_str.as_ptr(), &other.0)) }
    }

    pub fn call(&self, name: &str, args: &[ByondValue]) -> Result<ByondValue, Error> {
        let c_string = CString::new(name).unwrap();
        let c_str = c_string.as_c_str();

        let ptr = args.as_ptr();

        let mut new_value = ByondValue::new();

        unsafe {
            map_byond_error!(BYOND.Byond_CallProc(
                &self.0,
                c_str.as_ptr(),
                ptr as *const byondapi_sys::CByondValue,
                args.len() as u32,
                &mut new_value.0
            ))?;
        }

        Ok(new_value)
    }
}
