use lazy_static::lazy_static;

use sophia_inmem::graph::FastGraph;
use sophia_term::ArcTerm;

use crate::parser::extractors::config_extractor::extract_parse_config;
use crate::parser::extractors::{ExtractorResult, FromVocab, RcTerm};
use crate::parser::rml_model::source_target::{Source, SourceType};

lazy_static! {
    static ref PARSE_CONFIGS_PREDICATES: Vec<(String, ArcTerm)> = vec![
        (
            vocab::rmls::property::HOSTNAME.1.to_string(),
            vocab::rmls::property::HOSTNAME.to_arcterm()
        ),
        (
            vocab::rmls::property::PORT.1.to_string(),
            vocab::rmls::property::PORT.to_arcterm()
        ),
        (
            vocab::rmls::property::TOPIC.1.to_string(),
            vocab::rmls::property::TOPIC.to_arcterm()
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
