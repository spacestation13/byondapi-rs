use std::ffi::CString;

use byondapi_sys::{u4c, ByondValueType, CByondValue};

use super::ByondValue;
use crate::{static_global::byond, Error};

impl Default for ByondValue {
    fn default() -> Self {
        ByondValue::null()
    }
}

/// # Constructors
impl ByondValue {
    pub fn new() -> Self {
        ByondValue::null()
    }

    pub fn null() -> Self {
        Self(CByondValue {
            type_: 0,
            junk1: 0,
            junk2: 0,
            junk3: 0,
            data: byondapi_sys::ByondValueData { ref_: 0 },
        })
    }

    pub fn new_ref(typ: ByondValueType, ptr: u4c) -> Self {
        Self(CByondValue {
            type_: typ,
            junk1: 0,
            junk2: 0,
            junk3: 0,
            data: byondapi_sys::ByondValueData { ref_: ptr },
        })
    }

    pub fn new_num(f: f32) -> Self {
        Self(CByondValue {
            type_: 0x2A,
            junk1: 0,
            junk2: 0,
            junk3: 0,
            data: byondapi_sys::ByondValueData { num: f },
        })
    }

    pub fn new_str<S: Into<Vec<u8>>>(s: S) -> Result<Self, Error> {
        let c_str = CString::new(s.into()).unwrap();
        let str_id = unsafe { byond().Byond_AddGetStrId(c_str.as_ptr()) };
        if str_id == u32::MAX {
            return Err(Error::UnableToCreateString);
        }
        Ok(Self(CByondValue {
            type_: 0x06,
            junk1: 0,
            junk2: 0,
            junk3: 0,
            data: byondapi_sys::ByondValueData { ref_: str_id },
        }))
    }

    pub fn new_list() -> Result<Self, Error> {
        let mut new_self = Self::new();

        unsafe { map_byond_error!(byond().Byond_CreateList(&mut new_self.0))? }

        Ok(new_self)
    }
}

impl<'a> ByondValue {
    /// # Safety
    /// The [`CByondValue`] must be initialized.
    pub unsafe fn from_ref(s: &'a CByondValue) -> &'a Self {
        unsafe { std::mem::transmute(s) }
    }
}
