use std::ffi::{CStr, CString};

use byondapi_sys::{u4c, ByondValueType, CByondValue};

use super::ByondValue;
use crate::{
    map::byond_length, prelude::ByondValueList, static_global::byond,
    typecheck_trait::ByondTypeCheck, Error,
};

/// # Compatibility with the C++ API
impl ByondValue {
    /// This is available for when you really really need access to the raw [`CByondValue`] but you shouldn't use this
    /// normally.
    pub fn into_inner(mut self) -> CByondValue {
        std::mem::replace(&mut self.0, unsafe { std::mem::zeroed() })
    }

    /// Try to get a [`bool`] or fail if this isn't a number type
    pub fn get_bool(&self) -> Result<bool, Error> {
        self.get_number().map(|num| match num as u32 {
            (1..) => true,
            0 => false,
        })
    }

    /// Try to get an [`f32`] or fail if this isn't a number type
    pub fn get_number(&self) -> Result<f32, Error> {
        if self.is_num() {
            Ok(unsafe { byond().ByondValue_GetNum(&self.0) })
        } else {
            Err(Error::InvalidConversion)
        }
    }

    /// Try to get a [`CString`] or fail if this isn't a string type
    pub fn get_cstring(&self) -> Result<CString, Error> {
        if self.is_str() {
            let ptr = unsafe { byond().ByondValue_GetStr(&self.0) };
            let cstr = unsafe { CStr::from_ptr(ptr) };
            Ok(cstr.to_owned())
        } else {
            Err(Error::InvalidConversion)
        }
    }

    /// Try to get a [`String`] or fail if this isn't a string type or isn't utf8
    pub fn get_string(&self) -> Result<String, Error> {
        self.get_cstring().map(|cstring| {
            cstring
                .to_str()
                .map_err(|_| Error::NonUtf8String)
                .map(str::to_owned)
        })?
    }

    /// Try to get a [`crate::prelude::ByondValueList`] or fail if this isn't a string type or isn't utf8
    pub fn get_list(&self) -> Result<ByondValueList, Error> {
        let mut new_list = ByondValueList::new();

        unsafe { map_byond_error!(byond().Byond_ReadList(&self.0, &mut new_list.0))? }

        Ok(new_list)
    }

