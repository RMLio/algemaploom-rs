use std::collections::HashMap;

use lazy_static::lazy_static;
use sophia_inmem::graph::FastGraph;
use sophia_term::{ArcTerm, RcTerm};

use crate::new_rml::translator::source::extract_parse_config;
use crate::new_rml::extractors::store::get_object;
use crate::new_rml::extractors::{
    stringify_rcterm, ExtractorResult, FromVocab,
};

lazy_static! {
    static ref PARSE_CONFIGS_PREDICATES: Vec<(String, ArcTerm)> = vec![
        (
            vocab::rmls::PROPERTY::HOSTNAME.1.to_string(),
            vocab::rmls::PROPERTY::HOSTNAME.to_arcterm()
        ),
        (
            vocab::rmls::PROPERTY::PORT.1.to_string(),
            vocab::rmls::PROPERTY::PORT.to_arcterm()
        ),
        (
            vocab::rmls::PROPERTY::TOPIC.1.to_string(),
            vocab::rmls::PROPERTY::TOPIC.to_arcterm()
        )
    ];
}

pub fn extract_tcp_source(
    subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<HashMap<String, String>> {
    let config =
        extract_parse_config(subject, graph, &PARSE_CONFIGS_PREDICATES)?;

    Ok(config)
}
