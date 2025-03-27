use std::fmt::Display;

use plangenerator::error::PlanError;

use super::parcombi::errors::ParseCombiError;

#[derive(Debug)]
pub struct ShExMLTranslationError {
    kind: ErrorKind,
}

impl From<PlanError> for ShExMLTranslationError {
    fn from(value: PlanError) -> Self {
        Self { kind: value.into() }
    }
}
impl From<ParseCombiError> for ShExMLTranslationError {
    fn from(value: ParseCombiError) -> Self {
        Self { kind: value.into() }
    }
}

impl Display for ShExMLTranslationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error while processing a ShExML document")
    }
}

impl std::error::Error for ShExMLTranslationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.kind.source()
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    ParseCombiError(ParseCombiError),
    IOError(std::io::Error),
    PlanError(PlanError),
}

impl From<ParseCombiError> for ErrorKind {
    fn from(v: ParseCombiError) -> Self {
        Self::ParseCombiError(v)
    }
}

impl From<PlanError> for ErrorKind {
    fn from(v: PlanError) -> Self {
        Self::PlanError(v)
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::ParseCombiError(parse_combi_error) => {
                write!(f, "error while parsing/lexing ShExML document")
                    .and_then(|_| write!(f, "{}", parse_combi_error))
            }
            ErrorKind::IOError(err) => write!(f, "io error ocurred"),
            ErrorKind::PlanError(err) => {
                write!(f, "error while translating to algebraic plan")
            }
        }
    }
}

impl std::error::Error for ErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ErrorKind::IOError(error) => error.source(),
            ErrorKind::PlanError(error) => error.source(),
            _ => None,
        }
    }
}
