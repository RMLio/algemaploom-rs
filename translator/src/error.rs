use std::fmt::Display;
use std::path::PathBuf;

use crate::new_rml::error::NewRMLTranslationError;
use crate::rml::error::RMLTranslationError;
use crate::shexml::error::ShExMLTranslationError;

#[derive(Debug)]
pub struct TranslationError {
    pub kind: TranslationErrorKind,
}

impl From<NewRMLTranslationError> for TranslationError {
    fn from(value: NewRMLTranslationError) -> Self {
        Self {
            kind: TranslationErrorKind::LanguageError(value.into()),
        }
    }
}
impl From<ShExMLTranslationError> for TranslationError {
    fn from(value: ShExMLTranslationError) -> Self {
        Self {
            kind: TranslationErrorKind::LanguageError(value.into()),
        }
    }
}

impl From<RMLTranslationError> for TranslationError {
    fn from(value: RMLTranslationError) -> Self {
        Self {
            kind: TranslationErrorKind::LanguageError(value.into()),
        }
    }
}

impl Display for TranslationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "error while translating a mapping document to algebraic plan"
        )
    }
}

impl std::error::Error for TranslationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.kind)
    }
}

#[derive(Debug)]
pub enum TranslationErrorKind {
    LanguageError(LanguageErrorKind),
    FileLanguageError {
        file:          PathBuf,
        lang_err_kind: LanguageErrorKind,
    },
    FileMsgError {
        file: PathBuf,
        msg:  String,
    },
    FileStdError {
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

impl Display for TranslationErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranslationErrorKind::LanguageError(_language_error_kind) => {
                write!(
                    f,
                    "error with a particular mapping language translation"
                )
            }
            TranslationErrorKind::FileLanguageError {
                file,
                lang_err_kind: _,
            } => {
                write!(
                    f,
                    "error while translating the file {}",
                    file.to_string_lossy()
                )
            }
            TranslationErrorKind::FileMsgError { file, msg } => {
                write!(
                    f,
                    "error while translating file {} with msg: {}",
                    file.to_string_lossy(),
                    msg
                )
            }
            TranslationErrorKind::FileStdError { file, error: _ } => {
                write!(
                    f,
                    "error while translating file {} with std error",
                    file.to_string_lossy()
                )
            }
            TranslationErrorKind::IoError(_error) => {
                write!(f, "IO error occurred")
            }
        }
    }
}

impl std::error::Error for TranslationErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TranslationErrorKind::LanguageError(language_error_kind) => {
                language_error_kind.source()
            }
            TranslationErrorKind::FileLanguageError {
                file: _,
                lang_err_kind,
            } => lang_err_kind.source(),
            TranslationErrorKind::FileStdError { file: _, error } => {
                error.source()
            }
            TranslationErrorKind::IoError(error) => error.source(),
            TranslationErrorKind::FileMsgError { file, msg } => None,
        }
    }
}

#[derive(Debug)]
pub enum LanguageErrorKind {
    RMLTranslationError(Box<RMLTranslationError>),
    NewRMLTranslationError(Box<NewRMLTranslationError>),
    ShExMLTranslationError(Box<ShExMLTranslationError>),
}

impl From<NewRMLTranslationError> for LanguageErrorKind {
    fn from(v: NewRMLTranslationError) -> Self {
        Self::NewRMLTranslationError(Box::new(v))
    }
}

impl From<ShExMLTranslationError> for LanguageErrorKind {
    fn from(v: ShExMLTranslationError) -> Self {
        Self::ShExMLTranslationError(Box::new(v))
    }
}

impl From<RMLTranslationError> for LanguageErrorKind {
    fn from(v: RMLTranslationError) -> Self {
        Self::RMLTranslationError(Box::new(v))
    }
}

impl Display for LanguageErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageErrorKind::RMLTranslationError(_rmltranslation_error) => {
                write!(f, "error while translating a RML v1.1.2 document")
            }
            LanguageErrorKind::ShExMLTranslationError(
                _shexml_translation_error,
            ) => write!(f, "error while translating a ShExML document"),
            LanguageErrorKind::NewRMLTranslationError(
                new_rmltranslation_error,
            ) => write!(f, "error while translating RML v2.0 document"),
        }
    }
}

impl std::error::Error for LanguageErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LanguageErrorKind::RMLTranslationError(rmltranslation_error) => {
                Some(rmltranslation_error)
            }
            LanguageErrorKind::ShExMLTranslationError(
                shexml_translation_error,
            ) => Some(shexml_translation_error),
            LanguageErrorKind::NewRMLTranslationError(
                new_rmltranslation_error,
            ) => Some(new_rmltranslation_error),
        }
    }
}
