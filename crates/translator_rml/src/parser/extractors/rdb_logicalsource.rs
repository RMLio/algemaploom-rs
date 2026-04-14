use lazy_static::lazy_static;
use sophia_inmem::graph::FastGraph;
use sophia_term::{ArcTerm, RcTerm};

use crate::parser::extractors::config_extractor::extract_parse_config;
use crate::parser::extractors::{ExtractorResult, FromVocab};
use crate::parser::rml_model::source_target::{Source, SourceType};

lazy_static! {
    static ref PARSE_CONFIGS_PREDICATES: Vec<(String, ArcTerm)> = vec![
        (
            vocab::rml::PROPERTY::QUERY.1.to_string(),
            vocab::rml::PROPERTY::QUERY.to_arcterm()
        ),
        (
            vocab::r2rml::property::TABLENAME.1.to_string(),
            vocab::r2rml::property::TABLENAME.to_arcterm()
        ),
        (
            vocab::r2rml::property::SQLVERSION.1.to_string(),
            vocab::r2rml::property::SQLVERSION.to_arcterm()
        ),
        (
            vocab::r2rml::property::COLUMN.1.to_string(),
            vocab::r2rml::property::COLUMN.to_arcterm()
        )
    ];
}

pub fn update_with_logicalsource(
    subject: &RcTerm,
    graph: &FastGraph,
    old_source: &Source,
) -> ExtractorResult<Source> {
    let mut config =
        extract_parse_config(&subject, graph, &*PARSE_CONFIGS_PREDICATES)?;
    config.extend(old_source.config.clone());
    Ok(Source {
        source_type: SourceType::RDB,
        config,
    })
}
