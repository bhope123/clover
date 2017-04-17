use std::error::Error as StdError;
use std::fmt;

/// Any error from this crate.
#[derive(Debug)]
pub enum Error {
    // An error from the `reqwest` crate.
    Http(::reqwest::Error),
    // An error from the `serde` crate for deserializing json.
    Json(::serde_json::Error),
    // A `std::io` error.
    Read(::std::io::Error),
    // An error from the `regex` crate. Failed to build a regex.
    Regex(::regex::Error),
    // An error from the `time` crate that `chrono` uses.
    // Signifies a bad conversion between `chrono::Duration` and
    // `std::time::Duration`.
    Time(::time::OutOfRangeError),
    // Tried to create a board that doesn't exist.
    InvalidBoardName,
    // Unexpected HTTP response received.
    UnexpectedResponse
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Http(ref e) => fmt::Display::fmt(e, f),
            Error::Json(ref e) => fmt::Display::fmt(e, f),
            Error::Read(ref e) => fmt::Display::fmt(e, f),
            Error::Regex(ref e) => fmt::Display::fmt(e, f),
            Error::Time(ref e) => fmt::Display::fmt(e, f),
            Error::InvalidBoardName => f.pad("Invalid board name"),
            Error::UnexpectedResponse => f.pad("Unexpected HTTP response received")
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Http(ref e) => e.description(),
            Error::Json(ref e) => e.description(),
            Error::Read(ref e) => e.description(),
            Error::Regex(ref e) => e.description(),
            Error::Time(ref e) => e.description(),
            Error::InvalidBoardName => "Invalid board name",
            Error::UnexpectedResponse => "Unexpected HTTP response received"
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Http(ref e) => Some(e),
            Error::Json(ref e) => Some(e),
            Error::Read(ref e) => Some(e),
            Error::Regex(ref e) => Some(e),
            Error::Time(ref e) => Some(e),
            Error::InvalidBoardName => None,
            Error::UnexpectedResponse => None
        }
    }
}

impl From<::reqwest::Error> for Error {
    fn from(err: ::reqwest::Error) -> Error {
        Error::Http(err)
    }
}

impl From<::serde_json::Error> for Error {
    fn from(err: ::serde_json::Error) -> Error {
        Error::Json(err)
    }
}

impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Error {
        Error::Read(err)
    }
}

impl From<::regex::Error> for Error {
    fn from(err: ::regex::Error) -> Error {
        Error::Regex(err)
    }
}

impl From<::time::OutOfRangeError> for Error {
    fn from(err: ::time::OutOfRangeError) -> Error {
        Error::Time(err)
    }
}

/// A `Result` alias where the `Err` case is `clover::Error`
pub type Result<T> = ::std::result::Result<T, Error>;
