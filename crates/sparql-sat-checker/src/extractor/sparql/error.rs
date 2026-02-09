use spargebra::SparqlSyntaxError;
use std::{fmt::Display, path::Path};
#[derive(Debug)]
pub struct SparqlExtractError {
    pub query: Box<str>,
    pub kind: ExtractErrorKind,
}
impl Display for SparqlExtractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "error while extracting triples patterns from query: {}",
            self.query
        )
    }
}

impl std::error::Error for SparqlExtractError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ExtractErrorKind::SparqlParseError(sparql_syntax_error) => Some(sparql_syntax_error),
        }
    }
}

#[derive(Debug)]
pub enum ExtractErrorKind {
    SparqlParseError(SparqlSyntaxError),
}

#[derive(Debug)]
pub struct FromFileError {
    pub path: Box<Path>,
    pub kind: FromFileErrorKind,
}

impl Display for FromFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error while processing file: {}", self.path.display())
    }
}

impl std::error::Error for FromFileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            FromFileErrorKind::StdIoError(error) => Some(error),
            FromFileErrorKind::ExtractError(extract_error) => Some(extract_error),
        }
    }
}

#[derive(Debug)]
pub enum FromFileErrorKind {
    ExtractError(SparqlExtractError), 
    StdIoError(std::io::Error),
}
