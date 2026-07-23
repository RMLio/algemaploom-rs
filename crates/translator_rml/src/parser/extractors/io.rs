//! 
//! Contains functionalities to deal with IO operations for parsing/translating 
//! [RML v1.1.2 document](https://rml.io/specs/rml/)
//!
use std::io::BufRead;

use sophia_api::source::TripleSource;
use sophia_inmem::graph::FastGraph;
use sophia_turtle::parser::turtle;

use super::error::ParseError;
use super::triplesmap_extractor::extract_triples_maps;
use super::ExtractorResult;
use crate::parser::rml_model::{Document, TriplesMap};

fn extract_base_iri(input: &str) -> Option<String> {
    input
        .strip_prefix("@base")
        .map(|e| e[0..e.len() - 1].replace(['<', '>'], "").trim().to_string())
}


/// Parse the input buffer to [sophia's in-memory graph](FastGraph)
///
/// # Error
/// Returns an error if something goes worng with [sophia's turtle parsing](turtle::parse_bufread)
pub fn load_graph_bread(buf_read: impl BufRead) -> ExtractorResult<FastGraph> {
    match turtle::parse_bufread(buf_read).collect_triples() {
        Ok(it) => Ok(it),
        Err(err) => {
            Err(ParseError::GenericError(format!(
                "Something went wrong with sophia's turtle parsing: {}",
                err
            ))
            .into())
        }
    }
}


/// Parse the input string to [sophia's in-memory graph](FastGraph)
///
/// # Error
/// Returns an error if something goes worng with [sophia's turtle parsing](turtle::parse_bufread)
pub fn load_graph_str(input_str: &str) -> ExtractorResult<FastGraph> {
    match turtle::parse_str(input_str).collect_triples() {
        Ok(it) => Ok(it),
        Err(err) => {
            Err(ParseError::GenericError(format!(
                "Something went wrong with sophia's turtle parsing: {}",
                err
            ))
            .into())
        }
    }
}

fn try_create_document(
    triples_maps: Vec<TriplesMap>,
    base_iri: Option<String>,
) -> ExtractorResult<Document> {
    if triples_maps.is_empty() {
        return Err(ParseError::GenericError(
            "no triples maps are sucessfully extracted".to_string(),
        )
        .into());
    }

    Ok(Document {
        triples_maps,
        default_base_iri: base_iri,
    })
}


/// Parses the input str representation of an RML document into the 
/// data model [Document].
///
/// # Errors
///
/// Returns an error if something goes wrong while parsing into the data model.
pub fn parse_str(input_str: &str) -> ExtractorResult<Document> {
    let graph = load_graph_str(input_str)?;
    let triples_maps = extract_triples_maps(&graph)?;
    let base_iri = input_str.split('\n').filter_map(extract_base_iri).next();
    try_create_document(triples_maps, base_iri)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_case;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn one_tm_test() -> ExtractorResult<()> {
        let path = PathBuf::from(test_case!("rml/sample_mapping.ttl"));
        let parsed_res = parse_str(&fs::read_to_string(path)?)?;

        // One TriplesMap should be parsed
        assert_eq!(parsed_res.triples_maps.len(), 1);

        Ok(())
    }

    #[test]
    fn multiple_tm_test() {
        let path = PathBuf::from(test_case!("rml/multiple_tm.ttl"));
        let parsed_res = parse_str(&fs::read_to_string(path).unwrap());

        assert!(parsed_res.is_ok());
        // One TriplesMap should be parsed
        assert_eq!(parsed_res.unwrap().triples_maps.len(), 2);
    }
}
