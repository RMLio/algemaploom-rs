use std::collections::HashMap;

use lazy_static::lazy_static;
use sophia_api::term::TTerm;
use sophia_inmem::graph::FastGraph;
use crate::extractors::store::get_object;
use crate::extractors::{ExtractorResult, FromVocab, RcTerm};
use crate::extractors::config_extractor::extract_parse_config;
use crate::rml_model::source_target::{LogicalSource, Source, SourceType};
use crate::TermString;

lazy_static! {
    static ref PARSE_CONFIGS_PREDICATES: Vec<(String, TermString)> = vec![
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
) -> ExtractorResult<Source> {
    let mut config = extract_parse_config(&subject, graph, &*PARSE_CONFIGS_PREDICATES)?;

    Ok(Source {
        source_type: SourceType::TCP,
        config,
    })

}