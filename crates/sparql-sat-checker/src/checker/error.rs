use std::fmt::Display;

pub type CheckerResult<T> = Result<T, CheckerError>;

#[derive(Debug)]
pub struct CheckerError {
    pub kind: CheckerErrorKind,
}

impl Display for CheckerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error occurred while checking for satisfiability")
    }
}

impl std::error::Error for CheckerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.kind)
    }
}

#[derive(Debug)]
pub enum CheckerErrorKind {
    InvalidOperatorConversion(String),
    RegexError {
        iri: String,
        regex_expr: String,
        err: regex::Error,
    },
}

impl Display for CheckerErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckerErrorKind::RegexError {
                iri,
                regex_expr,
                err: _,
            } => write!(
                f,
                "regex error occurred while compiling regex expression {} to match against IRI {}",
                regex_expr, iri
            ),
            CheckerErrorKind::InvalidOperatorConversion(msg) => write!(
                f,
                "errored while converting operator enum to more concrete type with msg: {}",
                msg
            ),
        }
    }
}

impl std::error::Error for CheckerErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CheckerErrorKind::RegexError {
                err,
                iri: _,
                regex_expr: _,
            } => Some(err),
            CheckerErrorKind::InvalidOperatorConversion(_) => None,
        }
    }
}
