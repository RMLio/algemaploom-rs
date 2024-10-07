use lazy_static::lazy_static;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;
use crate::extractors::{ExtractorResult, FromVocab};
use crate::extractors::config_extractor::extract_parse_config;
use crate::rml_model::source_target::{Source, SourceType};
use crate::TermString;

lazy_static! {
    static ref PARSE_CONFIGS_PREDICATES: Vec<(String, TermString)> = vec![
        (
            vocab::rml::PROPERTY::QUERY.1.to_string(),
            vocab::rml::PROPERTY::QUERY.to_term()
        ),
        (
            vocab::r2rml::PROPERTY::TABLENAME.1.to_string(),
            vocab::r2rml::PROPERTY::TABLENAME.to_term()
        ),
        (
            vocab::r2rml::PROPERTY::SQLVERSION.1.to_string(),
            vocab::r2rml::PROPERTY::SQLVERSION.to_term()
        ),
        (
            vocab::r2rml::PROPERTY::COLUMN.1.to_string(),
            vocab::r2rml::PROPERTY::COLUMN.to_term()
        )
    ];
}

pub fn update_with_logicalsource(
    subject: &RcTerm,
    graph: &FastGraph,
    old_source: &Source,
) -> ExtractorResult<Source> {
    let mut config = extract_parse_config(&subject, graph, &*PARSE_CONFIGS_PREDICATES)?;
    config.extend(old_source.config.clone());
    Ok(Source {
        source_type: SourceType::RDB,
        config,
    })
}