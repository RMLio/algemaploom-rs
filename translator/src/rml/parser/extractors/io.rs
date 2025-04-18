use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

use sophia_api::source::TripleSource;
use sophia_inmem::graph::FastGraph;
use sophia_turtle::parser::turtle;

use super::error::ParseError;
use super::triplesmap_extractor::extract_triples_maps;
use super::ExtractorResult;
use crate::rml::parser::rml_model::Document;

fn extract_base_iri(input: &str) -> Option<String> {
    input.strip_prefix("@base").map(|e| e[0..e.len()-1].replace(['<', '>'], "").trim().to_string())
}

pub fn load_graph_bread(buf_read: impl BufRead) -> ExtractorResult<FastGraph> {
    match turtle::parse_bufread(buf_read).collect_triples() {
        Ok(it) => Ok(it),
        Err(err) => {
            Err(ParseError::GenericError(format!(
                "Something went wrong with sophia's turtle parsing: {}",
                err
            )))
        }
    }
}

pub fn load_graph_str(input_str: &str) -> ExtractorResult<FastGraph> {
    match turtle::parse_str(input_str).collect_triples() {
        Ok(it) => Ok(it),
        Err(err) => {
            Err(ParseError::GenericError(format!(
                "Something went wrong with sophia's turtle parsing: {}",
                err
            )))
        }
    }
}

pub fn parse_str(input_str: &str) -> ExtractorResult<Document> {
    let graph = load_graph_str(input_str)?;
    let triples_maps = extract_triples_maps(&graph)?;
    let base_iri = input_str.split('\n').filter_map(extract_base_iri).next();
    Ok(Document {
        triples_maps,
        default_base_iri: base_iri,
    })
}

pub fn parse_file(path: PathBuf) -> ExtractorResult<Document> {
    if let Some(ext) = path.extension() {
        if ext != "ttl" {
            return Err(ParseError::ExtensionError(format!(
                "Extension does not exist {}",
                ext.to_str().unwrap()
            )));
        }

        let buf_read = BufReader::new(File::open(path.clone())?);
        let triples_maps = extract_triples_maps(&load_graph_bread(buf_read)?)?;

        // TODO: Refactor extraction of base iri from RML file <02-08-24, SMO> //
        let mut buf_read = BufReader::new(File::open(path)?);
        let mut input_string = String::default();
        buf_read.read_to_string(&mut input_string)?;
        let base_iri = input_string.split('\n').filter_map(extract_base_iri).next();

        return Ok(Document {
            triples_maps,
            default_base_iri: base_iri,
        });
    }

    Err(ParseError::IOErrorStr(format!(
        "File can't be read {}",
        path.to_str().unwrap()
    )))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_case;

    #[test]
    fn one_tm_test() -> ExtractorResult<()> {
        let path = PathBuf::from(test_case!("rml/sample_mapping.ttl"));
        let parsed_res = parse_file(path)?;

        // One TriplesMap should be parsed
        assert!(parsed_res.triples_maps.len() == 1);

        Ok(())
    }

    #[test]
    fn multiple_tm_test() {
        let path = PathBuf::from(test_case!("rml/multiple_tm.ttl"));
        let parsed_res = parse_file(path);

        assert!(parsed_res.is_ok());
        // One TriplesMap should be parsed
        assert!(parsed_res.unwrap().triples_maps.len() == 2);
    }
}
