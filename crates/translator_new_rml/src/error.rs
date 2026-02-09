use std::fmt::Display;

use plan::error::PlanError;

use super::extractors::error::ParseError;
use super::translator::error::TranslationError;

pub type NewRMLTranslationResult<T> = Result<T, NewRMLTranslationError>;

#[derive(Debug)]
pub enum NewRMLTranslationError {
    ParseError(Box<ParseError>),
    TranslationError(Box<TranslationError>),
    IoError(std::io::Error),
}

impl From<PlanError> for NewRMLTranslationError {
    fn from(value: PlanError) -> Self {
        Self::TranslationError(Box::new(value.into()))
    }
}

impl From<std::io::Error> for NewRMLTranslationError {
    fn from(v: std::io::Error) -> Self {
        Self::IoError(v)
    }
}

impl From<ParseError> for NewRMLTranslationError {
    fn from(v: ParseError) -> Self {
        Self::ParseError(Box::new(v))
    }
}

impl From<TranslationError> for NewRMLTranslationError {
    fn from(value: TranslationError) -> Self {
        Self::TranslationError(Box::new(value))
    }
}

impl Display for NewRMLTranslationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "error occurred while translating a RML 2.0 spec mapping document"
        )
    }
}

impl std::error::Error for NewRMLTranslationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            NewRMLTranslationError::ParseError(parse_error) => {
                Some(parse_error)
            }
            NewRMLTranslationError::TranslationError(translation_error) => {
                Some(translation_error)
            }
            NewRMLTranslationError::IoError(error) => Some(error),
        }
    }
}
