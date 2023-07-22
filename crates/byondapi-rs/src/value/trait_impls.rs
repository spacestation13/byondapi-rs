use super::*;

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
