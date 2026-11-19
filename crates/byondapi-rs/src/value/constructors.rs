use std::ffi::CString;

use byondapi_sys::CByondValue;

use super::{types::ValueType, ByondValue};
use crate::{static_global::byond, Error};

impl Default for ByondValue {
    fn default() -> Self {
        ByondValue::null()
    }
}

/// # Constructors
impl ByondValue {
    /// Creates a null ByondValue
    pub fn null() -> Self {
        unsafe {
            let mut value = std::mem::MaybeUninit::uninit();
            byond().ByondValue_Clear(value.as_mut_ptr());
            Self(value.assume_init())
        }
    }

    /// Creates a number ByondValue
    pub fn new_num(f: f32) -> Self {
        unsafe {
            let mut value = std::mem::MaybeUninit::uninit();
            byond().ByondValue_SetNum(value.as_mut_ptr(), f);
            Self(value.assume_init())
        }
    }

    /// Creates a reference ByondValue and increments its ref
    pub fn new_ref(typ: ValueType, ptr: u32) -> Self {
        unsafe {
            let mut value = std::mem::MaybeUninit::uninit();
            byond().ByondValue_SetRef(value.as_mut_ptr(), typ as u8, ptr);
            let value = value.assume_init();

            byond().ByondValue_IncRef(&value);
            Self(value)
        }
    }

    /// Creates a global reference ByondValue and increments its ref
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

    /// Creates a string ByondValue and increments its ref
    /// Fails if the string is not a valid utf8 or other byond reasons
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

    /// Creates a list ByondValue
    /// Fails for byond reasons
    pub fn new_list() -> Result<Self, Error> {
        unsafe {
            let mut new_self = std::mem::MaybeUninit::uninit();
            map_byond_error!(byond().Byond_CreateList(new_self.as_mut_ptr()))?;
            Ok(Self(new_self.assume_init()))
        }
    }

    /// Creates a list ByondValue with capacity
    /// Fails for byond reasons
    pub fn new_list_with_capacity(len: u32) -> Result<Self, Error> {
        unsafe {
            let mut new_self = std::mem::MaybeUninit::uninit();
            map_byond_error!(byond().Byond_CreateListLen(new_self.as_mut_ptr(), len))?;
            Ok(Self(new_self.assume_init()))
        }
    }

    /// Creates a multidimensional list with length of each dimension in length,
    /// e.g for [2,2] this function returns a list(list(null, null), list(null, null))
    /// Fails for byond reasons
    pub fn new_dimensional_list(len: &[u32]) -> Result<Self, Error> {
        unsafe {
            let mut new_self = std::mem::MaybeUninit::uninit();
            map_byond_error!(byond().Byond_CreateDimensionalList(
                new_self.as_mut_ptr(),
                len.as_ptr(),
                len.len() as u32
            ))?;
            Ok(Self(new_self.assume_init()))
        }
    }
}
