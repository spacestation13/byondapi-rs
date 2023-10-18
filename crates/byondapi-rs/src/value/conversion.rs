use std::ffi::CString;

use super::ByondValue;
use crate::Error;

// From Impls
impl From<bool> for ByondValue {
    fn from(value: bool) -> Self {
        if value {
            ByondValue::new_num(1.0)
        } else {
            ByondValue::new_num(0.0)
        }
    }
}

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
impl TryFrom<ByondValue> for bool {
    type Error = Error;

    fn try_from(value: ByondValue) -> Result<Self, Self::Error> {
        value.get_bool()
    }
}

impl TryFrom<&ByondValue> for bool {
    type Error = Error;

    fn try_from(value: &ByondValue) -> Result<Self, Self::Error> {
        value.get_bool()
    }
}

impl TryFrom<ByondValue> for f32 {
    type Error = Error;

    fn try_from(value: ByondValue) -> Result<Self, Self::Error> {
        value.get_number()
    }
}

impl TryFrom<&ByondValue> for f32 {
    type Error = Error;

    fn try_from(value: &ByondValue) -> Result<Self, Self::Error> {
        value.get_number()
    }
}

impl TryFrom<ByondValue> for CString {
    type Error = Error;

    fn try_from(value: ByondValue) -> Result<Self, Self::Error> {
        value.get_cstring()
    }
}

impl TryFrom<&ByondValue> for CString {
    type Error = Error;

    fn try_from(value: &ByondValue) -> Result<Self, Self::Error> {
        value.get_cstring()
    }
}

impl TryFrom<ByondValue> for String {
    type Error = Error;

    fn try_from(value: ByondValue) -> Result<Self, Self::Error> {
        value.get_string()
    }
}

impl TryFrom<&ByondValue> for String {
    type Error = Error;

    fn try_from(value: &ByondValue) -> Result<Self, Self::Error> {
        value.get_string()
    }
}
