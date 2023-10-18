//! Error types for any problems this runs into, including internal BYOND errors.
use std::ffi::{CStr, CString};

use crate::static_global::byond;

#[derive(Debug)]
pub enum Error {
    /// This error is thrown when you try to convert a [`crate::ByondValue`] into a type which it does not represent, or the value failed to convert to a [`crate::ByondValue`].
    InvalidConversion,
    /// This error is thrown from call when you try to call something that isn't in BYOND's string tree (thus is not a valid proc)
    InvalidProc,
    /// Thrown when trying to get a [`String`] from a [`crate::ByondValue`].
    NonUtf8String,
    /// Internal BYOND API error
    ByondError(ByondError),
    /// When the BYOND API doesn't tell us what the error is
    UnknownByondError,
    /// Thrown by us when we know this call will panic internally because of the version
    NotAvailableForThisByondVersion,
    /// Thrown by us when we know this type does not have a ref
    NotReferencable,
    /// Thrown by us when we know this type is not a list, and we're expecting one
    NotAList,
    /// Thrown by us when we know this type is not a string, and we're expecting one
    NotAString,
    /// Thrown by us when we know this type is not a number, and we're expecting one
    NotANum,
    /// Thrown by us when we know this type is not a pointer, and we're expecting one
    NotAPtr,
    /// Thrown by [`crate::byond_string::str_id_of_cstr`] when the string doesn't exist in
    /// byondland
    NonExistentString,
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

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidConversion => write!(f, "Cannot convert value to target type"),
            Self::InvalidProc => write!(f, "Cannot call proc, proc doesn't exist"),
            Self::NonUtf8String => write!(f, "String is not utf8"),
            Self::ByondError(e) => write!(f, "Byondapi error: {:#?}", e.0),
            Self::UnknownByondError => write!(f, "Unknown byondapi error"),
            Self::NotAvailableForThisByondVersion => write!(
                f,
                "This call is not available on current version of the api"
            ),
            Self::NotReferencable => write!(f, "Cannot get a ref from this value"),
            Self::NotAList => write!(f, "Value is not a list"),
            Self::NotAString => write!(f, "Value is not a string"),
            Self::NotANum => write!(f, "Value is not a number"),
            Self::NotAPtr => write!(f, "Value is not a pointer"),
            Self::NonExistentString => write!(f, "String id not found"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub struct ByondError(pub CString);

impl ByondError {
    pub fn get_last() -> Option<Self> {
        // Safety: It's always safe to call Byond_LastError
        let ptr = unsafe { byond().Byond_LastError() };
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
