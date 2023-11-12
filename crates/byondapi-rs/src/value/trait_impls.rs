use super::ByondValue;
use crate::static_global::byond;
use std::fmt::Debug;

// Memory handling
impl Clone for ByondValue {
    fn clone(&self) -> Self {
        let mut cloned_value = ByondValue::null();

        unsafe {
            byond().ByondValue_CopyFrom(&mut cloned_value.0, &self.0);
        };

        cloned_value
    }
}

impl Drop for ByondValue {
    fn drop(&mut self) {
        // Safety: We are being dropped, it is okay to free our inner CByondValue.
        unsafe { byond().ByondValue_Free(&mut self.0) }
    }
}

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
