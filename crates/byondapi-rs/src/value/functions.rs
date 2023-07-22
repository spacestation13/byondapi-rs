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
        unsafe { succeeds!(BYOND.ByondValue_SetStr(&mut self.0, c_str.as_ptr())) }
    }

    pub fn set_ref(&mut self, type_: ByondValueType, ref_: u4c) {
        unsafe { BYOND.ByondValue_SetRef(&mut self.0, type_, ref_) }
    }
}
