use std::fmt::Display;

use plan::error::PlanError;

use super::parcombi::errors::ParseCombiError;

#[derive(Debug)]
pub enum ShExMLTranslationError {
    ParseCombiError(ParseCombiError),
    IOError(std::io::Error),
    PlanError(Box<PlanError>),
}

impl From<ParseCombiError> for ShExMLTranslationError {
    fn from(v: ParseCombiError) -> Self {
        Self::ParseCombiError(v)
    }
}

impl From<PlanError> for ShExMLTranslationError {
    fn from(v: PlanError) -> Self {
        Self::PlanError(Box::new(v))
    }
}

impl Display for ShExMLTranslationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShExMLTranslationError::ParseCombiError(parse_combi_error) => {
                write!(f, "error while parsing/lexing ShExML document")
                    .and_then(|_| write!(f, "{}", parse_combi_error))
            }
            ShExMLTranslationError::IOError(_err) => {
                write!(f, "io error ocurred")
            }
            ShExMLTranslationError::PlanError(_err) => {
                write!(f, "error while translating to algebraic plan")
            }
        }
    }
}

impl std::error::Error for ShExMLTranslationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ShExMLTranslationError::IOError(error) => Some(error),
            ShExMLTranslationError::PlanError(error) => Some(error),
            _ => None,
        }
    }
}
