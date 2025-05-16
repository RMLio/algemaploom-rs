use std::fmt::Display;

use plan::error::PlanError;

#[derive(Debug)]
pub enum TranslationError {
    SourceError(String),
    ExtendError(String),
    PlanError(PlanError),
    Infallible,
}

impl From<PlanError> for TranslationError {
    fn from(v: PlanError) -> Self {
        Self::PlanError(v)
    }
}

impl Display for TranslationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            TranslationError::SourceError(msg) => write!(f, "error while translating for source operator with msg: {}", msg),
            TranslationError::ExtendError(msg) => write!(f, "error while translating for extend operator with msg: {}", msg),
            TranslationError::PlanError(_) => write!(f, "error while generating the plan"),
            TranslationError::Infallible => write!(f, "something messed up happened and failed when it isn't supposed to!"),
        }
    }
}

impl std::error::Error for TranslationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TranslationError::PlanError(plan_error) => Some(plan_error),
            _ => None,
        }
    }
}
