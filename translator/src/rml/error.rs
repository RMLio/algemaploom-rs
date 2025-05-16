use std::error::Error;
use std::fmt::Display;

use plan::error::PlanError;

use super::parser::extractors::error::ParseError;

#[derive(Debug)]
pub enum RMLTranslationError {
    IOError(std::io::Error),
    PlanError(Box<PlanError>),
    ParseError(ParseError),
}

impl Display for RMLTranslationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RMLTranslationError::PlanError(_plan_error) => {
                write!(f, "error while generating algebraic plan")
            }
            RMLTranslationError::ParseError(_parse_error) => {
                write!(f, "error while parsing input RML document")
            }
            RMLTranslationError::IOError(_error) => {
                write!(f, "io error while parsing")
            }
        }
    }
}

impl Error for RMLTranslationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RMLTranslationError::PlanError(plan_error) => Some(plan_error),
            RMLTranslationError::ParseError(parse_error) => {
                Some(parse_error)
            }
            RMLTranslationError::IOError(error) => Some(error),
        }
    }
}

impl From<std::io::Error> for RMLTranslationError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<ParseError> for RMLTranslationError {
    fn from(v: ParseError) -> Self {
        Self::ParseError(v)
    }
}

impl From<PlanError> for RMLTranslationError {
    fn from(v: PlanError) -> Self {
        Self::PlanError(Box::new(v))
    }
}
