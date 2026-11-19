use super::ByondValue;
use crate::{static_global::byond, Error};

#[repr(transparent)]
pub struct ByondValuePointer(pub ByondValue);

impl ByondValuePointer {
    /// If the value is actually a pointer, this will wrap it in a comfy type. Otherwise it fails.
    pub fn new(value: ByondValue) -> Result<Self, Error> {
        value.try_into()
    }

    /// Read from this pointer and get a new [`ByondValue`]
    pub fn read(&self) -> Result<ByondValue, Error> {
        unsafe {
            let mut new_value = std::mem::MaybeUninit::uninit();
            map_byond_error!(byond().Byond_ReadPointer(&self.0 .0, new_value.as_mut_ptr()))?;
            Ok(ByondValue(new_value.assume_init()))
        }
    }

    /// Write a [`ByondValue`] through this pointer
    pub fn write(&self, value: &ByondValue) -> Result<(), Error> {
        unsafe { map_byond_error!(byond().Byond_WritePointer(&self.0 .0, &value.0)) }
    }
}

impl TryFrom<ByondValue> for ByondValuePointer {
    type Error = Error;

    fn try_from(value: ByondValue) -> Result<Self, Self::Error> {
        if value.is_ptr() {
            Ok(Self(value))
        } else {
            Err(Error::NotAPtr(format!("{:?}", value)))
        }
    }
}
