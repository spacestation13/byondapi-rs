use super::{types::ValueType, ByondValue};
use crate::static_global::byond;
use byondapi_sys::CByondValue;
use std::fmt::Debug;

// Equality
// Equivalent to byond's == operator
impl PartialEq for ByondValue {
    fn eq(&self, other: &Self) -> bool {
        // Safety: This operation only fails if our CByondValue is invalid, which cannot happen.
        unsafe { byond().ByondValue_Equals(&self.0, &other.0) }
    }
}

// Debug
impl Debug for ByondValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Ok(type_enum) = self.0.type_.try_into() else {
            return f
                .debug_tuple("ByondValue")
                .field(&format!("Unknown type: {:X}", self.0.type_))
                .finish();
        };
        let typ = format!("{type_enum:?}");

        let value = match type_enum {
            ValueType::Null => "NULL".to_owned(),
            ValueType::Number => format!("{}", unsafe { self.0.data.ref_ as f32 }),
            ValueType::String => self
                .get_string()
                .unwrap_or_else(|_| "Invalid string".to_owned()),
            _ => format!("[{:X}]", unsafe { self.0.data.ref_ }),
        };

        f.debug_tuple("ByondValue")
            .field(&typ)
            .field(&value)
            .finish()
    }
}

impl Clone for ByondValue {
    fn clone(&self) -> Self {
        //incrementing does nothing on non-ref types
        unsafe {
            byond().ByondValue_IncRef(&self.0);
        };
        Self(CByondValue {
            type_: self.0.type_,
            junk1: self.0.junk1,
            junk2: self.0.junk2,
            junk3: self.0.junk3,
            data: self.0.data,
        })
    }
}

// Drop
impl Drop for ByondValue {
    fn drop(&mut self) {
        //decrementing does nothing on non-ref types
        unsafe {
            byond().ByondValue_DecRef(&self.0);
        }
    }
}
