use super::ByondValue;
use crate::{static_global::BYOND, typecheck_trait::ByondTypeCheck, Error};

#[repr(transparent)]
pub struct ByondValuePointer(pub ByondValue);

impl ByondValuePointer {
    /// If the value is actually a pointer, this will wrap it in a comfy type. Otherwise it fails.
    pub fn new(value: ByondValue) -> Result<Self, Error> {
        value.try_into()
    }

    /// Read from this pointer and get a new [`ByondValue`]
    pub fn read(&self) -> Result<ByondValue, Error> {
        let mut new_value = ByondValue::new();

        unsafe {
            map_byond_error!(BYOND.Byond_ReadPointer(&self.0 .0, &mut new_value.0))?;
        }

        Ok(new_value)
    }

    /// Write a [`ByondValue`] through this pointer
    pub fn write(&self, value: &ByondValue) -> Result<(), Error> {
        unsafe { map_byond_error!(BYOND.Byond_WritePointer(&self.0 .0, &value.0)) }
    }
}

impl TryFrom<ByondValue> for ByondValuePointer {
    type Error = Error;

    fn try_from(value: ByondValue) -> Result<Self, Self::Error> {
        if value.is_ptr() {
            Ok(Self(value))
        } else {
            Err(Error::InvalidConversion)
        }
    }
}
