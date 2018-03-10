use std::error;
use std::fmt;
use std::io;

use serde_json;
use notify;

#[derive(Debug)]
pub enum Error {
    ArgumentError,
    IOError(io::Error),
    JSONError(serde_json::Error),
    NotifyError(notify::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match self {
            &ArgumentError => "Please supply the project file (.yyp)",
            &IOError(ref error) => error.description(),
            &JSONError(ref error) => error.description(),
            &NotifyError(ref error) => error.description(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Error {
        Error::IOError(other)
    }
}

impl From<serde_json::Error> for Error {
    fn from(other: serde_json::Error) -> Error {
        Error::JSONError(other)
    }
}

impl From<notify::Error> for Error {
    fn from(other: notify::Error) -> Error {
        Error::NotifyError(other)
    }
}
