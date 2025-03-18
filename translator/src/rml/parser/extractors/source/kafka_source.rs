

use lazy_static::lazy_static;

use sophia_inmem::graph::FastGraph;



use crate::rml::parser::extractors::{ExtractorResult, FromVocab, RcTerm};
use crate::rml::parser::extractors::config_extractor::extract_parse_config;
use crate::rml::parser::rml_model::source_target::{Source, SourceType};
use crate::rml::parser::TermString;


lazy_static! {
    static ref PARSE_CONFIGS_PREDICATES: Vec<(String, TermString)> = vec![
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
) -> ExtractorResult<Source> {
    let config = extract_parse_config(&subject, graph, &*PARSE_CONFIGS_PREDICATES)?;

    Ok(Source {
        source_type: SourceType::Kafka,
        config,
    })

}
