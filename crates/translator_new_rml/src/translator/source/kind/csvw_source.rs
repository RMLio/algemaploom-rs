use std::collections::HashMap;

use lazy_static::lazy_static;
use sophia_inmem::graph::FastGraph;
use sophia_term::{ArcTerm, RcTerm};

use crate::extractors::store::get_object;
use crate::extractors::{ExtractorResult, FromVocab};
use crate::translator::source::extract_parse_config;

lazy_static! {
    /// What the table itself says, next to the dialect it points at.
    static ref TABLE_PREDICATES: Vec<(String, ArcTerm)> = vec![
        (
            vocab::csvw::property::URL.1.to_string(),
            vocab::csvw::property::URL.to_arcterm()
        ),
        (
            vocab::csvw::property::NULL.1.to_string(),
            vocab::csvw::property::NULL.to_arcterm()
        ),
    ];

    /// How the rows are written, read from the dialect the table points at.
    static ref DIALECT_PREDICATES: Vec<(String, ArcTerm)> = vec![
        (
            vocab::csvw::property::TRIM.1.to_string(),
            vocab::csvw::property::TRIM.to_arcterm()
        ),
        (
            vocab::csvw::property::COMMENT_PREFIX.1.to_string(),
            vocab::csvw::property::COMMENT_PREFIX.to_arcterm()
        ),
        (
            vocab::csvw::property::DELIMITER.1.to_string(),
            vocab::csvw::property::DELIMITER.to_arcterm()
        ),
        (
            vocab::csvw::property::DOUBLE_QUOTE.1.to_string(),
            vocab::csvw::property::DOUBLE_QUOTE.to_arcterm()
        ),
        (
            vocab::csvw::property::ENCODING.1.to_string(),
            vocab::csvw::property::ENCODING.to_arcterm()
        ),
        (
            vocab::csvw::property::HEADER.1.to_string(),
            vocab::csvw::property::HEADER.to_arcterm()
        ),
        (
            vocab::csvw::property::HEADER_ROW_COUNT.1.to_string(),
            vocab::csvw::property::HEADER_ROW_COUNT.to_arcterm()
        ),
        (
            vocab::csvw::property::LINE_TERMINATORS.1.to_string(),
            vocab::csvw::property::LINE_TERMINATORS.to_arcterm()
        ),
        (
            vocab::csvw::property::QUOTE_CHAR.1.to_string(),
            vocab::csvw::property::QUOTE_CHAR.to_arcterm()
        ),
        (
            vocab::csvw::property::SKIP_ROWS.1.to_string(),
            vocab::csvw::property::SKIP_ROWS.to_arcterm()
        ),
        (
            vocab::csvw::property::SKIP_COLUMNS.1.to_string(),
            vocab::csvw::property::SKIP_COLUMNS.to_arcterm()
        ),
        (
            vocab::csvw::property::SKIP_BLANK_ROWS.1.to_string(),
            vocab::csvw::property::SKIP_BLANK_ROWS.to_arcterm()
        ),
        (
            vocab::csvw::property::SKIP_INITIAL_SPACE.1.to_string(),
            vocab::csvw::property::SKIP_INITIAL_SPACE.to_arcterm()
        ),
    ];
}

/// The configuration of a CSV on the Web table.
///
/// The table says where the data is (`csvw:url`) and which values stand for no value at
/// all (`csvw:null`); how the rows are written — the delimiter, the quote character, the
/// encoding — is said by the dialect the table points at, and is read from there. A table
/// without a dialect is read with the defaults, so a missing dialect is not an error.
pub fn extract_csvw_source(
    subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<HashMap<String, String>> {
    let mut config = extract_parse_config(subject, graph, &TABLE_PREDICATES)?;

    let dialect_pred = vocab::csvw::property::DIALECT.to_rcterm();
    if let Ok(dialect_subject) = get_object(graph, subject, &dialect_pred) {
        config.extend(extract_parse_config(
            &dialect_subject,
            graph,
            &DIALECT_PREDICATES,
        )?);
    }

    Ok(config)
}
