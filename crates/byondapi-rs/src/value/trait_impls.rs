use super::ByondValue;
use crate::static_global::BYOND;
use std::{fmt::Debug, mem::MaybeUninit};

// Memory handling
impl Clone for ByondValue {
    fn clone(&self) -> Self {
        let mut new_inner = MaybeUninit::uninit();

        let new_inner = unsafe {
            // Safety: new_inner is going to an initialization function, it will only write to the pointer.
            BYOND.ByondValue_CopyFrom(new_inner.as_mut_ptr(), &self.0);
            // Safety: ByondValue_Init will have initialized the new_inner.
            new_inner.assume_init()
        };

        Self(new_inner)
    }
}

impl Drop for ByondValue {
    fn drop(&mut self) {
        // Safety: We are being dropped, it is okay to free our inner CByondValue.
        unsafe { BYOND.ByondValue_Free(&mut self.0) }
    }
}

// Equality
impl PartialEq for ByondValue {
    fn eq(&self, other: &Self) -> bool {
        // Safety: This operation only fails if our CByondValue is invalid, which cannot happen.
        unsafe { BYOND.ByondValue_Equals(&self.0, &other.0) }
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
