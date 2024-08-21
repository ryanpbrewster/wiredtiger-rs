use std::{ffi::{c_int, CStr, NulError}, fmt::Display};

use wiredtiger_sys::wiredtiger_strerror;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Placeholder(String),
    Unknown { errno: c_int },
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for Error {}

impl From<NulError> for Error {
    fn from(value: NulError) -> Self {
        Self::Placeholder(value.to_string())
    }
}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Placeholder(value.to_string())
    }
}

impl Error {
    pub fn from_wt(errno: c_int) -> Self {
        let msg = unsafe {
            let raw = wiredtiger_strerror(errno);
            if raw.is_null() {
                return Error::Unknown { errno };
            }
            CStr::from_ptr(raw)
        };
        Error::Placeholder(
            msg.to_str()
                .expect("wiredtiger_strerror should return valid utf8")
                .to_owned(),
        )
    }
}
