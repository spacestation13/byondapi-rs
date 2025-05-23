use super::{types::ValueType, ByondValue};
use crate::static_global::byond;
use std::fmt::Debug;

// Equality
impl PartialEq for ByondValue {
    fn eq(&self, other: &Self) -> bool {
        // Safety: This operation only fails if our CByondValue is invalid, which cannot happen.
        unsafe { byond().ByondValue_Equals(&self.0, &other.0) }
    }
}

// Debug!
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
            _ => format!("[{:X}]", unsafe { self.0.data.ref_ }),
        };

        f.debug_tuple("ByondValue")
            .field(&typ)
            .field(&value)
            .finish()
    }
}
