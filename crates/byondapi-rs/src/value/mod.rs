//! [Newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) pattern over [`CByondValue`]

use byondapi_sys::{ByondValueType, CByondValue};

use crate::{static_global::byond, typecheck_trait::ByondTypeCheck};

/// [Newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) pattern over [`CByondValue`]
#[repr(transparent)]
pub struct ByondValue(pub CByondValue);

/// It is safe to send ByondValue with ownership, but it is not safe to have references between threads.
unsafe impl Send for ByondValue {}

pub mod constructors;
pub mod conversion;
pub mod functions;
pub mod pointer;
pub mod trait_impls;

/// FIXME: Use a Byond_IsPtr here instead of checking the type by hand
fn is_pointer_shim(value: &ByondValue) -> bool {
    let type_ = value.get_type();
    type_ == 0x3C
}

// Typechecking
impl ByondTypeCheck for ByondValue {
    fn get_type(&self) -> ByondValueType {
        // Safety: This operation only fails if our CByondValue is invalid, which cannot happen.
        unsafe { byond().ByondValue_Type(&self.0) }
    }

    fn is_null(&self) -> bool {
        // Safety: This operation only fails if our CByondValue is invalid, which cannot happen.
        unsafe { byond().ByondValue_IsNull(&self.0) }
    }

    fn is_num(&self) -> bool {
        // Safety: This operation only fails if our CByondValue is invalid, which cannot happen.
        unsafe { byond().ByondValue_IsNum(&self.0) }
    }

    fn is_str(&self) -> bool {
        // Safety: This operation only fails if our CByondValue is invalid, which cannot happen.
        unsafe { byond().ByondValue_IsStr(&self.0) }
    }

    fn is_list(&self) -> bool {
        // Safety: This operation only fails if our CByondValue is invalid, which cannot happen.
        unsafe { byond().ByondValue_IsList(&self.0) }
    }

    fn is_ptr(&self) -> bool {
        is_pointer_shim(self)
    }
}
