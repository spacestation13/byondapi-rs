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
        unsafe {
            let mut value = std::mem::MaybeUninit::uninit();
            byond().ByondValue_Clear(value.as_mut_ptr());
            Self(value.assume_init())
        }
    }

    pub fn new_num(f: f32) -> Self {
        unsafe {
            let mut value = std::mem::MaybeUninit::uninit();
            byond().ByondValue_SetNum(value.as_mut_ptr(), f);
            Self(value.assume_init())
        }
    }

    /// # Safety
    /// I have made sure this is a valid ref
    pub unsafe fn new_ref(typ: ValueType, ptr: u4c) -> Self {
        unsafe {
            let mut value = std::mem::MaybeUninit::uninit();
            byond().ByondValue_SetRef(value.as_mut_ptr(), typ as u8, ptr);
            let value = value.assume_init();

            byond().ByondValue_IncRef(&value);
            Self(value)
        }
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
        unsafe {
            let mut value = std::mem::MaybeUninit::uninit();
            byond().ByondValue_SetStr(value.as_mut_ptr(), c_str.as_ptr());
            let value = value.assume_init();

            if byond().ByondValue_IsNull(&value) {
                return Err(Error::UnableToCreateString(c_str));
            }

            byond().ByondValue_IncRef(&value);
            Ok(Self(value))
        }
    }

    pub fn new_list() -> Result<Self, Error> {
        let mut new_self = Self::default();

        unsafe { map_byond_error!(byond().Byond_CreateList(&mut new_self.0))? }

        Ok(new_self)
    }

    pub fn new_list_with_capacity(len: u32) -> Result<Self, Error> {
        let mut new_self = Self::default();

        unsafe { map_byond_error!(byond().Byond_CreateListLen(&mut new_self.0, len))? }

        Ok(new_self)
    }

    pub fn new_dimensional_list(len: &[u32], dimension: u32) -> Result<Self, Error> {
        let mut new_self = Self::default();

        unsafe {
            map_byond_error!(byond().Byond_CreateDimensionalList(
                &mut new_self.0,
                len.as_ptr(),
                dimension
            ))?
        }

        Ok(new_self)
    }
}
