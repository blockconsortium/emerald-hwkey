use std::{io, fmt, error};

#[derive(Debug, PartialEq, Clone)]
pub enum HWKeyError {
    /// An unsupported cipher
    CryptoError(String),
    /// Error from HID communication
    CommError(String),
    /// Communication encoding error
    EncodingError(String),
    /// Something else
    OtherError(String),
}

impl From<io::Error> for HWKeyError {
    fn from(err: io::Error) -> Self {
        HWKeyError::CommError(err.to_string())
    }
}

impl<'a> From<&'a str> for HWKeyError {
    fn from(err: &str) -> Self {
        HWKeyError::OtherError(err.to_string())
    }
}

impl fmt::Display for HWKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HWKeyError::CryptoError(ref str) => write!(f, "HWKey error: {}", str),
            HWKeyError::CommError(ref str) => write!(f, "Communication protocol error: {}", str),
            HWKeyError::EncodingError(ref str) => write!(f, "Encoding error: {}", str),
            HWKeyError::OtherError(ref str) => write!(f, "HWKey other error: {}", str),
        }
    }
}

impl error::Error for HWKeyError {
    fn description(&self) -> &str {
        "HWKey error"
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            _ => None,
        }
    }
}