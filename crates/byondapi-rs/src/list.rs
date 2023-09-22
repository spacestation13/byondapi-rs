//! [Newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) pattern over [`CByondValueList`]
use std::{
    fmt::Debug,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};

use crate::{static_global::byond, value::ByondValue, Error};
use byondapi_sys::CByondValueList;

/// [Newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) pattern over [`CByondValueList`]
#[repr(transparent)]
pub struct ByondValueList(pub CByondValueList);

/// # Constructors
impl ByondValueList {
    pub fn new() -> Self {
        Default::default()
    }
}

/// # Helpers
impl ByondValueList {
    pub fn capacity(&self) -> usize {
        self.0.capacity as usize
    }

    pub fn len(&self) -> usize {
        self.0.count as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// # Accessors
impl ByondValueList {
    /// Add a copy of value to the end of the list
    pub fn push(&mut self, value: &ByondValue) -> Result<(), Error> {
        unsafe { map_byond_error!(byond().ByondValueList_Add(&mut self.0, &value.0)) }
    }

    /// Remove the last element from the list
    pub fn pop(&mut self) -> Result<ByondValue, Error> {
        self.remove((self.0.count as usize) - 1)
    }

    /// Add a copy of value at a specific index
    pub fn insert(&mut self, index: usize, element: &ByondValue) -> Result<(), Error> {
        unsafe {
            map_byond_error!(byond().ByondValueList_InsertAt(&mut self.0, index as i32, &element.0))
        }
    }

    /// Remove a value at a specific index
    pub fn remove(&mut self, index: usize) -> Result<ByondValue, Error> {
        let element = self[index].clone();

        let num_removed = unsafe { byond().ByondValueList_RemoveAt(&mut self.0, index as u32, 1) };
        if num_removed != 1 {
            Err(Error::get_last_byond_error())
        } else {
            Ok(element)
        }
    }
}

/// # Safety
/// See the constraints in [`std::slice::from_raw_parts`]
/// - `data` is valid for `len * mem::size_of::<ByondValue>()`
///     - The entire memory range is contained within a `malloc()` block
///     - zero length slices are just constructed normally
/// - `data` points to `len` consecutive properly initialized values of [`ByondValue`]
/// - The lifetime is based on the lifetime of the list
/// - The total size is never going to be larger than `isize::MAX`
impl Deref for ByondValueList {
    type Target = [ByondValue];
    fn deref(&self) -> &Self::Target {
        unsafe {
            let count = self.0.count;
            if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(self.0.items as *const ByondValue, self.0.count as usize)
            }
        }
    }
}

/// # Safety
/// See the constraints in [`std::slice::from_raw_parts_mut`]
/// - `data` is valid for `len * mem::size_of::<ByondValue>()`
///   - The entire memory range is contained within a `malloc()` block
///   - zero length slices are just constructed normally
/// - `data` points to `len` consecutive properly initialized values of [`ByondValue`]
/// - The lifetime is based on the lifetime of the list
/// - The total size is never going to be larger than `isize::MAX`
impl DerefMut for ByondValueList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let count = self.0.count;
            if count == 0 {
                &mut []
            } else {
                std::slice::from_raw_parts_mut(
                    self.0.items as *mut ByondValue,
                    self.0.count as usize,
                )
            }
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
        unsafe { byond().ByondValueList_Free(&mut self.0) }
    }
}

impl Default for ByondValueList {
    fn default() -> Self {
        let mut new_inner = MaybeUninit::uninit();

        let new_inner = unsafe {
            // Safety: new_inner is going to an initialization function, it will only write to the pointer.
            byond().ByondValueList_Init(new_inner.as_mut_ptr());
            // Safety: ByondValue_Init will have initialized the new_inner.
            new_inner.assume_init()
        };

        Self(new_inner)
    }
}

impl TryFrom<&ByondValue> for ByondValueList {
    type Error = Error;

    fn try_from(value: &ByondValue) -> Result<Self, Self::Error> {
        let mut new_list = ByondValueList::new();

        unsafe { map_byond_error!(byond().Byond_ReadList(&value.0, &mut new_list.0))? }

        Ok(new_list)
    }
}

impl TryFrom<&ByondValueList> for ByondValue {
    type Error = Error;

    fn try_from(value: &ByondValueList) -> Result<Self, Self::Error> {
        // The API must be called in this order:
        // ByondValue_Init(&value) // Initializes the value
        // Byond_CreateList(&value) // Creates a list() inside DM
        // Byond_WriteList(&value, &list) // Copies the CByondList into the dm list()
        let new_value = ByondValue::new_list().unwrap();

        unsafe {
            map_byond_error!(byond().Byond_WriteList(&new_value.0, &value.0))?;
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

impl<'a> From<&'a ByondValueList> for &'a [ByondValue] {
    fn from(value: &'a ByondValueList) -> Self {
        value
    }
}

impl<'a> From<&'a mut ByondValueList> for &'a mut [ByondValue] {
    fn from(value: &'a mut ByondValueList) -> Self {
        value
    }
}

/// Clones the list into a vec
impl From<ByondValueList> for Vec<ByondValue> {
    fn from(value: ByondValueList) -> Self {
        value.iter().cloned().collect()
    }
}

/// Clones the list into a vec
impl From<&ByondValueList> for Vec<ByondValue> {
    fn from(value: &ByondValueList) -> Self {
        value.iter().cloned().collect()
    }
}
