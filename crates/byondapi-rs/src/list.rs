use std::{fmt::Debug, marker::PhantomData, mem::MaybeUninit, ptr::NonNull};

use byondapi_sys::{u4c, CByondValue, CByondValueList};

use crate::{prelude::ByondValue, static_global::BYOND, Error};

#[repr(transparent)]
pub struct ByondValueList(pub(crate) CByondValueList);

impl Default for ByondValueList {
    fn default() -> Self {
        let mut new_inner = MaybeUninit::uninit();

        let new_inner = unsafe {
            // Safety: new_inner is going to an initialization function, it will only write to the pointer.
            BYOND.ByondValueList_Init(new_inner.as_mut_ptr());
            // Safety: ByondValue_Init will have initialized the new_inner.
            new_inner.assume_init()
        };

        Self(new_inner)
    }
}

/// # Constructors
impl ByondValueList {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut new_inner = MaybeUninit::uninit();

        let new_inner = unsafe {
            // Safety: new_inner is going to an initialization function, it will only write to the pointer.
            BYOND.ByondValueList_InitCount(new_inner.as_mut_ptr(), capacity as u4c);
            // Safety: ByondValue_Init will have initialized the new_inner.
            new_inner.assume_init()
        };

        Self(new_inner)
    }
}

/// # Accessors
impl ByondValueList {
    pub fn push(&mut self, value: &ByondValue) -> Result<(), Error> {
        unsafe { map_byond_error!(BYOND.ByondValueList_Add(&mut self.0, &value.0)) }
    }

    pub fn insert(&mut self, index: usize, element: &ByondValue) -> Result<(), Error> {
        unsafe {
            map_byond_error!(BYOND.ByondValueList_InsertAt(&mut self.0, index as i32, &element.0))
        }
    }
}

// Debug!
impl Debug for ByondValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr = format! {"{:p}", self.0.items};
        let count = format!("0x{:X}", self.0.count);
        let capacity = format!("0x{:X}", self.0.capacity);

        f.debug_tuple("ByondValueList")
            .field(&ptr)
            .field(&count)
            .field(&capacity)
            .finish()
    }
}

impl TryFrom<ByondValue> for ByondValueList {
    type Error = Error;

    fn try_from(value: ByondValue) -> Result<Self, Self::Error> {
        let mut new_list = ByondValueList::new();

        unsafe { map_byond_error!(BYOND.Byond_ReadList(&value.0, &mut new_list.0))? }

        Ok(new_list)
    }
}

impl TryFrom<ByondValueList> for ByondValue {
    type Error = Error;

    fn try_from(value: ByondValueList) -> Result<Self, Self::Error> {
        let new_value = ByondValue::new_list().unwrap();

        unsafe {
            map_byond_error!(BYOND.Byond_WriteList(&new_value.0, &value.0))?;
        }

        Ok(new_value)
    }
}
