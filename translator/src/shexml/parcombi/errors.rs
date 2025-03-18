use std::fmt::{Debug, Display};
use std::io;

pub type ShExMLResult<T> = Result<T, ShExMLError>;

#[derive(Debug, Clone)]
pub struct ShExMLError {
    pub dbg_msg: String,
    pub msg:     String,
    pub err:     ShExMLErrorType,
}

impl Display for ShExMLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error Type: {:?}", self.err)?;
        writeln!(f, "Message: {}", self.msg)
    }
}

#[derive(Debug, Clone)]
pub enum ShExMLErrorType {
    LexerError,
    ParserError,
    SerdeError,
    IOError,
}

impl Display for ShExMLErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShExMLErrorType::LexerError => {
                write!(
                    f,
                    "Something went wrong while lexing the ShExMLDocument"
                )
            }
            ShExMLErrorType::ParserError => {
                write!(
                    f,
                    "Something went wrong while parsing the ShExMLDocument"
                )
            }
            ShExMLErrorType::IOError => {
                write!(
                    f,
                    "Something went wrong while reading/writing to a file!"
                )
            }
            ShExMLErrorType::SerdeError => {
                write!(f, "Something went wrong while using serde")
            }
        }
    }
}

impl From<io::Error> for ShExMLError {
    fn from(value: io::Error) -> Self {
        ShExMLError {
            dbg_msg: format!("{:?}", value),
            msg:     format!("{}", value),
            err:     ShExMLErrorType::IOError,
        }
    }
}
