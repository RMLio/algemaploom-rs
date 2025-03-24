use std::collections::HashMap;

use lazy_static::lazy_static;
use sophia_inmem::graph::FastGraph;

use super::extract_parse_config;
use crate::extractors::store::get_object;
use crate::extractors::{ExtractorResult, FromVocab, RcTerm};

lazy_static! {
    static ref PARSE_CONFIGS_PREDICATES: Vec<(String, RcTerm)> = vec![
        (
            vocab::csvw::PROPERTY::TRIM.1.to_string(),
            vocab::csvw::PROPERTY::TRIM.to_rcterm()
        ),
        (
            vocab::csvw::PROPERTY::COMMENT_PREFIX.1.to_string(),
            vocab::csvw::PROPERTY::COMMENT_PREFIX.to_rcterm()
        ),
        (
            vocab::csvw::PROPERTY::DELIMITER.1.to_string(),
            vocab::csvw::PROPERTY::DELIMITER.to_rcterm()
        ),
        (
            vocab::csvw::PROPERTY::DOUBLE_QUOTE.1.to_string(),
            vocab::csvw::PROPERTY::DOUBLE_QUOTE.to_rcterm()
        ),
        (
            vocab::csvw::PROPERTY::ENCODING.1.to_string(),
            vocab::csvw::PROPERTY::ENCODING.to_rcterm()
        ),
        (
            vocab::csvw::PROPERTY::HEADER.1.to_string(),
            vocab::csvw::PROPERTY::HEADER.to_rcterm()
        ),
        (
            vocab::csvw::PROPERTY::HEADER_ROW_COUNT.1.to_string(),
            vocab::csvw::PROPERTY::HEADER_ROW_COUNT.to_rcterm()
        ),
        (
            vocab::csvw::PROPERTY::LINE_TERMINATORS.1.to_string(),
            vocab::csvw::PROPERTY::LINE_TERMINATORS.to_rcterm()
        ),
        (
            vocab::csvw::PROPERTY::QUOTE_CHARS.1.to_string(),
            vocab::csvw::PROPERTY::QUOTE_CHARS.to_rcterm()
        ),
        (
            vocab::csvw::PROPERTY::SKIP_ROWS.1.to_string(),
            vocab::csvw::PROPERTY::SKIP_ROWS.to_rcterm()
        ),
        (
            vocab::csvw::PROPERTY::SKIP_COLUMNS.1.to_string(),
            vocab::csvw::PROPERTY::SKIP_COLUMNS.to_rcterm()
        ),
        (
            vocab::csvw::PROPERTY::SKIP_BLANK_ROWS.1.to_string(),
            vocab::csvw::PROPERTY::SKIP_BLANK_ROWS.to_rcterm()
        ),
        (
            vocab::csvw::PROPERTY::SKIP_INITIAL_SPACE.1.to_string(),
            vocab::csvw::PROPERTY::SKIP_INITIAL_SPACE.to_rcterm()
        ),
    ];
}

pub fn extract_csvw_source(
    subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<HashMap<String, String>> {
    let url_pred = vocab::csvw::PROPERTY::URL.to_rcterm();
    let url = get_object(graph, subject, &url_pred)?.value().to_string();
    let dialect_pred = vocab::csvw::PROPERTY::DIALECT.to_rcterm();
    let dialect_iri = get_object(graph, subject, &dialect_pred)?;
    let mut config =
        extract_parse_config(&dialect_iri, graph, &*PARSE_CONFIGS_PREDICATES)?;

    config.insert("url".to_string(), url);
    Ok(config)
}
