use super::*;

/// # Compatibility with the C++ API
impl ByondValue {
    /// Maybe don't use this, questionable
    pub fn into_inner(mut self) -> CByondValue {
        std::mem::replace(&mut self.0, unsafe { std::mem::zeroed() })
    }

    pub fn get_number(&self) -> Result<f32, Error> {
        self.try_into()
    }

    pub fn get_cstring(&self) -> Result<CString, Error> {
        self.try_into()
    }

    pub fn get_string(&self) -> Result<String, Error> {
        self.try_into()
    }
}