    /// Get the underlying ref number to this value
    pub fn get_ref(&self) -> Result<u4c, Error> {
        if self.is_str() || self.is_null() || self.is_num() {
            return Err(Error::NotReferencable);
        }
        Ok(unsafe { byond().ByondValue_GetRef(&self.0) })
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

/// # Accessors by ids
impl ByondValue {
    /// Read a variable through the ref. Fails if this isn't a ref type, or the id is invalid.
    pub fn read_var_id(&self, name: u4c) -> Result<ByondValue, Error> {
        let mut new_value = ByondValue::new();
        unsafe {
            map_byond_error!(byond().Byond_ReadVarByStrId(&self.0, name, &mut new_value.0))?;
        }

        Ok(new_value)
    }

    /// Write to a variable through the ref. Fails if this isn't a ref type, or the id is invalid.
    pub fn write_var_id(&mut self, name: u4c, other: &ByondValue) -> Result<(), Error> {
        unsafe { map_byond_error!(byond().Byond_WriteVarByStrId(&self.0, name, &other.0)) }
    }

    /// Call a proc using self as src. Fails if this isn't a ref type, or the id is invalid.
    ///
    /// Implicitly set waitfor=0, will never block.
    ///
    /// # WARNING FOR BYOND 515.1609 and 515.1610
    /// This is treated as verb name, so underscores are replaced with spaces.
    /// For example `/obj/proc/get_name` would have to be called as `obj.call("get name")`.
    pub fn call_id(&self, name: u4c, args: &[ByondValue]) -> Result<ByondValue, Error> {
        let ptr = args.as_ptr();
        let mut new_value = ByondValue::new();
        unsafe {
            map_byond_error!(byond().Byond_CallProcByStrId(
                &self.0,
                name,
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
        if !self.is_list() {
            return Err(Error::NotAList);
        }
        let index: ByondValue = index.try_into().map_err(|_| Error::InvalidConversion)?;
        self.read_list_index_internal(&index)
    }

    /// Writes a value by key through the ref. Fails if this isn't a list.
    pub fn write_list_index<I: TryInto<ByondValue>, V: TryInto<ByondValue>>(
        &mut self,
        index: I,
        value: V,
    ) -> Result<(), Error> {
        if !self.is_list() {
            return Err(Error::NotAList);
        }
        let index: ByondValue = index.try_into().map_err(|_| Error::InvalidConversion)?;
        let value: ByondValue = value.try_into().map_err(|_| Error::InvalidConversion)?;
        self.write_list_index_internal(&index, &value)
    }

    /// Reads a value by key through the ref. Fails if the index doesn't exist
    pub fn read_list_index_internal(&self, index: &ByondValue) -> Result<ByondValue, Error> {
        let mut result = ByondValue::new();
        unsafe {
            map_byond_error!(byond().Byond_ReadListIndex(&self.0, &index.0, &mut result.0))?;
        }
        Ok(result)
    }

    /// Writes a value by key through the ref. Dunno why it can fail
    pub fn write_list_index_internal(
        &mut self,
        index: &ByondValue,
        value: &ByondValue,
    ) -> Result<(), Error> {
        unsafe {
            map_byond_error!(byond().Byond_WriteListIndex(&self.0, &index.0, &value.0))?;
        }
        Ok(())
    }
}

/// # Builtins
impl ByondValue {
    pub fn builtin_length(&self) -> Result<ByondValue, Error> {
        let mut result = ByondValue::new();
        unsafe {
            map_byond_error!(byond().Byond_Length(&self.0, &mut result.0))?;
        }
        Ok(result)
    }
}

/// # Helpers
impl ByondValue {
    /// Reads a number through the ref. Fails if this isn't a ref type or this isn't a number.
    pub fn read_number<T: Into<Vec<u8>>>(&self, name: T) -> Result<f32, Error> {
        self.read_var(name)?.get_number()
    }

    /// Reads a string through the ref. Fails if this isn't a ref type or this isn't a string.
    pub fn read_string<T: Into<Vec<u8>>>(&self, name: T) -> Result<String, Error> {
        self.read_var(name)?.get_string()
    }

    /// Reads a list through the ref. Fails if this isn't a ref type or this isn't a list.
    pub fn read_list<T: Into<Vec<u8>>>(
        &self,
        name: T,
    ) -> Result<crate::prelude::ByondValueList, Error> {
        self.read_var(name)?.try_into()
    }

    /// Iterates through the assoc values of the list if this value is a list, if the value isn't a list then it returns an error.
    /// Non assoc lists will have the second field of the tuple be null
    /// (key, value) for proper assoc lists
    pub fn iter(&self) -> Result<impl Iterator<Item = (ByondValue, ByondValue)> + '_, Error> {
        if !self.is_list() {
            return Err(Error::NotAList);
        }
        let len: f32 = byond_length(self)?.try_into()?;
        Ok(ListIterator {
            value: self,
            len: len as u32,
            ctr: 1,
        })
    }
    pub fn values(&self) -> Result<impl Iterator<Item = ByondValue> + '_, Error> {
        if !self.is_list() {
            return Err(Error::NotAList);
        }
        let len: f32 = byond_length(self)?.try_into()?;
        Ok(ValueIterator {
            value: self,
            len: len as u32,
            ctr: 1,
        })
    }
}

struct ValueIterator<'a> {
    value: &'a ByondValue,
    len: u32,
    ctr: u32,
}
impl<'a> Iterator for ValueIterator<'a> {
    type Item = ByondValue;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ctr <= self.len {
            let value = self
                .value
                .read_list_index_internal(&ByondValue::from(self.ctr as f32))
                .ok()?;
            self.ctr += 1;
            Some(value)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.len as usize))
    }
}

struct ListIterator<'a> {
    value: &'a ByondValue,
    len: u32,
    ctr: u32,
}
impl<'a> Iterator for ListIterator<'a> {
    type Item = (ByondValue, ByondValue);
    fn next(&mut self) -> Option<Self::Item> {
        if self.ctr <= self.len {
            let key = self
                .value
                .read_list_index_internal(&ByondValue::from(self.ctr as f32))
                .ok()?;
            let value = self.value.read_list_index_internal(&key).ok()?;
            self.ctr += 1;
            Some((key, value))
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.len as usize))
    }
}
