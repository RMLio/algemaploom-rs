use std::convert::Infallible;
use std::error::Error;
use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub enum ParseError {
    IOErrorStr(String),
    IOError(io::Error),
    SerdeError(serde_json::Error),
    GenericError(String),
    NoTermMapFoundError(String),
    ExtensionError(String),
    Infallible,
}

impl From<serde_json::Error> for ParseError {
    fn from(value: serde_json::Error) -> Self {
        ParseError::SerdeError(value)
    }
}

impl From<Infallible> for ParseError {
    fn from(_: Infallible) -> Self {
        ParseError::Infallible
    }
}

impl From<io::Error> for ParseError {
    fn from(value: io::Error) -> Self {
        ParseError::IOError(value)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::IOErrorStr(msg) => {
                write!(f, "IO error occurred while parsing with msg: {}", msg)
            }
            ParseError::IOError(error) => {
                write!(f, "IO error occurred while parsing")
            }
            ParseError::SerdeError(error) => {
                write!(f, "JSON serde error occurred while parsing")
            }
            ParseError::GenericError(msg) => {
                write!(f, "generic error while parsing with msg: \n {}", msg)
            }
            ParseError::NoTermMapFoundError(msg) => {
                write!(f, "no term map found error with msg: \n {}", msg)
            }
            ParseError::ExtensionError(msg) => {
                write!(f, "file extension error with msg: \n {}", msg)
            }
            ParseError::Infallible => panic!(),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseError::IOError(error) => error.source(),
            ParseError::SerdeError(error) => error.source(),
            _ => None,
        }
    }
}
