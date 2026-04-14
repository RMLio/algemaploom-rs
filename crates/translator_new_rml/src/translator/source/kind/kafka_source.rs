use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::extractors::{
    ExtractorResult, FromVocab,
};
use crate::translator::source::extract_parse_config;
use sophia_inmem::graph::FastGraph;
use sophia_term::{ArcTerm, RcTerm};





lazy_static! {
    static ref PARSE_CONFIGS_PREDICATES: Vec<(String, ArcTerm)> = vec![
        (
            vocab::rmls::property::TOPIC.1.to_string(),
            vocab::rmls::property::TOPIC.to_arcterm()
        ),
        (
            vocab::rmls::property::GROUPID.1.to_string(),
            vocab::rmls::property::GROUPID.to_arcterm()
        ),
        (
            vocab::rmls::property::BROKER.1.to_string(),
            vocab::rmls::property::BROKER.to_arcterm()
        )
    ];

}

pub fn extract_kafka_source(
    subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<HashMap<String, String>> {
    let config = extract_parse_config(subject, graph, &PARSE_CONFIGS_PREDICATES)?;

    Ok(config)

}
