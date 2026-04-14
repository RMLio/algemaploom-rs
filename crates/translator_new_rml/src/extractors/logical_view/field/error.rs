use std::error::Error;
use std::fmt::Display;

use crate::extractors::error::SophiaStoreError;

#[derive(Debug, Clone)]
pub enum FieldErrorEnum {
    ConflictingNamesError(Vec<String>),
    SophiaStoreError(SophiaStoreError),
    IterableError(String),
}

impl From<SophiaStoreError> for FieldErrorEnum {
    fn from(value: SophiaStoreError) -> Self {
        Self::SophiaStoreError(value)
    }
}

impl Display for FieldErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldErrorEnum::ConflictingNamesError(items) => {
                write!(
                    f,
                    "field contains following conflicting/overlapping names: {:?}",
                    items
                )
            }
            FieldErrorEnum::SophiaStoreError(_sophia_store_error) => {
                write!(f, "error caused while querying store with sophia_rs")
            }
            FieldErrorEnum::IterableError(msg) => {
                write!(
                    f,
                    "error occurred while parsing rml iterables with msg: {}",
                    msg
                )
            }
        }
    }
}

impl Error for FieldErrorEnum {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FieldErrorEnum::ConflictingNamesError(_items) => None,
            FieldErrorEnum::SophiaStoreError(sophia_store_error) => {
                Some(sophia_store_error)
            }
            FieldErrorEnum::IterableError(_) => None,
        }
    }
}
