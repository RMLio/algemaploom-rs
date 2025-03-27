use std::error::Error;
use std::fmt::Display;

use plangenerator::error::PlanError;

use super::parser::extractors::error::ParseError;

#[derive(Debug)]
pub struct RMLTranslationError {
    kind: ErrorKind,
}

impl From<PlanError> for RMLTranslationError {
    fn from(value: PlanError) -> Self {
        Self { kind: value.into() }
    }
}
impl From<ParseError> for RMLTranslationError {
    fn from(value: ParseError) -> Self {
        Self { kind: value.into() }
    }
}

impl From<std::io::Error> for RMLTranslationError {
    fn from(value: std::io::Error) -> Self {
        Self { kind: value.into() }
    }
}

impl Display for RMLTranslationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "error while translating RML v1 document to algebraic mapping plan"
        )
    }
}

impl Error for RMLTranslationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.kind.source()
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    IOError(std::io::Error),
    PlanError(PlanError),
    ParseError(ParseError),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::PlanError(plan_error) => {
                write!(f, "error while generating algebraic plan")
            }
            ErrorKind::ParseError(parse_error) => {
                write!(f, "error while parsing input RML document")
            }
            ErrorKind::IOError(error) => write!(f, "io error while parsing"),
        }
    }
}

impl Error for ErrorKind {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ErrorKind::PlanError(plan_error) => plan_error.source(),
            ErrorKind::ParseError(parse_error) => parse_error.source(),
            ErrorKind::IOError(error) => error.source(),
        }
    }
}

impl From<std::io::Error> for ErrorKind {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<ParseError> for ErrorKind {
    fn from(v: ParseError) -> Self {
        Self::ParseError(v)
    }
}

impl From<PlanError> for ErrorKind {
    fn from(v: PlanError) -> Self {
        Self::PlanError(v)
    }
}
