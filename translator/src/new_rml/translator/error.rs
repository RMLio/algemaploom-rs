use std::fmt::Display;

use plangenerator::error::PlanError;

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


impl Display for TranslationError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


impl std::error::Error for TranslationError{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        todo!()
    }
}
