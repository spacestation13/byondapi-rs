//! [Newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) pattern over [`CByondValue`]

use byondapi_sys::{ByondValueType, CByondValue};

use crate::static_global::byond;

/// [Newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) pattern over [`CByondValue`]
/// WARNING: If this value is a ref created by byond passed to byondapi, it's a temp ref and will be deleted in a while
/// Please convert it into a RcByondValue if you want byond to persist this ref
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ByondValue(pub CByondValue);

/// It is safe to send ByondValue with ownership, but it is not safe to have references between threads.
unsafe impl Send for ByondValue {}

pub mod builtins;
pub mod constructors;
pub mod conversion;
pub mod functions;
pub mod list;
pub mod pointer;
pub mod refcounted;
pub mod trait_impls;
pub mod types;

/// TODO: Use a Byond_IsPtr here instead of checking the type by hand
fn is_pointer_shim(value: &ByondValue) -> bool {
    let type_ = value.get_type();
    type_ == 0x3C
}

// Typechecking
impl ByondValue {
    pub fn get_type(&self) -> ByondValueType {
        // Safety: This operation only fails if our CByondValue is invalid, which cannot happen.
        unsafe { byond().ByondValue_Type(&self.0) }
    }

    pub fn is_null(&self) -> bool {
        // Safety: This operation only fails if our CByondValue is invalid, which cannot happen.
        unsafe { byond().ByondValue_IsNull(&self.0) }
    }

    pub fn is_num(&self) -> bool {
        // Safety: This operation only fails if our CByondValue is invalid, which cannot happen.
        unsafe { byond().ByondValue_IsNum(&self.0) }
    }

    pub fn is_str(&self) -> bool {
        // Safety: This operation only fails if our CByondValue is invalid, which cannot happen.
        unsafe { byond().ByondValue_IsStr(&self.0) }
    }

    pub fn is_list(&self) -> bool {
        // Safety: This operation only fails if our CByondValue is invalid, which cannot happen.
        unsafe { byond().ByondValue_IsList(&self.0) }
    }

    pub fn is_ptr(&self) -> bool {
        is_pointer_shim(self)
    }

    pub fn is_true(&self) -> bool {
        // Safety: This operation only fails if our CByondValue is invalid, which cannot happen.
        unsafe { byond().ByondValue_IsTrue(&self.0) }
    }
}
