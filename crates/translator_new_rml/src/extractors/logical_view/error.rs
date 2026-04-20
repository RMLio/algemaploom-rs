use std::fmt::Display;

use super::field::error::FieldErrorEnum;

#[derive(Debug, Clone)]
pub enum LogicalViewErrorEnum {
    FieldError {
        logical_view_id: String,
        field_error:     FieldErrorEnum,
    },
}

impl Display for LogicalViewErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicalViewErrorEnum::FieldError {
                logical_view_id,
                field_error: _,
            } => {
                write!(
                    f,
                    "Logical view {} contains field related errors ",
                    logical_view_id
                )
            }
        }
    }
}

impl std::error::Error for LogicalViewErrorEnum {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LogicalViewErrorEnum::FieldError {
                logical_view_id: _,
                field_error,
            } => Some(field_error),
        }
    }
}
