use std::collections::HashMap;

use lazy_static::lazy_static;
use sophia_inmem::graph::FastGraph;
use sophia_term::{ArcTerm, RcTerm};

use crate::new_rml::extractors::{ExtractorResult, FromVocab};
use crate::new_rml::translator::source::extract_parse_config;

lazy_static! {
    static ref PARSE_CONFIGS_PREDICATES: Vec<(String, ArcTerm)> = vec![
        (
            vocab::rml_io::PROPERTY::ROOT.1.to_string(),
            vocab::rml_io::PROPERTY::ROOT.to_arcterm()
        ),
        (
            vocab::rml_io::PROPERTY::PATH.1.to_string(),
            vocab::rml_io::PROPERTY::PATH.to_arcterm()
        ),
    ];
}

pub fn extract_file_source(
    subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<HashMap<String, String>> {
    let config =
        extract_parse_config(subject, graph, &PARSE_CONFIGS_PREDICATES)?;

    Ok(config)
}
