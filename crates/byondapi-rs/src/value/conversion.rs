use std::ffi::{CStr, CString};

use super::ByondValue;
use crate::{static_global::BYOND, typecheck_trait::ByondTypeCheck, Error};

// From Impls
impl From<f32> for ByondValue {
    fn from(value: f32) -> Self {
        ByondValue::new_num(value)
    }
}

impl TryFrom<&str> for ByondValue {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        ByondValue::new_str(value)
    }
}

impl TryFrom<String> for ByondValue {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        ByondValue::new_str(value)
    }
}

// TryFrom ByondValue -> x impls
impl TryFrom<ByondValue> for f32 {
    type Error = Error;

    fn try_from(value: ByondValue) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}

impl TryFrom<&ByondValue> for f32 {
    type Error = Error;

    fn try_from(value: &ByondValue) -> Result<Self, Self::Error> {
        if value.is_num() {
            Ok(unsafe { BYOND.ByondValue_GetNum(&value.0) })
        } else {
            Err(Error::InvalidConversion)
        }
    }
}

impl TryFrom<ByondValue> for CString {
    type Error = Error;

    fn try_from(value: ByondValue) -> Result<Self, Self::Error> {
        value.try_into()
    }
}

impl TryFrom<&ByondValue> for CString {
    type Error = Error;

    fn try_from(value: &ByondValue) -> Result<Self, Self::Error> {
        if value.is_str() {
            let ptr = unsafe { BYOND.ByondValue_GetStr(&value.0) };
            let cstr = unsafe { CStr::from_ptr(ptr) };
            Ok(cstr.to_owned())
        } else {
            Err(Error::InvalidConversion)
        }
    }
}

impl TryFrom<ByondValue> for String {
    type Error = Error;

    fn try_from(value: ByondValue) -> Result<Self, Self::Error> {
        value.try_into()
    }
}

impl TryFrom<&ByondValue> for String {
    type Error = Error;

    fn try_from(value: &ByondValue) -> Result<Self, Self::Error> {
        let cstring: CString = value.try_into()?;
        cstring.into_string().map_err(|_| Error::NonUtf8String)
    }
}
