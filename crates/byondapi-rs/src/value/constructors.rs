use std::ffi::CString;

use byondapi_sys::{u4c, ByondValueType, CByondValue};

use super::ByondValue;
use crate::{static_global::byond, Error};

impl Default for ByondValue {
    fn default() -> Self {
        #[cfg(any(
            feature = "byond-515-1609",
            feature = "byond-515-1610",
            feature = "byond-515-1611",
            feature = "byond-515-1617"
        ))]
        {
            use std::mem::MaybeUninit;
            let mut new_inner = MaybeUninit::uninit();

            let new_inner = unsafe {
                // Safety: new_inner is going to an initialization function, it will only write to the pointer.
                byond().ByondValue_Init(new_inner.as_mut_ptr());
                // Safety: ByondValue_Init will have initialized the new_inner.
                new_inner.assume_init()
            };

            Self(new_inner)
        }
        #[cfg(any(feature = "byond-515-1620"))]
        {
            Self(CByondValue {
                type_: 0,
                junk1: 0,
                junk2: 0,
                junk3: 0,
                data: byondapi_sys::ByondValueData { ref_: 0 },
            })
        }
    }
}

/// # Constructors
impl ByondValue {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn null() -> Self {
        Default::default()
    }

    pub fn new_ref(typ: ByondValueType, ptr: u4c) -> Self {
        #[cfg(any(
            feature = "byond-515-1609",
            feature = "byond-515-1610",
            feature = "byond-515-1611",
            feature = "byond-515-1617"
        ))]
        {
            use std::mem::MaybeUninit;
            let mut new_inner = MaybeUninit::uninit();

            let new_inner = unsafe {
                // Safety: new_inner is going to an initialization function, it will only write to the pointer.
                byond().ByondValue_InitRef(new_inner.as_mut_ptr(), typ, ptr);
                // Safety: ByondValue_Init will have initialized the new_inner.
                new_inner.assume_init()
            };

            Self(new_inner)
        }
        #[cfg(any(feature = "byond-515-1620"))]
        {
            Self(CByondValue {
                type_: typ,
                junk1: 0,
                junk2: 0,
                junk3: 0,
                data: byondapi_sys::ByondValueData { ref_: ptr },
            })
        }
    }

    pub fn new_num(f: f32) -> Self {
        #[cfg(any(
            feature = "byond-515-1609",
            feature = "byond-515-1610",
            feature = "byond-515-1611",
            feature = "byond-515-1617"
        ))]
        {
            use std::mem::MaybeUninit;
            let mut new_inner = MaybeUninit::uninit();

            let new_inner = unsafe {
                // Safety: new_inner is going to an initialization function, it will only write to the pointer.
                byond().ByondValue_InitNum(new_inner.as_mut_ptr(), f);
                // Safety: ByondValue_Init will have initialized the new_inner.
                new_inner.assume_init()
            };

            Self(new_inner)
        }
        #[cfg(any(feature = "byond-515-1620"))]
        {
            Self(CByondValue {
                type_: 0x2A,
                junk1: 0,
                junk2: 0,
                junk3: 0,
                data: byondapi_sys::ByondValueData { num: f },
            })
        }
    }

    pub fn new_str<S: Into<Vec<u8>>>(s: S) -> Result<Self, Error> {
        let c_str = CString::new(s.into()).unwrap();

        #[cfg(any(
            feature = "byond-515-1609",
            feature = "byond-515-1610",
            feature = "byond-515-1611",
            feature = "byond-515-1617"
        ))]
        {
            use std::mem::MaybeUninit;
            let mut new_inner = MaybeUninit::uninit();

            let new_inner = unsafe {
                let result = map_byond_error!(
                    byond().ByondValue_InitStr(new_inner.as_mut_ptr(), c_str.as_ptr())
                );

                match result {
                    Ok(_) => {
                        // Safety: ByondValue_Init will have initialized the new_inner.
                        new_inner.assume_init()
                    }
                    Err(e) => return Err(e),
                }
            };

            Ok(Self(new_inner))
        }
        #[cfg(any(feature = "byond-515-1620"))]
        {
            let str_id = unsafe { byond().Byond_AddGetStrId(c_str.as_ptr()) };
            if str_id == u32::MAX {
                return Err(Error::UnableToCreateString);
            }
            Ok(Self(CByondValue {
                type_: 0x06,
                junk1: 0,
                junk2: 0,
                junk3: 0,
                data: byondapi_sys::ByondValueData { ref_: str_id },
            }))
        }
    }

    pub fn new_list() -> Result<Self, Error> {
        let mut new_self = Self::new();

        unsafe { map_byond_error!(byond().Byond_CreateList(&mut new_self.0))? }

        Ok(new_self)
    }
}

impl<'a> ByondValue {
    /// # Safety
    /// The [`CByondValue`] must be initialized.
    pub unsafe fn from_ref(s: &'a CByondValue) -> &'a Self {
        unsafe { std::mem::transmute(s) }
    }
}
