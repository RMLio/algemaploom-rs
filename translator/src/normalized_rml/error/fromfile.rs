use std::error::Error;
use std::fmt::Display;
use std::io;
use std::path::{Path, PathBuf};

use oxigraph::model::IriParseError;

use crate::normalized_rml::error::oxigraph::{OxigraphError, OxigraphErrorKind};

#[derive(Debug)]
pub struct FromFileError {
    pub path: PathBuf,
    pub kind: FromFileErrorKind,
}

impl FromFileError {
    pub fn from_io_err(path: &Path, err: io::Error) -> Self {
        Self {
            path: path.to_path_buf(),
            kind: FromFileErrorKind::ReadFile(err),
        }
    }

    pub fn from_iri_parser_error(path: &Path, err: IriParseError) -> Self {
        Self {
            path: path.to_path_buf(),
            kind: FromFileErrorKind::OxigraphParseError(OxigraphError {
                kind: OxigraphErrorKind::IriParseError(err),
            }),
        }
    }
}

impl Display for FromFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error reading {}", self.path.to_string_lossy())
    }
}

impl Error for FromFileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            FromFileErrorKind::ReadFile(error) => Some(error),
            FromFileErrorKind::OxigraphParseError(error) => Some(error),
            FromFileErrorKind::UnsupportedExtension => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum FromFileErrorKind {
    ReadFile(io::Error),
    UnsupportedExtension,
    OxigraphParseError(OxigraphError),
}

impl From<io::Error> for FromFileErrorKind {
    fn from(value: io::Error) -> Self {
        Self::ReadFile(value)
    }
}
