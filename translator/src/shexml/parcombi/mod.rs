pub mod errors;
mod lexer;
mod parser;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use chumsky::Parser;
use errors::ShExMLResult;
pub use parser::r#type::*;

use super::parcombi::errors::{ShExMLError, ShExMLErrorType};

pub fn parse_file<P: AsRef<Path>>(
    file_path: P,
) -> ShExMLResult<ShExMLDocument> {
    let mut f = File::open(file_path)?;
    let mut buffer_string = String::new();
    let _ = f.read_to_string(&mut buffer_string);
    parse_string(buffer_string)
}

pub fn parse_string(shexml_doc_string: String) -> ShExMLResult<ShExMLDocument> {
    let tokens_res = lexer::shexml().parse(shexml_doc_string);

    let tokens = tokens_res.or_else(|err| {
        Err(ShExMLError {
            dbg_msg: format!("{:?}", err),
            msg:     format!("{}", ShExMLErrorType::LexerError),
            err:     ShExMLErrorType::LexerError,
        })
    })?;

    let shexml_doc_res = parser::shexml().parse(tokens);

    shexml_doc_res.or_else(|err| {
        Err(ShExMLError {
            dbg_msg: format!("{:?}", err),
            msg:     format!("{}", ShExMLErrorType::ParserError),
            err:     ShExMLErrorType::ParserError,
        })
    })
}
