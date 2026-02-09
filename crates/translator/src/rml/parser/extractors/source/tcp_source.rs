

use lazy_static::lazy_static;

use sophia_inmem::graph::FastGraph;
use sophia_term::ArcTerm;

use crate::rml::parser::extractors::{ExtractorResult, FromVocab, RcTerm};
use crate::rml::parser::extractors::config_extractor::extract_parse_config;
use crate::rml::parser::rml_model::source_target::{Source, SourceType};

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
) -> ExtractorResult<Source> {
    let config = extract_parse_config(&subject, graph, &*PARSE_CONFIGS_PREDICATES)?;

    Ok(Source {
        source_type: SourceType::TCP,
        config,
    })

}
