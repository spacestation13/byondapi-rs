use std::{
    ffi::{CStr, CString},
    mem::MaybeUninit,
};

use byondapi_sys::{u4c, ByondValueType, CByondValue};

use crate::{error::Error, static_global::BYOND, typecheck_trait::ByondTypeCheck};

#[repr(transparent)]
pub struct ByondValue(CByondValue);

impl Default for ByondValue {
    fn default() -> Self {
        let mut new_inner = MaybeUninit::uninit();

        let new_inner = unsafe {
            // Safety: new_inner is going to an initialization function, it will only write to the pointer.
            BYOND.ByondValue_Init(new_inner.as_mut_ptr());
            // Safety: ByondValue_Init will have initialized the new_inner.
            new_inner.assume_init()
        };

        Self(new_inner)
    }
}

impl ByondValue {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_ref(typ: ByondValueType, ptr: u4c) -> Self {
        let mut new_inner = MaybeUninit::uninit();

        let new_inner = unsafe {
            // Safety: new_inner is going to an initialization function, it will only write to the pointer.
            BYOND.ByondValue_InitRef(new_inner.as_mut_ptr(), typ, ptr);
            // Safety: ByondValue_Init will have initialized the new_inner.
            new_inner.assume_init()
        };

        Self(new_inner)
    }

    pub fn new_num(f: f32) -> Self {
        let mut new_inner = MaybeUninit::uninit();

        let new_inner = unsafe {
            // Safety: new_inner is going to an initialization function, it will only write to the pointer.
            BYOND.ByondValue_InitNum(new_inner.as_mut_ptr(), f);
            // Safety: ByondValue_Init will have initialized the new_inner.
            new_inner.assume_init()
        };

        Self(new_inner)
    }

    pub fn new_str<S: AsRef<str>>(s: S) -> Result<Self, Error> {
        let c_str = CString::new(s.as_ref()).unwrap();

        let mut new_inner = MaybeUninit::uninit();

        let new_inner = unsafe {
            // Safety: new_inner is going to an initialization function, it will only write to the pointer.
            let success = BYOND.ByondValue_InitStr(new_inner.as_mut_ptr(), c_str.as_ptr());
            if success {
                // Safety: ByondValue_Init will have initialized the new_inner.
                new_inner.assume_init()
            } else {
                return Err(Error::get_last_byond_error());
            }
        };

        Ok(Self(new_inner))
    }

    /// Maybe don't use this, questionable
    pub fn into_inner(mut self) -> CByondValue {
        std::mem::replace(&mut self.0, unsafe { std::mem::zeroed() })
    }
}

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

// From Impls
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
impl TryFrom<ByondValue> for f32 {
    type Error = Error;

    fn try_from(value: ByondValue) -> Result<Self, Self::Error> {
        if value.is_num() {
            Ok(unsafe { BYOND.ByondValue_GetNum(&value.0) })
        } else {
            Err(Error::InvalidConversion)
        }
    }
}

impl TryFrom<ByondValue> for CString {
    type Error = Error;

    fn try_from(value: ByondValue) -> Result<Self, Self::Error> {
        if value.is_str() {
            let ptr = unsafe { BYOND.ByondValue_GetStr(&value.0) };
            let cstr = unsafe { CStr::from_ptr(ptr) };
            Ok(cstr.to_owned())
        } else {
            Err(Error::InvalidConversion)
        }
    }
}

impl TryFrom<ByondValue> for String {
    type Error = Error;

    fn try_from(value: ByondValue) -> Result<Self, Self::Error> {
        let cstring: CString = value.try_into()?;
        cstring.into_string().map_err(|_| Error::NonUtf8String)
    }
}

// Equality
impl PartialEq for ByondValue {
    fn eq(&self, other: &Self) -> bool {
        // Safety: This operation only fails if our CByondValue is invalid, which cannot happen.
        unsafe { BYOND.ByondValue_Equals(&self.0, &other.0) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init_and_drop_bare() {
        let meow = ByondValue::new();
        std::hint::black_box(&meow);
        std::mem::drop(meow);
    }

    #[test]
    fn init_and_drop_string() {
        let meow = ByondValue::new_str("Meow meow meow meorw lwemow");
        std::hint::black_box(&meow);
        std::mem::drop(meow);
    }

    #[test]
    fn conversions() {
        let null = ByondValue::new();
        assert!(null.is_null());
        assert!(!null.is_num());
        assert!(!null.is_str());
        assert!(!null.is_list());

        let number: ByondValue = (42.0).into();
        assert!(!number.is_null());
        assert!(number.is_num());
        assert!(!number.is_str());
        assert!(!number.is_list());

        let string: ByondValue = "meow".try_into().unwrap();
        assert!(!string.is_null());
        assert!(!string.is_num());
        assert!(string.is_str());
        assert!(!string.is_list());
    }

    #[test]
    fn equality() {
        let number: ByondValue = (42.0).into();
        let number2: ByondValue = (42.0).into();
        assert!(number.eq(&number2));

        let nan: ByondValue = (f32::NAN).into();
        let nan2: ByondValue = (f32::NAN).into();
        assert!(nan.ne(&nan2));

        let string: ByondValue = "meow".try_into().unwrap();
        let string2: ByondValue = "meow".try_into().unwrap();
        assert!(string.eq(&string2));

        let null: ByondValue = ByondValue::new();
        let null2: ByondValue = ByondValue::new();
        assert!(null.eq(&null2));
    }
}
