use std::error::Error;
use std::fmt::Display;

use oxigraph::model::IriParseError;
use oxigraph::sparql::EvaluationError;
use oxigraph::store::StorageError;

#[derive(Debug)]
pub struct OxigraphError {
    pub kind: OxigraphErrorKind,
}

impl OxigraphError {
    pub fn new(kind: OxigraphErrorKind) -> Self {
        Self { kind }
    }
}

impl From<EvaluationError> for OxigraphError {
    fn from(value: EvaluationError) -> Self {
        Self {
            kind: OxigraphErrorKind::EvaluationError(value),
        }
    }
}

impl Display for OxigraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error caused by oxigraph functions")
    }
}

impl Error for OxigraphError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.kind)
    }
}

#[derive(Debug)]
pub enum OxigraphErrorKind {
    GraphInitError,
    EvaluationError(EvaluationError),
    IriParseError(IriParseError),
    StorageError(StorageError),
    // TODO: Check the usage of generic errors again and replace them with appropriate error type! <06-03-25, SMO> //
    GenericError(String),
}

impl Display for OxigraphErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OxigraphErrorKind::GraphInitError => {
                write!(f, "error while initializaing graph")
            }
            OxigraphErrorKind::GenericError(val) => {
                write!(f, "generic error from oxigraph: {}", val)
            }
            OxigraphErrorKind::EvaluationError(_) => {
                write!(f, "error while evaluating the query")
            }
            OxigraphErrorKind::IriParseError(_) => {
                write!(f, "error while parsing iri")
            }
            OxigraphErrorKind::StorageError(_) => {
                write!(f, "error while using oxigraph storage operations")
            }
        }
    }
}

impl Error for OxigraphErrorKind {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            OxigraphErrorKind::EvaluationError(err) => Some(err),
            OxigraphErrorKind::IriParseError(err) => Some(err),
            OxigraphErrorKind::StorageError(err) => Some(err),
            _ => None,
        }
    }
}
