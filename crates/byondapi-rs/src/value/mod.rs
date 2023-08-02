//! [Newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) pattern over [`CByondValue`]
use std::{
    ffi::{CStr, CString},
    mem::MaybeUninit,
};

use byondapi_sys::{u4c, ByondValueType, CByondValue};

use crate::{error::Error, static_global::BYOND, typecheck_trait::ByondTypeCheck};

/// [Newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) pattern over [`CByondValue`]
#[repr(transparent)]
pub struct ByondValue(pub CByondValue);

/// It is safe to send ByondValue with ownership, but it is not safe to have references between threads.
unsafe impl Send for ByondValue {}

pub mod constructors;
pub mod functions;
pub mod pointer;
pub mod trait_impls;

// Typechecking
impl ByondTypeCheck for ByondValue {
    fn get_type(&self) -> ByondValueType {
        // Safety: This operation only fails if our CByondValue is invalid, which cannot happen.
        unsafe { BYOND.ByondValue_Type(&self.0) }
    }

    fn is_null(&self) -> bool {
        // Safety: This operation only fails if our CByondValue is invalid, which cannot happen.
        unsafe { BYOND.ByondValue_IsNull(&self.0) }
    }

    fn is_num(&self) -> bool {
        // Safety: This operation only fails if our CByondValue is invalid, which cannot happen.
        unsafe { BYOND.ByondValue_IsNum(&self.0) }
    }

    fn is_str(&self) -> bool {
        // Safety: This operation only fails if our CByondValue is invalid, which cannot happen.
        unsafe { BYOND.ByondValue_IsStr(&self.0) }
    }

    fn is_list(&self) -> bool {
        // Safety: This operation only fails if our CByondValue is invalid, which cannot happen.
        unsafe { BYOND.ByondValue_IsList(&self.0) }
    }
}
