use std::fmt;
use std::io;
use std::result;

/// An enum that represents all types of errors that can occur when using PickleDB
#[derive(Debug)]
pub enum ErrorType {
    /// I/O error when reading or writing to file, for example: file not found, etc.
    Io,
    /// An error when trying to serialize or deserialize data
    Serialization,
}

/// A struct that represents all possible errors that can occur when using PickleDB
pub struct Error {
    err_code: ErrorCode,
}

/// Alias for a `Result` with the error type [Error](struct.Error.html).
pub type Result<T> = result::Result<T, Error>;

impl Error {
    pub(crate) fn new(err_code: ErrorCode) -> Error {
        Error { err_code }
    }

    /// Get the error type
    pub fn get_type(&self) -> ErrorType {
        match self.err_code {
            ErrorCode::Io(_) => ErrorType::Io,
            ErrorCode::Serialization(_) => ErrorType::Serialization,
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
        fmt.write_str(&format!(
            "Error {{ msg: {} }}",
            match self.err_code {
                ErrorCode::Io(ref err) => err.to_string(),
                ErrorCode::Serialization(ref err_str) => err_str.to_string(),
            }
        ))
    }
}

impl std::error::Error for Error {}

pub(crate) enum ErrorCode {
    Io(io::Error),
    Serialization(String),
}
