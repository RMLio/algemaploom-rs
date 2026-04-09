use std::fmt::{Debug, Display};
use std::io;

pub type ShExMLParseCombiResult<T> = Result<T, ParseCombiError>;

#[derive(Debug, Clone)]
pub struct ParseCombiError {
    pub dbg_msg: String,
    pub msg:     String,
    pub kind:    ParseCombiErrorKind,
}

impl Display for ParseCombiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error Type: {:?}", self.kind)?;
        writeln!(f, "Message: {}", self.msg)
    }
}

#[derive(Debug, Clone)]
pub enum ParseCombiErrorKind {
    LexerError,
    ParserError,
    SerdeError,
    IOError,
}

impl Display for ParseCombiErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseCombiErrorKind::LexerError => {
                write!(
                    f,
                    "Something went wrong while lexing the ShExMLDocument"
                )
            }
            ParseCombiErrorKind::ParserError => {
                write!(
                    f,
                    "Something went wrong while parsing the ShExMLDocument"
                )
            }
            ParseCombiErrorKind::IOError => {
                write!(
                    f,
                    "Something went wrong while reading/writing to a file!"
                )
            }
            ParseCombiErrorKind::SerdeError => {
                write!(f, "Something went wrong while using serde")
            }
        }
    }
}

impl From<io::Error> for ParseCombiError {
    fn from(value: io::Error) -> Self {
        ParseCombiError {
            dbg_msg: format!("{:?}", value),
            msg:     format!("{}", value),
            kind:    ParseCombiErrorKind::IOError,
        }
    }
}
