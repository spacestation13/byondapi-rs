use crate::{byond_string, static_global::byond, value::ByondValue, Error};
/// List stuff goes here, Keep in mind that all indexing method starts at zero instead of one like byondland
impl ByondValue {
    /// Gets an array of all the list values, this means values for assoc lists and just items in the listfor regular lists
    pub fn get_list_values(&self) -> Result<Vec<ByondValue>, Error> {
        use std::cell::RefCell;
        if !self.is_list() {
            return Err(Error::NotAList(*self));
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
                    buff.reserve_exact(len as usize);
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

    /// Gets an array of all the list elements, this means both keys and values for assoc lists and values for regular lists
    /// Reads items as key,value pairs from an associative list, storing them sequentially as key1, value1, key2, value2, etc.
    pub fn get_list(&self) -> Result<Vec<ByondValue>, Error> {
        use std::cell::RefCell;
        if !self.is_list() {
            return Err(Error::NotAList(*self));
        }

        thread_local! {
            static BUFFER: RefCell<Vec<ByondValue>> = RefCell::new(Vec::with_capacity(1));
        }

        BUFFER.with_borrow_mut(|buff| -> Result<Vec<ByondValue>, Error> {
            let mut len = buff.capacity() as u32;

            // Safety: buffer capacity is passed to byond, which makes sure it writes in-bound
            let initial_res =
                unsafe { byond().Byond_ReadListAssoc(&self.0, buff.as_mut_ptr().cast(), &mut len) };
            match (initial_res, len) {
                (false, 1..) => {
                    buff.reserve_exact(len as usize);
                    // Safety: buffer capacity is passed to byond, which makes sure it writes in-bound
                    unsafe {
                        map_byond_error!(byond().Byond_ReadListAssoc(
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

    /// Reads a value by key through the ref. Fails if this isn't a list.
    pub fn read_list_index<I: TryInto<ByondValue>>(&self, index: I) -> Result<ByondValue, Error> {
        if !self.is_list() {
            return Err(Error::NotAList(*self));
        }
        let index: ByondValue = index.try_into().map_err(|_| Error::InvalidConversion)?;
        self.read_list_index_internal(&index)
    }

    /// Writes a value by key through the ref. Fails if this isn't a list.
    pub fn write_list_index<I: TryInto<ByondValue>, V: TryInto<ByondValue>>(
        &mut self,
        index: I,
        value: V,
    ) -> Result<(), Error> {
        if !self.is_list() {
            return Err(Error::NotAList(*self));
        }
        let index: ByondValue = index.try_into().map_err(|_| Error::InvalidConversion)?;
        let value: ByondValue = value.try_into().map_err(|_| Error::InvalidConversion)?;
        self.write_list_index_internal(&index, &value)
    }

    /// Reads a value by key through the ref. Fails if the index doesn't exist
    pub fn read_list_index_internal(&self, index: &ByondValue) -> Result<ByondValue, Error> {
        let mut result = ByondValue::default();
        unsafe {
            map_byond_error!(byond().Byond_ReadListIndex(&self.0, &index.0, &mut result.0))?;
        }
        Ok(result)
    }

    /// Writes a value by key through the ref. Dunno why it can fail
    pub fn write_list_index_internal(
        &mut self,
        index: &ByondValue,
        value: &ByondValue,
    ) -> Result<(), Error> {
        unsafe {
            map_byond_error!(byond().Byond_WriteListIndex(&self.0, &index.0, &value.0))?;
        }
        Ok(())
    }

    /// Pushes a value into a list
    pub fn push_list(&mut self, value: ByondValue) -> Result<(), Error> {
        if !self.is_list() {
            return Err(Error::NotAList(*self));
        }
        self.call_id(byond_string!("Add"), &[value])?;
        Ok(())
    }

    /// Pops a value from a list
    pub fn pop_list(&mut self) -> Result<Option<ByondValue>, Error> {
        if !self.is_list() {
            return Err(Error::NotAList(*self));
        }
        let len = self.builtin_length()?.get_number()? as usize;
        if len == 0 {
            return Ok(None);
        }
        let value = self.read_list_index(len as f32)?;
        self.call_id(byond_string!("Remove"), &[value])?;
        Ok(Some(value))
    }
}
