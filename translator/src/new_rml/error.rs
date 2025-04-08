use std::fmt::Display;

use super::extractors::error::ParseError;
use super::translator::error::TranslationError;

pub type NewRMLTranslationResult<T> = Result<T, NewRMLTranslationError>;

#[derive(Debug)]
pub enum NewRMLTranslationError {
    ParseError(ParseError),
    TranslationError(TranslationError),
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

impl From<TranslationError> for NewRMLTranslationError {
    fn from(value: TranslationError) -> Self {
        Self::TranslationError(value)
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
