use std::error;
use std::fmt;
use std::io;
use std::path;
use std::num;

mod parse_error;

use serde_json;
use notify;

pub use self::parse_error::ParseError;

#[derive(Debug)]
pub enum Error {
    ArgumentError,
    NoProject,
    InvalidCharacter,
    UnexpectedCharacter,
    MalformedNumericLiteral,
    CommentNestingDepth,
    InvalidPreprocessorDirective,
    UnexpectedEOF,
    ParseError(ParseError),
    MissingResource(String),
    IOError(io::Error),
    JSONError(serde_json::Error),
    NotifyError(notify::Error),
    ParseFloatError(num::ParseFloatError),
    ParseIntError(num::ParseIntError),
    CharsError(io::CharsError),
}

impl Error {
    pub fn missing_resource(path: &path::Path) -> Self {
        Error::MissingResource(format!("The resource is missing at path {:?}", path))
    }
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
            &NoProject => "The project file does not exist at the supplied path",
            &InvalidCharacter => "Invalid character in source file",
            &UnexpectedCharacter => "Unexpected character in source file",
            &MalformedNumericLiteral => "Malformed numeric literal",
            &CommentNestingDepth => "Comment nesting is too deep",
            &InvalidPreprocessorDirective => "Invalid preprocessor directive",
            &UnexpectedEOF => "Unexpected end of input",
            &ParseError(ref error) => error.description(),
            &ParseFloatError(ref error) => error.description(),
            &ParseIntError(ref error) => error.description(),
            &MissingResource(ref message) => &message,
            &IOError(ref error) => error.description(),
            &JSONError(ref error) => error.description(),
            &NotifyError(ref error) => error.description(),
            &CharsError(ref error) => error.description(),
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

impl From<num::ParseFloatError> for Error {
    fn from(other: num::ParseFloatError) -> Error {
        Error::ParseFloatError(other)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(other: num::ParseIntError) -> Error {
        Error::ParseIntError(other)
    }
}

impl From<io::CharsError> for Error {
    fn from(other: io::CharsError) -> Error {
        Error::CharsError(other)
    }
}
