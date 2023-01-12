use std::fmt::{self, Display};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Message(String),
    LogicError(String),
    EofError,
    IoError(String),
    SerializeError(String),
    DeserializeError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Message(msg) => f.write_str(msg),
            Error::EofError => f.write_str("Unexpected EOF"),
            Error::IoError(msg) => f.write_str(&format!("IO Error: {}", msg)),
            Error::LogicError(msg) => f.write_str(&format!("Logic Error: {}", msg)),
            Error::SerializeError(msg) => f.write_str(&format!("Serialize Error: {}", msg)),
            Error::DeserializeError(msg) => f.write_str(&format!("Deserialize Error: {}", msg)),
        }
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::SerializeError(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::DeserializeError(msg.to_string())
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(err: std::array::TryFromSliceError) -> Self {
        Self::Message(err.to_string())
    }
}
impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::Message(err.to_string())
    }
}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.to_string())
    }
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Feb 03 2022, 12:10 [CST]
