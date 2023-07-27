use std::{
    ffi::{CStr, CString},
    mem::MaybeUninit,
};

use byondapi_sys::{u4c, ByondValueType, CByondValue};

use crate::{error::Error, static_global::BYOND, typecheck_trait::ByondTypeCheck};

#[repr(transparent)]
pub struct ByondValue(pub(crate) CByondValue);

pub mod constructors;
pub mod functions;
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
