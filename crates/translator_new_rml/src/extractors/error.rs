use std::convert::Infallible;
use std::error::Error;
use std::fmt::Display;
use std::io;

#[derive(Debug, Clone)]
pub enum SophiaStoreError {
    TriplesNotFound {
        subj: String,
        pred: String,
        obj:  String,
    },
    SubjectNotFound {
        pred: String,
        obj:  String,
    },
    ObjectNotFound {
        subj: String,
        pred: String,
    },
}

impl Error for SophiaStoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for SophiaStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SophiaStoreError::TriplesNotFound { subj, pred, obj } => {
                write!(
                    f,
                    "error while searching for nonexistent triple: subj: {}  pred: {}  obj: {}",
                    subj, pred, obj
                )
            }
            SophiaStoreError::SubjectNotFound { pred, obj } => 
                write!(
                    f,
                    "error while searching for nonexistent subject with predicate and object: pred: {} obj: {}",
                    pred, obj
                ), 
            SophiaStoreError::ObjectNotFound { subj, pred } => 
                write!(
                    f,
                    "error while searching for nonexistent object with subject and predicate: subj: {} pred: {}",
                   subj,  pred
                ), 
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    IOErrorStr(String),
    IOError(io::Error),
    SerdeError(serde_json::Error),
    SophiaStoreError(SophiaStoreError), 
    GenericError(String),
    NoTermMapFoundError(String),
    ExtensionError(String),
    Infallible,
}

impl From<SophiaStoreError> for ParseError{
    fn from(value: SophiaStoreError) -> Self {
        ParseError::SophiaStoreError(value)
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(value: serde_json::Error) -> Self {
        ParseError::SerdeError(value)
    }
}

impl From<Infallible> for ParseError {
    fn from(_: Infallible) -> Self {
        ParseError::Infallible
    }
}

impl From<io::Error> for ParseError {
    fn from(value: io::Error) -> Self {
        ParseError::IOError(value)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::IOErrorStr(msg) => {
                write!(f, "IO error occurred while parsing with msg: {}", msg)
            }
            ParseError::IOError(_error) => {
                write!(f, "IO error occurred while parsing")
            }
            ParseError::SerdeError(_error) => {
                write!(f, "JSON serde error occurred while parsing")
            }
            ParseError::GenericError(msg) => {
                write!(f, "generic error while parsing with msg: \n {}", msg)
            }
            ParseError::NoTermMapFoundError(msg) => {
                write!(f, "no term map found error with msg: \n {}", msg)
            }
            ParseError::ExtensionError(msg) => {
                write!(f, "file extension error with msg: \n {}", msg)
            }
            ParseError::SophiaStoreError(_sophia_store_error) => write!(f, "error occurred while using sophia_rs's graph store"),
            ParseError::Infallible => panic!("Reached infallible error state, something went really wrong"),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseError::IOError(error) => Some(error),
            ParseError::SerdeError(error) => Some(error),
            ParseError::SophiaStoreError(error) => Some(error),  
            _ => None,
        }
    }
}
