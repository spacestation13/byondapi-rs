use std::ffi::CString;

use byondapi_sys::{u4c, ByondValueType, CByondValue};

use super::ByondValue;
use crate::{map::byond_length, static_global::byond, Error};

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
            Err(Error::NotANum(*self))
        }
    }

    /// Try to get a [`CString`] or fail if this isn't a string type
    pub fn get_cstring(&self) -> Result<CString, Error> {
        use std::cell::RefCell;
        if !self.is_str() {
            return Err(Error::NotAString(*self));
        }

        thread_local! {
            static BUFFER: RefCell<Vec<u8>> = RefCell::new(Vec::with_capacity(1));
        }

        let bytes = BUFFER.with_borrow_mut(|buff| -> Result<Vec<u8>, Error> {
            let initial_len = buff.capacity() as u32;
            let mut len = buff.capacity() as u32;
            // Safety: buffer capacity is passed to byond, which makes sure it writes in-bound
            let initial_res =
                unsafe { byond().Byond_ToString(&self.0, buff.as_mut_ptr().cast(), &mut len) };
            match (initial_res, len) {
                (false, 1..) => {
                    debug_assert!(len > initial_len);
                    buff.reserve_exact((len - initial_len) as usize);
                    // Safety: buffer capacity is passed to byond, which makes sure it writes in-bound
                    unsafe {
                        map_byond_error!(byond().Byond_ToString(
                            &self.0,
                            buff.as_mut_ptr().cast(),
                            &mut len
                        ))?
                    };
                    // Safety: buffer should be written to at this point
                    unsafe { buff.set_len(len as usize) };
                    Ok(std::mem::take(buff))
                }
                (true, _) => {
                    // Safety: buffer should be written to at this point
                    unsafe { buff.set_len(len as usize) };
                    Ok(std::mem::take(buff))
                }
                (false, 0) => Err(Error::get_last_byond_error()),
            }
        })?;
        CString::from_vec_with_nul(bytes).map_err(|_| Error::NonUtf8String)
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

    /// Get the underlying ref number to this value
    pub fn get_ref(&self) -> Result<u4c, Error> {
        if self.is_str() || self.is_null() || self.is_num() {
            return Err(Error::NotReferencable(*self));
        }
        Ok(unsafe { byond().ByondValue_GetRef(&self.0) })
    }

    /// Get the string id of this value, fail if this isn't a string
    pub fn get_strid(&self) -> Result<u4c, Error> {
        if !self.is_str() {
            Err(Error::NotAString(*self))
        } else {
            Ok(unsafe { self.0.data.ref_ })
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
        unsafe { byond().ByondValue_SetStr(&mut self.0, c_str.as_ptr()) }
        if self.is_null() {
            return Err(Error::UnableToCreateString(c_string));
        }
        Ok(())
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
        if self.is_num() || self.is_str() || self.is_ptr() || self.is_null() || self.is_list() {
            return Err(Error::NotReferencable(*self));
        }
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
        if str_id == crate::sys::u2c::MAX as u32 {
            return Err(Error::InvalidProc(c_string));
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
        if self.is_num() || self.is_str() || self.is_ptr() || self.is_null() || self.is_list() {
            return Err(Error::NotReferencable(*self));
        }
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

/// # Refcount operations
impl ByondValue {
    pub fn increment_ref(&mut self) {
        unsafe { byond().ByondValue_IncRef(&self.0) }
    }

    pub fn decrement_ref(&mut self) {
        unsafe { byond().ByondValue_DecRef(&self.0) }
    }

    pub fn get_refcount(&self) -> Result<u32, Error> {
        let mut result = 0u32;
        unsafe { map_byond_error!(byond().Byond_Refcount(&self.0, &mut result))? };
        Ok(result)
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
    /// Reads a number from a var. Fails if this isn't a ref type or this isn't a number.
    pub fn read_number<T: Into<Vec<u8>>>(&self, name: T) -> Result<f32, Error> {
        self.read_var(name)?.get_number()
    }

    /// Reads a string from a var. Fails if this isn't a ref type or this isn't a string.
    pub fn read_string<T: Into<Vec<u8>>>(&self, name: T) -> Result<String, Error> {
        self.read_var(name)?.get_string()
    }

    /// Reads a list from a var. Fails if this isn't a ref type or this isn't a list.
    pub fn read_list<T: Into<Vec<u8>>>(&self, name: T) -> Result<Vec<ByondValue>, Error> {
        self.read_var(name)?.get_list()
    }

    /// Reads a number from a var id. Fails if this isn't a ref type or this isn't a number.
    pub fn read_number_id(&self, id: u32) -> Result<f32, Error> {
        self.read_var_id(id)?.get_number()
    }

    /// Reads a string from a var id. Fails if this isn't a ref type or this isn't a string.
    pub fn read_string_id(&self, id: u32) -> Result<String, Error> {
        self.read_var_id(id)?.get_string()
    }

    /// Reads a list from a var id. Fails if this isn't a ref type or this isn't a list.
    pub fn read_list_id(&self, id: u32) -> Result<Vec<ByondValue>, Error> {
        self.read_var_id(id)?.get_list()
    }

    /// Iterates through the assoc values of the list if this value is a list, if the value isn't a list then it returns an error.
    /// Non assoc lists will have the second field of the tuple be null
    /// (key, value) for proper assoc lists
    pub fn iter(&self) -> Result<impl Iterator<Item = (ByondValue, ByondValue)> + '_, Error> {
        if !self.is_list() {
            return Err(Error::NotAList(*self));
        }
        let len: f32 = byond_length(self)?.try_into()?;
        Ok(ListIterator {
            value: self,
            len: len as u32,
            ctr: 1,
        })
    }

    /// Iterates through key values of the list if the list is an assoc list, if not, just iterates through values
    pub fn values(&self) -> Result<impl Iterator<Item = ByondValue> + '_, Error> {
        if !self.is_list() {
            return Err(Error::NotAList(*self));
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
