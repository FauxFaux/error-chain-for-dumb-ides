//! Totally stubbed out methods.
//!
//! Actually executing anything will panic as soon as possible.

use std::error;
use std::fmt;
use std::result;

pub enum ErrorKind {}

pub trait ResultExt<T> {
    fn chain_err<F, EK>(self, callback: F) -> Result<T>;
}

pub type Result<T> = result::Result<T, Error>;

pub struct Error;

impl fmt::Debug for Error {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        unimplemented!()
    }
}

impl<T> ResultExt<T> for Result<T> {
    fn chain_err<F, EK>(self, _callback: F) -> Result<T> {
        unimplemented!()
    }
}
