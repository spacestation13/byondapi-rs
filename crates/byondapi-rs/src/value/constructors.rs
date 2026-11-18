use std::ffi::CString;

use byondapi_sys::{u4c, CByondValue};

use super::{types::ValueType, ByondValue};
use crate::{static_global::byond, Error};

impl Default for ByondValue {
    fn default() -> Self {
        ByondValue::null()
    }
}

/// # Constructors
impl ByondValue {
    pub fn null() -> Self {
        let mut value = CByondValue {
            type_: 0,
            junk1: 0,
            junk2: 0,
            junk3: 0,
            data: byondapi_sys::ByondValueData { ref_: 0 },
        };
        unsafe {
            byond().ByondValue_Clear(&mut value);
        }
        Self(value)
    }

    pub fn new_num(f: f32) -> Self {
        let mut value = CByondValue {
            type_: 0,
            junk1: 0,
            junk2: 0,
            junk3: 0,
            data: byondapi_sys::ByondValueData { ref_: 0 },
        };
        unsafe {
            byond().ByondValue_SetNum(&mut value, f);
        };
        Self(value)
    }

    /// # Safety
    /// I have made sure this is a valid ref
    pub unsafe fn new_ref(typ: ValueType, ptr: u4c) -> Self {
        let mut value = CByondValue {
            type_: 0,
            junk1: 0,
            junk2: 0,
            junk3: 0,
            data: byondapi_sys::ByondValueData { ref_: 0 },
        };
        unsafe {
            byond().ByondValue_SetRef(&mut value, typ as u8, ptr);
            byond().ByondValue_IncRef(&value)
        }
        Self(value)
    }

    pub fn new_global_ref() -> Self {
        let value = CByondValue {
            type_: ValueType::World as u8,
            junk1: 0,
            junk2: 0,
            junk3: 0,
            data: byondapi_sys::ByondValueData { ref_: 1 },
        };

        unsafe { byond().ByondValue_IncRef(&value) }
        Self(value)
    }

    pub fn new_str<S: Into<Vec<u8>>>(s: S) -> Result<Self, Error> {
        let c_str = CString::new(s.into()).unwrap();
        let mut value = CByondValue {
            type_: 0,
            junk1: 0,
            junk2: 0,
            junk3: 0,
            data: byondapi_sys::ByondValueData { ref_: 0 },
        };
        unsafe {
            byond().ByondValue_SetStr(&mut value, c_str.as_ptr());
        }
        if unsafe { byond().ByondValue_IsNull(&value) } {
            return Err(Error::UnableToCreateString(c_str));
        }
        unsafe {
            byond().ByondValue_IncRef(&value);
        }
        Ok(Self(value))
    }

    pub fn new_list() -> Result<Self, Error> {
        let mut new_self = Self::default();

        unsafe { map_byond_error!(byond().Byond_CreateList(&mut new_self.0))? }

        Ok(new_self)
    }
}
