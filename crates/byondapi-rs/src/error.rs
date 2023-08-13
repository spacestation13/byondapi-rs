//! Error types for any problems this runs into, including internal BYOND errors.
use std::ffi::{CStr, CString};

use crate::static_global::BYOND;

#[derive(Debug)]
pub enum Error {
    /// This error is thrown when you try to convert a ByondValue into a type which it does not represent.
    InvalidConversion,
    /// This error is thrown from call when you try to call something that isn't in BYOND's string tree (thus is not a valid proc)
    InvalidProc,
    /// Thrown when trying to get a String from a ByondValue.
    NonUtf8String,
    /// Internal BYOND API error
    ByondError(ByondError),
    /// When the BYOND API doesn't tell us what the error is
    UnknownByondError,
    /// Thrown by us when we know this call will panic internally because of the version
    NotAvailableForThisByondVersion,
}

impl Error {
    pub fn get_last_byond_error() -> Self {
        if let Some(err) = ByondError::get_last() {
            Self::ByondError(err)
        } else {
            Self::UnknownByondError
        }
    }
}

#[derive(Debug)]
pub struct ByondError(CString);

impl ByondError {
    pub fn get_last() -> Option<Self> {
        // Safety: It's always safe to call Byond_LastError
        let ptr = unsafe { BYOND.Byond_LastError() };
        if !ptr.is_null() {
            // Safety: We just have to trust that Byond gave us a valid cstring...
            let cstr = unsafe { CStr::from_ptr(ptr) };
            Some(ByondError(cstr.to_owned()))
        } else {
            None
        }
    }
}

macro_rules! map_byond_error {
    ($x:expr) => {{
        let result = $x;
        if result {
            Ok(())
        } else {
            Err(Error::get_last_byond_error())
        }
    }};
}

/// For extreme cases where we know we're about to crash, we write to a log.txt file in PWD so the user has some idea
/// what went wrong.
pub mod crash_logging {
    pub fn log_to_file<S: AsRef<str>>(log: S) {
        // Just drop the error, if we can't write the log then :shrug:
        let _ = std::fs::write("./byondapi-rs-log.txt", log.as_ref());
    }
}
