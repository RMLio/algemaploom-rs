use lazy_static::lazy_static;

use sophia_inmem::graph::FastGraph;
use sophia_term::ArcTerm;

use crate::parser::extractors::config_extractor::extract_parse_config;
use crate::parser::extractors::{ExtractorResult, FromVocab, RcTerm};
use crate::parser::rml_model::source_target::{Source, SourceType};

lazy_static! {
    static ref PARSE_CONFIGS_PREDICATES: Vec<(String, ArcTerm)> = vec![
        (
            vocab::d2rq::property::USERNAME.1.to_string(),
            vocab::d2rq::property::USERNAME.to_arcterm()
        ),
        (
            vocab::d2rq::property::PASSWORD.1.to_string(),
            vocab::d2rq::property::PASSWORD.to_arcterm()
        ),
        (
            vocab::d2rq::property::JDBCDSN.1.to_string(),
            vocab::d2rq::property::JDBCDSN.to_arcterm()
        ),
        (
            vocab::d2rq::property::JDBCDRIVER.1.to_string(),
            vocab::d2rq::property::JDBCDRIVER.to_arcterm()
        ),
        (
            vocab::d2rq::property::SQLQUERY.1.to_string(),
            vocab::d2rq::property::SQLQUERY.to_arcterm()
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
