// use crate::{value::ByondValue, BYOND};
// use std::{cell::UnsafeCell, mem::MaybeUninit};

// use byondapi_sys::CByondValueList;

// pub struct ByondValueList {
//     _internal: UnsafeCell<CByondValueList>,
// }

// impl Default for ByondValueList {
//     fn default() -> Self {
//         let m = MaybeUninit::<UnsafeCell<CByondValueList>>::uninit();
//         unsafe {
//             let raw_ptr = UnsafeCell::raw_get(m.as_ptr());
//             BYOND.ByondValueList_Init(raw_ptr);
//         }

//         let uc = unsafe { m.assume_init() };
//         ByondValueList { _internal: uc }
//     }
// }

// impl ByondValueList {
//     pub fn new() -> Self {
//         Default::default()
//     }

//     pub fn push(&mut self, value: ByondValue) -> bool {
//         let raw = value.get_const();
//         unsafe { BYOND.ByondValueList_Add(self._internal.get(), raw) }
//     }

//     pub fn len(&self) -> usize {
//         unsafe { (*self.get_const()).count as usize }
//     }

//     pub fn capacity(&self) -> usize {
//         unsafe { (*self.get_const()).capacity as usize }
//     }

//     pub fn is_empty(&self) -> bool {
//         self.len() == 0
//     }

//     pub fn pop(&mut self) -> ByondValue {
//         let raw = self._internal.get_mut();
//         let ptr = unsafe { raw.items.offset((raw.count - 1) as isize) };

//         unsafe {
//             assert_eq!(
//                 BYOND.ByondValueList_RemoveAt(raw as *mut _, raw.count - 1, 1),
//                 1
//             );
//         }

//         unsafe { ByondValue::from_raw(ptr) }
//     }

//     #[allow(dead_code)]
//     pub(crate) fn get_const(&self) -> *const CByondValueList {
//         self._internal.get() as *const CByondValueList
//     }

//     #[allow(dead_code)]
//     pub(crate) fn get_mut(&mut self) -> *mut CByondValueList {
//         self._internal.get()
//     }
// }

// impl Drop for ByondValueList {
//     fn drop(&mut self) {
//         unsafe { BYOND.ByondValueList_Free(self._internal.get()) }
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn init_and_drop() {
//         let meow = ByondValueList::new();
//         std::hint::black_box(&meow);
//         std::mem::drop(meow);
//     }

//     #[test]
//     fn push() {
//         let mut list = ByondValueList::new();
//         let meow = ByondValue::new();
//         assert_eq!(list.len(), 0);
//         assert!(list.push(meow));
//         assert_eq!(list.len(), 1);
//     }

//     #[test]
//     fn push_and_pop() {
//         let mut list = ByondValueList::new();
//         let meow = ByondValue::new();
//         // The list should be initially completely empty
//         assert_eq!(list.len(), 0);
//         assert_eq!(list.capacity(), 0);

//         // We push the value into it
//         assert!(list.push(meow));

//         // The list should now have a len of 1 and a capacity other than 0
//         assert_eq!(list.len(), 1);
//         let new_capacity = list.capacity();
//         assert_ne!(new_capacity, 0);

//         // Remove the value
//         let _ = list.pop();

//         // The len should be zero, but the capacity should remain
//         assert_eq!(list.len(), 0);
//         assert_eq!(list.capacity(), new_capacity);
//     }

//     #[test]
//     fn big_pushpop() {
//         let mut list = ByondValueList::new();
//         // The list should be initially completely empty
//         assert_eq!(list.len(), 0);
//         assert_eq!(list.capacity(), 0);

//         // We push the values into it
//         for _ in 0..100 {
//             assert!(list.push(ByondValue::new()));
//         }

//         // The list should now have a len of 100 and a capacity other than 0
//         assert_eq!(list.len(), 100);
//         let new_capacity = list.capacity();
//         assert_ne!(new_capacity, 0);

//         // Remove the values
//         for _ in 0..100 {
//             let _ = list.pop();
//         }

//         // The len should be zero, but the capacity should remain
//         assert_eq!(list.len(), 0);
//         assert_eq!(list.capacity(), new_capacity);
//     }
// }
