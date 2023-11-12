use super::ByondValue;
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
        let typ = format!("{:X}", self.0.type_);
        let value = format!("{:X}", unsafe { self.0.data.ref_ });

        f.debug_tuple("ByondValue")
            .field(&typ)
            .field(&value)
            .finish()
    }
}
