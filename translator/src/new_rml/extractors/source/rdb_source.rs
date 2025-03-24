use std::collections::HashMap;

use lazy_static::lazy_static;
use sophia_inmem::graph::FastGraph;

use crate::extractors::{ExtractorResult, FromVocab, RcTerm};
use crate::TermStr;

use super::extract_parse_config;

lazy_static! {
    static ref PARSE_CONFIGS_PREDICATES: Vec<(String, TermStr)> = vec![
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
        )
    ];
}

pub fn extract_rdb_source(
    subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<HashMap<String, String>> {
    extract_parse_config(&subject, graph, &*PARSE_CONFIGS_PREDICATES)
}
