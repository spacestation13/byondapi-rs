use std::ffi::CString;

use byondapi_sys::{u4c, ByondValueType, CByondValue};

use super::ByondValue;
use crate::{static_global::byond, Error};

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
        // ByondValue_GetRef already checks our type to make sure we are a ref.
        let ref_ = unsafe { byond().ByondValue_GetRef(&self.0) };
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
        unsafe { byond().ByondValue_SetNum(&mut self.0, f) }
    }

    /// Replaces whatever is currently in this value with a string
    pub fn set_str<T: Into<Vec<u8>>>(&mut self, f: T) -> Result<(), Error> {
        let c_string = CString::new(f).unwrap();
        let c_str = c_string.as_c_str();
        unsafe { map_byond_error!(byond().ByondValue_SetStr(&mut self.0, c_str.as_ptr())) }
    }

    /// Replaces whatever is currently in this value with a ref
    pub fn set_ref(&mut self, type_: ByondValueType, ref_: u4c) {
        unsafe { byond().ByondValue_SetRef(&mut self.0, type_, ref_) }
    }
}

/// # Accessors
impl ByondValue {
    /// Read a variable through the ref. Fails if this isn't a ref type.
    pub fn read_var<T: Into<Vec<u8>>>(&self, name: T) -> Result<ByondValue, Error> {
        let c_string = CString::new(name).unwrap();
        let c_str = c_string.as_c_str();

        let mut new_value = ByondValue::new();

        unsafe {
            map_byond_error!(byond().Byond_ReadVar(&self.0, c_str.as_ptr(), &mut new_value.0))?;
        }

        Ok(new_value)
    }

    /// Write to a variable through the ref. Fails if this isn't a ref type.
    pub fn write_var<T: Into<Vec<u8>>>(
        &mut self,
        name: T,
        other: &ByondValue,
    ) -> Result<(), Error> {
        let c_string = CString::new(name).unwrap();
        let c_str = c_string.as_c_str();

        unsafe { map_byond_error!(byond().Byond_WriteVar(&self.0, c_str.as_ptr(), &other.0)) }
    }

    /// Call a proc using self as src. Fails if this isn't a ref type.
    ///
    /// Implicitly set waitfor=0, will never block.
    ///
    /// # WARNING FOR BYOND 515.1609 and 515.1610
    /// This is treated as verb name, so underscores are replaced with spaces.
    /// For example `/obj/proc/get_name` would have to be called as `obj.call("get name")`.
    pub fn call<T: Into<Vec<u8>>>(
        &self,
        name: T,
        args: &[ByondValue],
    ) -> Result<ByondValue, Error> {
        let c_string = CString::new(name).unwrap();
        let c_str = c_string.as_c_str();

        let str_id = unsafe { byond().Byond_GetStrId(c_str.as_ptr()) };
        if str_id == 0 {
            return Err(Error::InvalidProc);
        }

        let ptr = args.as_ptr();
        let mut new_value = ByondValue::new();
        unsafe {
            map_byond_error!(byond().Byond_CallProcByStrId(
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

/// # List operations by key instead of indices (why are they even here lumlum?????)
impl ByondValue {
    /// Reads a value by key through the ref. Fails if this isn't a list.
    pub fn read_list_index<I: TryInto<ByondValue>>(&self, index: I) -> Result<ByondValue, Error> {
        let index: ByondValue = index.try_into().map_err(|_| Error::InvalidConversion)?;
        let mut result = ByondValue::new();
        unsafe {
            map_byond_error!(byond().Byond_ReadListIndex(&self.0, &index.0, &mut result.0))?;
        }
        Ok(result)
    }

    /// Writes a value by key through the ref. Fails if this isn't a list.
    pub fn write_list_index<I: TryInto<ByondValue>, V: TryInto<ByondValue>>(
        &self,
        index: I,
        value: V,
    ) -> Result<(), Error> {
        let index: ByondValue = index.try_into().map_err(|_| Error::InvalidConversion)?;
        let value: ByondValue = value.try_into().map_err(|_| Error::InvalidConversion)?;
        unsafe {
            map_byond_error!(byond().Byond_WriteListIndex(&self.0, &index.0, &value.0))?;
        }
        Ok(())
    }
}

/// # Helpers
impl ByondValue {
    /// Reads a number through the ref. Fails if this isn't a ref type or this isn't a number.
    pub fn read_number<T: Into<Vec<u8>>>(&self, name: T) -> Result<f32, Error> {
        self.read_var(name)?.try_into()
    }

    /// Reads a string through the ref. Fails if this isn't a ref type or this isn't a string.
    pub fn read_string<T: Into<Vec<u8>>>(&self, name: T) -> Result<String, Error> {
        self.read_var(name)?.try_into()
    }

    /// Reads a list through the ref. Fails if this isn't a ref type or this isn't a list.
    pub fn read_list<T: Into<Vec<u8>>>(
        &self,
        name: T,
    ) -> Result<crate::prelude::ByondValueList, Error> {
        self.read_var(name)?.try_into()
    }
}
