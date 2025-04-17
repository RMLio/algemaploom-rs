use lazy_static::lazy_static;
use sophia_inmem::graph::FastGraph;
use sophia_term::{ArcTerm, RcTerm};

use crate::new_rml::translator::source::extract_parse_config;
use crate::new_rml::extractors::{
    ExtractorResult, FromVocab,
};

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
        )
    ];
}

pub fn extract_rdb_source(
    subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<std::collections::HashMap<String, String>> {
    let config =
        extract_parse_config(subject, graph, &PARSE_CONFIGS_PREDICATES)?;

    Ok(config)
}
