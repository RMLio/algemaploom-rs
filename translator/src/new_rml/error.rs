use std::fmt::Display;

use super::extractors::error::ParseError;

#[derive(Debug)]
pub enum NewRMLTranslationError {
    ParseError(ParseError),
    IoError(std::io::Error), 
}

impl From<std::io::Error> for NewRMLTranslationError {
    fn from(v: std::io::Error) -> Self {
        Self::IoError(v)
    }
}

impl From<ParseError> for NewRMLTranslationError {
    fn from(v: ParseError) -> Self {
        Self::ParseError(v)
    }
}

impl Display for NewRMLTranslationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for NewRMLTranslationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        todo!()
    }
}
