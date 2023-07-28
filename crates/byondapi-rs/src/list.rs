use std::{fmt::Debug, mem::MaybeUninit};

use byondapi_sys::CByondValueList;

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
}

/// # Accessors
impl ByondValueList {
    /// Add a copy of value to the end of the list
    pub fn push(&mut self, value: &ByondValue) -> Result<(), Error> {
        unsafe { map_byond_error!(BYOND.ByondValueList_Add(&mut self.0, &value.0)) }
    }

    /// Add a copy of value at a specific index
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

impl Drop for ByondValueList {
    fn drop(&mut self) {
        // Safety: We are being dropped, it is okay to free our inner CByondValue.
        unsafe { BYOND.ByondValueList_Free(&mut self.0) }
    }
}

impl TryFrom<&ByondValue> for ByondValueList {
    type Error = Error;

    fn try_from(value: &ByondValue) -> Result<Self, Self::Error> {
        let mut new_list = ByondValueList::new();

        unsafe { map_byond_error!(BYOND.Byond_ReadList(&value.0, &mut new_list.0))? }

        Ok(new_list)
    }
}

impl TryFrom<&ByondValueList> for ByondValue {
    type Error = Error;

    fn try_from(value: &ByondValueList) -> Result<Self, Self::Error> {
        let new_value = ByondValue::new_list().unwrap();

        unsafe {
            map_byond_error!(BYOND.Byond_WriteList(&new_value.0, &value.0))?;
        }

        Ok(new_value)
    }
}

impl TryFrom<ByondValue> for ByondValueList {
    type Error = Error;

    fn try_from(value: ByondValue) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}

impl TryFrom<ByondValueList> for ByondValue {
    type Error = Error;

    fn try_from(value: ByondValueList) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}

#[derive(Debug)]
pub struct Iter<'a> {
    num: u32,
    list: &'a ByondValueList,
}

impl<'a> Iterator for Iter<'a> {
    type Item = ByondValue;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.num < self.list.0.count {
            let ptr = unsafe { &*self.list.0.items.add(self.num as usize) };
            Some(ptr.try_into().unwrap())
        } else {
            None
        };

        self.num += 1;

        ret
    }
}

impl<'a> ByondValueList {
    /// Get an iterator for this list
    pub fn iter(&'a self) -> Iter<'a> {
        Iter { num: 0, list: self }
    }
}

impl TryFrom<&[ByondValue]> for ByondValueList {
    type Error = Error;

    fn try_from(value: &[ByondValue]) -> Result<Self, Self::Error> {
        let mut list = Self::new();

        for x in value {
            list.push(x)?;
        }

        Ok(list)
    }
}
