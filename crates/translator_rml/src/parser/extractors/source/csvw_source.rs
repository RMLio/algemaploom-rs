use lazy_static::lazy_static;
use sophia_inmem::graph::FastGraph;
use sophia_term::ArcTerm;

use crate::parser::extractors::config_extractor::extract_parse_config;
use crate::parser::extractors::store::get_object;
use crate::parser::extractors::{
    rcterm_to_string, ExtractorResult, FromVocab, RcTerm,
};
use crate::parser::rml_model::source_target::{Source, SourceType};

lazy_static! {
    static ref PARSE_CONFIGS_PREDICATES: Vec<(String, ArcTerm)> = vec![
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
            vocab::csvw::property::QUOTE_CHARS.1.to_string(),
            vocab::csvw::property::QUOTE_CHARS.to_arcterm()
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

pub fn extract_csvw_source(
    subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<Source> {
    let url_pred = vocab::csvw::property::URL.to_arcterm();
    let url = rcterm_to_string(&get_object(graph, subject, &url_pred)?);
    let dialect_pred = vocab::csvw::property::DIALECT.to_arcterm();
    let dialect_iri = get_object(graph, subject, &dialect_pred)?;
    let mut config =
        extract_parse_config(&dialect_iri, graph, &*PARSE_CONFIGS_PREDICATES)?;

    config.insert("url".to_string(), url);
    Ok(Source {
        source_type: SourceType::CSVW,
        config,
    })
}
