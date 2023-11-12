use crate::{static_global::byond, typecheck_trait::ByondTypeCheck, value::ByondValue, Error};
use std::cell::RefCell;

/// # Helpers
impl ByondValue {
    /// Calls byond's builtin length proc, may have different meaning for different types
    pub fn len(&self) -> Result<usize, Error> {
        let mut new_value = ByondValue::new();

        unsafe {
            map_byond_error!(byond().Byond_Length(&self.0, &mut new_value.0))?;
        }

        Ok(new_value.get_number()? as usize)
    }

    pub fn is_empty(&self) -> Result<bool, Error> {
        Ok(self.len()? == 0)
    }

    /// Gets an array of all the list elements, this includes both keys and values for assoc lists, in an arbitrary order
    pub fn read_list(&self) -> Result<Vec<ByondValue>, Error> {
        if !self.is_list() {
            return Err(Error::NotAList);
        }

        thread_local! {
            static BUFFER: RefCell<Vec<ByondValue>> = RefCell::new(Vec::with_capacity(1));
        }

        BUFFER.with_borrow_mut(|buff| -> Result<Vec<ByondValue>, Error> {
            let mut len = buff.capacity() as u32;

            // Safety: buffer capacity is passed to byond, which makes sure it writes in-bound
            let initial_res =
                unsafe { byond().Byond_ReadList(&self.0, buff.as_mut_ptr().cast(), &mut len) };
            match (initial_res, len) {
                (false, 1..) => {
                    buff.reserve_exact(len as usize - buff.capacity());
                    // Safety: buffer capacity is passed to byond, which makes sure it writes in-bound
                    unsafe {
                        map_byond_error!(byond().Byond_ReadList(
                            &self.0,
                            buff.as_mut_ptr().cast(),
                            &mut len
                        ))?
                    };
                    // Safety: buffer should be written to at this point
                    unsafe { buff.set_len(len as usize) };
                    Ok(std::mem::take(buff))
                }
                (true, _) => {
                    // Safety: buffer should be written to at this point
                    unsafe { buff.set_len(len as usize) };
                    Ok(std::mem::take(buff))
                }
                (false, 0) => Err(Error::get_last_byond_error()),
            }
        })
    }
    /// Writes an array to the list
    pub fn write_list(&self, list: &[ByondValue]) -> Result<(), Error> {
        unsafe {
            map_byond_error!(byond().Byond_WriteList(
                &self.0,
                list.as_ptr().cast(),
                list.len() as u32
            ))
        }
    }
}
