use std::ffi::CString;

use super::ByondValue;
use crate::{static_global::byond, Error};

impl ByondValue {
    /// Corresponds to [`dm::length`](https://www.byond.com/docs/ref/#/proc/length)
    /// Try to get a length of a string in bytes, lists in number of assoc elements probably, will fail if it's neither a list or string
    pub fn builtin_length(&self) -> Result<ByondValue, Error> {
        unsafe {
            let mut result = std::mem::MaybeUninit::uninit();
            map_byond_error!(byond().Byond_Length(&self.0, result.as_mut_ptr()))?;
            Ok(ByondValue(result.assume_init()))
        }
    }

    /// Corresponds to [`dm::new`](https://www.byond.com/docs/ref/#/proc/new)
    /// Try to create a new byond object, equivalent to byond's new
    pub fn builtin_new(
        value_type: &ByondValue,
        arglist: &[ByondValue],
    ) -> Result<ByondValue, Error> {
        unsafe {
            let mut result = std::mem::MaybeUninit::uninit();
            map_byond_error!(byond().Byond_New(
                &value_type.0,
                arglist.as_ptr().cast(),
                arglist.len() as u32,
                result.as_mut_ptr()
            ))?;
            Ok(ByondValue(result.assume_init()))
        }
    }

    /// Corresponds to [`dm::new`](https://www.byond.com/docs/ref/#/proc/new)
    /// Try to create a new byond object, equivalent to byond's new, but takes a list as arguments instead
    pub fn builtin_newarglist(
        value_type: &ByondValue,
        arglist: &ByondValue,
    ) -> Result<ByondValue, Error> {
        unsafe {
            let mut result = std::mem::MaybeUninit::zeroed();
            map_byond_error!(byond().Byond_NewArglist(
                &value_type.0,
                &arglist.0,
                result.as_mut_ptr()
            ))?;
            Ok(ByondValue(result.assume_init()))
        }
    }

    /// Corresponds to [`operator ~=`](https://www.byond.com/docs/ref/#/operator/~=)
    /// Compares this value to another one with byond's ~= operator, not the same as ==
    pub fn builtin_equiv(&self, other: &ByondValue) -> bool {
        unsafe { byond().ByondValue_Equiv(&self.0, &other.0) }
    }

    /// Corresponds to [`istype(src, text2path(typestr))`](https://www.byond.com/docs/ref/#/proc/istype)
    /// Checks if this value is of this type
    pub fn builtin_istype<T: Into<Vec<u8>>>(&self, typ: T) -> bool {
        let c_string = CString::new(typ).unwrap();
        unsafe { byond().ByondValue_IsType(&self.0, c_string.as_ptr().cast()) }
    }
}

/// Corresponds to the first variation of [`dm::locate(Type) in Container`](https://www.byond.com/docs/ref/#/proc/locate)
/// Finds an object prototype or tag within the haystack, usually used for finding objects within a turf/area/etc
pub fn builtin_locatein(needle: &ByondValue, haystack: &ByondValue) -> Result<ByondValue, Error> {
    unsafe {
        let mut output = std::mem::MaybeUninit::uninit();
        map_byond_error!(byond().Byond_LocateIn(&needle.0, &haystack.0, output.as_mut_ptr()))?;
        Ok(ByondValue(output.assume_init()))
    }
}

/// Corresponds to the third and forth variation of [`dm::locate(Tag/TextRef)`](https://www.byond.com/docs/ref/#/proc/locate)
/// Finds an object prototype or tag within the world.
pub fn builtin_locateby(target: &ByondValue) -> Result<ByondValue, Error> {
    unsafe {
        let mut output = std::mem::MaybeUninit::uninit();
        map_byond_error!(byond().Byond_LocateIn(&target.0, std::ptr::null(), output.as_mut_ptr()))?;
        Ok(ByondValue(output.assume_init()))
    }
}
