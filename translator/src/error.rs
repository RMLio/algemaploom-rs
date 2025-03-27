use std::path::PathBuf;

use plangenerator::error::PlanError;

use crate::new_rml::extractors::error::ParseError as NewRMLParseError;
use crate::rml::error::RMLTranslationError;
use crate::rml::parser::extractors::error::ParseError as RMLParseError;

#[derive(Debug)]
pub struct TranslationError {
    pub kind: TranslationErrorKind,
}

impl From<RMLTranslationError> for TranslationError {
    fn from(value: RMLTranslationError) -> Self {
        Self {
            kind: TranslationErrorKind::LanguageError(value.into()),
        }
    }
}


#[derive(Debug)]
pub enum TranslationErrorKind {
    LanguageError(LanguageErrorKind),
    FileLanguageError {
        file:  PathBuf,
        error: LanguageErrorKind,
    },
    FileError {
        file:  PathBuf,
        error: std::io::Error,
    },
    IoError(std::io::Error),
}

impl From<LanguageErrorKind> for TranslationErrorKind {
    fn from(v: LanguageErrorKind) -> Self {
        Self::LanguageError(v)
    }
}

#[derive(Debug)]
pub enum LanguageErrorKind {
    RMLTranslationError(RMLTranslationError),
}

impl From<RMLTranslationError> for LanguageErrorKind {
    fn from(v: RMLTranslationError) -> Self {
        Self::RMLTranslationError(v)
    }
}
