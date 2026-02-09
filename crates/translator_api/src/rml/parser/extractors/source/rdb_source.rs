

use lazy_static::lazy_static;

use sophia_inmem::graph::FastGraph;
use sophia_term::ArcTerm;

use crate::rml::parser::extractors::{ExtractorResult, FromVocab, RcTerm};
use crate::rml::parser::extractors::config_extractor::extract_parse_config;
use crate::rml::parser::rml_model::source_target::{Source, SourceType};

lazy_static! {
    static ref PARSE_CONFIGS_PREDICATES: Vec<(String, ArcTerm)> = vec![
        (
            vocab::d2rq::PROPERTY::USERNAME.1.to_string(),
            vocab::d2rq::PROPERTY::USERNAME.to_arcterm()
        ),
        (
            vocab::d2rq::PROPERTY::PASSWORD.1.to_string(),
            vocab::d2rq::PROPERTY::PASSWORD.to_arcterm()
        ),
        (
            vocab::d2rq::PROPERTY::JDBCDSN.1.to_string(),
            vocab::d2rq::PROPERTY::JDBCDSN.to_arcterm()
        ),
        (
            vocab::d2rq::PROPERTY::JDBCDriver.1.to_string(),
            vocab::d2rq::PROPERTY::JDBCDriver.to_arcterm()
        ),
        (
            vocab::d2rq::PROPERTY::SQLQUERY.1.to_string(),
            vocab::d2rq::PROPERTY::SQLQUERY.to_arcterm()
        )];

}

pub fn extract_rdb_source(
    subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<Source> {
    let config = extract_parse_config(&subject, graph, &*PARSE_CONFIGS_PREDICATES)?;

    Ok(Source {
        source_type: SourceType::RDB,
        config,
    })

}
