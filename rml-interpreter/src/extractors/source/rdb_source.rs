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
            vocab::d2rq::PROPERTY::USERNAME.1.to_string(),
            vocab::d2rq::PROPERTY::USERNAME.to_term()
        ),
        (
            vocab::d2rq::PROPERTY::PASSWORD.1.to_string(),
            vocab::d2rq::PROPERTY::PASSWORD.to_term()
        ),
        (
            vocab::d2rq::PROPERTY::JDBCDSN.1.to_string(),
            vocab::d2rq::PROPERTY::JDBCDSN.to_term()
        ),
        (
            vocab::d2rq::PROPERTY::JDBCDriver.1.to_string(),
            vocab::d2rq::PROPERTY::JDBCDriver.to_term()
        ),
        (
            vocab::d2rq::PROPERTY::SQLQUERY.1.to_string(),
            vocab::d2rq::PROPERTY::SQLQUERY.to_term()
        )];

}

pub fn extract_rdb_source(
    subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<Source> {
    let mut config = extract_parse_config(&subject, graph, &*PARSE_CONFIGS_PREDICATES)?;

    Ok(Source {
        source_type: SourceType::RDB,
        config,
    })

}