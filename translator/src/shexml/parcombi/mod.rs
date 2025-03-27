pub mod errors;
mod lexer;
mod parser;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use chumsky::Parser;
use errors::ShExMLParseCombiResult;
pub use parser::r#type::*;

use super::parcombi::errors::{ParseCombiError, ParseCombiErrorKind};

pub fn parse_file<P: AsRef<Path>>(
    file_path: P,
) -> ShExMLParseCombiResult<ShExMLDocument> {
    let mut f = File::open(file_path)?;
    let mut buffer_string = String::new();
    let _ = f.read_to_string(&mut buffer_string);
    parse_string(buffer_string)
}

pub fn parse_string(shexml_doc_string: String) -> ShExMLParseCombiResult<ShExMLDocument> {
    let tokens_res = lexer::shexml().parse(shexml_doc_string);

    let tokens = tokens_res.or_else(|err| {
        Err(ParseCombiError {
            dbg_msg: format!("{:?}", err),
            msg:     format!("{}", ParseCombiErrorKind::LexerError),
            kind:     ParseCombiErrorKind::LexerError,
        })
    })?;

    let shexml_doc_res = parser::shexml().parse(tokens);

    shexml_doc_res.or_else(|err| {
        Err(ParseCombiError {
            dbg_msg: format!("{:?}", err),
            msg:     format!("{}", ParseCombiErrorKind::ParserError),
            kind:     ParseCombiErrorKind::ParserError,
        })
    })
}
