use std::result;
use std::io;
use std::fmt;

/// TODO: fix doc
#[derive(Debug)]
pub enum ErrorType {
    Io,
    Serialization
}

/// TODO: fix doc
pub struct Error {
    err_code: ErrorCode
}

/// TODO: fix doc
pub type Result<T> = result::Result<T, Error>;

impl Error {

    pub(crate) fn new(err_code: ErrorCode) -> Error {
        Error { err_code: err_code }
    }

    pub fn get_type(&self) -> ErrorType {
        match self.err_code {
            ErrorCode::Io(_) => ErrorType::Io,
            ErrorCode::Serialization(_) => ErrorType::Serialization
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.err_code {
            ErrorCode::Io(ref err) => fmt::Display::fmt(err, f),
            ErrorCode::Serialization(ref err_str) => f.write_str(err_str),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&format!("Error {{ msg: {} }}",
            match self.err_code {
                ErrorCode::Io(ref err) => err.to_string(),
                ErrorCode::Serialization(ref err_str) => err_str.to_string()
            }
        ))
    }
}

pub(crate) enum ErrorCode {
    Io(io::Error),
    Serialization(String)
}