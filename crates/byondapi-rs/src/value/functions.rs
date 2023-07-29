use super::*;

/// # Compatibility with the C++ API
impl ByondValue {
    /// This is available for when you really really need access to the raw [`CByondValue`] but you shouldn't use this
    /// normally.
    pub fn into_inner(mut self) -> CByondValue {
        std::mem::replace(&mut self.0, unsafe { std::mem::zeroed() })
    }

    /// Try to get an [`f32`] or fail if this isn't a number type
    pub fn get_number(&self) -> Result<f32, Error> {
        self.try_into()
    }

    /// Try to get a [`CString`] or fail if this isn't a string type
    pub fn get_cstring(&self) -> Result<CString, Error> {
        self.try_into()
    }

    /// Try to get a [`String`] or fail if this isn't a string type or isn't utf8
    pub fn get_string(&self) -> Result<String, Error> {
        self.try_into()
    }

    /// Get the underlying ref number to this value
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
    /// Replaces whatever is currently in this value with a number
    pub fn set_number(&mut self, f: f32) {
        unsafe { BYOND.ByondValue_SetNum(&mut self.0, f) }
    }

    /// Replaces whatever is currently in this value with a string
    pub fn set_str(&mut self, f: &str) -> Result<(), Error> {
        let c_string = CString::new(f).unwrap();
        let c_str = c_string.as_c_str();
        unsafe { map_byond_error!(BYOND.ByondValue_SetStr(&mut self.0, c_str.as_ptr())) }
    }

    /// Replaces whatever is currently in this value with a ref
    pub fn set_ref(&mut self, type_: ByondValueType, ref_: u4c) {
        unsafe { BYOND.ByondValue_SetRef(&mut self.0, type_, ref_) }
    }
}

/// # Accessors
impl ByondValue {
    /// Read a variable through the ref. Fails if this isn't a ref type.
    pub fn read_var(&self, name: &str) -> Result<ByondValue, Error> {
        let c_string = CString::new(name).unwrap();
        let c_str = c_string.as_c_str();

        let mut new_value = ByondValue::new();

        unsafe {
            map_byond_error!(BYOND.Byond_ReadVar(&self.0, c_str.as_ptr(), &mut new_value.0))?;
        }

        Ok(new_value)
    }

    /// Write to a variable through the ref. Fails if this isn't a ref type.
    pub fn write_var(&mut self, name: &str, other: &ByondValue) -> Result<(), Error> {
        let c_string = CString::new(name).unwrap();
        let c_str = c_string.as_c_str();

        unsafe { map_byond_error!(BYOND.Byond_WriteVar(&self.0, c_str.as_ptr(), &other.0)) }
    }

    /// Call a proc using self as src. Fails if this isn't a ref type.
    ///
    /// Implicitly set waitfor=0, will never block.
    ///
    /// # WARNING FOR BYOND 515.1610
    /// This is treated as verb name, so underscores are replaced with spaces.
    /// For example `/obj/proc/get_name` would have to be called as `obj.call("get name")`.
    pub fn call(&self, name: &str, args: &[ByondValue]) -> Result<ByondValue, Error> {
        let c_string = CString::new(name).unwrap();
        let c_str = c_string.as_c_str();

        let str_id = unsafe { BYOND.Byond_GetStrId(c_str.as_ptr()) };
        if str_id == 0 {
            return Err(Error::InvalidProc);
        }

        let ptr = args.as_ptr();
        let mut new_value = ByondValue::new();
        unsafe {
            map_byond_error!(BYOND.Byond_CallProcByStrId(
                &self.0,
                str_id,
                ptr as *const byondapi_sys::CByondValue,
                args.len() as u32,
                &mut new_value.0
            ))?;
        }

        Ok(new_value)
    }
}
