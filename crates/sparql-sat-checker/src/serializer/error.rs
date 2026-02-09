use std::fmt::Display;

use operator::{Function, Operator, RcExtendFunction};

use crate::checker::error::CheckerError;

pub type PlanBuildResult<T> = Result<T, PlanBuildError>;

#[derive(Debug)]
pub enum PlanBuildError {
    UnsupportedOperator(String),
    IncorrectInnerFunction(String, String),
    CheckerError(CheckerError)
}
impl From<CheckerError> for PlanBuildError{
    fn from(value: CheckerError) -> Self {
        Self::CheckerError(value)
    }
}

impl Display for PlanBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanBuildError::UnsupportedOperator(operator) => write!(
                        f,
                        "unsupported operator for translating back to RML: {}",
                        operator
                    ),
            PlanBuildError::IncorrectInnerFunction(function, msg) => write!(
                        f,
                        "incorrect inner function found {} with msg {}",
                        function, msg
                    ),
            PlanBuildError::CheckerError(checker_error) => write!(f, "something happened while utilizing checker module's function"),
        }
    }
}

impl std::error::Error for PlanBuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self{
            PlanBuildError::CheckerError(checker_error) => Some(checker_error),
            _ => None,
        }
    }
}
