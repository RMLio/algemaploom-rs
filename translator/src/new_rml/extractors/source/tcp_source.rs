use std::collections::HashMap;

use lazy_static::lazy_static;
use sophia_inmem::graph::FastGraph;

use super::extract_parse_config;
use crate::extractors::{ExtractorResult, FromVocab, RcTerm};
use crate::TermStr;

lazy_static! {
    static ref PARSE_CONFIGS_PREDICATES: Vec<(String, TermStr)> = vec![
        (
            vocab::rmls::PROPERTY::HOSTNAME.1.to_string(),
            vocab::rmls::PROPERTY::HOSTNAME.to_term()
        ),
        (
            vocab::rmls::PROPERTY::PORT.1.to_string(),
            vocab::rmls::PROPERTY::PORT.to_term()
        ),
        (
            vocab::rmls::PROPERTY::TOPIC.1.to_string(),
            vocab::rmls::PROPERTY::TOPIC.to_term()
        )
    ];
}

pub fn extract_tcp_source(
    subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<HashMap<String, String>> {
    extract_parse_config(&subject, graph, &*PARSE_CONFIGS_PREDICATES)
}
