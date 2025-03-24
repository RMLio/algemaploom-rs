use std::convert::Infallible;
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
