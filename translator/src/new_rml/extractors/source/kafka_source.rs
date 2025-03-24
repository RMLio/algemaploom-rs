use std::collections::HashMap;

use lazy_static::lazy_static;
use sophia_inmem::graph::FastGraph;

use super::extract_parse_config;
use crate::extractors::{ExtractorResult, FromVocab, RcTerm};
use crate::TermStr;

lazy_static! {
    static ref PARSE_CONFIGS_PREDICATES: Vec<(String, TermStr)> = vec![
        (
            vocab::rmls::PROPERTY::TOPIC.1.to_string(),
            vocab::rmls::PROPERTY::TOPIC.to_term()
        ),
        (
            vocab::rmls::PROPERTY::GROUPID.1.to_string(),
            vocab::rmls::PROPERTY::GROUPID.to_term()
        ),
        (
            vocab::rmls::PROPERTY::BROKER.1.to_string(),
            vocab::rmls::PROPERTY::BROKER.to_term()
        )
    ];
}

pub fn extract_kafka_source(
    subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<HashMap<String, String>> {
    extract_parse_config(&subject, graph, &*PARSE_CONFIGS_PREDICATES)
}
