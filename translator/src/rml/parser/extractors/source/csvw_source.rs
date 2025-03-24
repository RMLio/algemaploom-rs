use lazy_static::lazy_static;
use sophia_api::term::Term;
use sophia_inmem::graph::FastGraph;
use sophia_term::ArcTerm;

use crate::rml::parser::extractors::config_extractor::extract_parse_config;
use crate::rml::parser::extractors::store::get_object;
use crate::rml::parser::extractors::{
    rcterm_to_string, ExtractorResult, FromVocab, RcTerm,
};
use crate::rml::parser::rml_model::source_target::{Source, SourceType};

lazy_static! {
    static ref PARSE_CONFIGS_PREDICATES: Vec<(String, ArcTerm)> = vec![
        (
            vocab::csvw::PROPERTY::TRIM.1.to_string(),
            vocab::csvw::PROPERTY::TRIM.to_arcterm()
        ),
        (
            vocab::csvw::PROPERTY::COMMENT_PREFIX.1.to_string(),
            vocab::csvw::PROPERTY::COMMENT_PREFIX.to_arcterm()
        ),
        (
            vocab::csvw::PROPERTY::DELIMITER.1.to_string(),
            vocab::csvw::PROPERTY::DELIMITER.to_arcterm()
        ),
        (
            vocab::csvw::PROPERTY::DOUBLE_QUOTE.1.to_string(),
            vocab::csvw::PROPERTY::DOUBLE_QUOTE.to_arcterm()
        ),
        (
            vocab::csvw::PROPERTY::ENCODING.1.to_string(),
            vocab::csvw::PROPERTY::ENCODING.to_arcterm()
        ),
        (
            vocab::csvw::PROPERTY::HEADER.1.to_string(),
            vocab::csvw::PROPERTY::HEADER.to_arcterm()
        ),
        (
            vocab::csvw::PROPERTY::HEADER_ROW_COUNT.1.to_string(),
            vocab::csvw::PROPERTY::HEADER_ROW_COUNT.to_arcterm()
        ),
        (
            vocab::csvw::PROPERTY::LINE_TERMINATORS.1.to_string(),
            vocab::csvw::PROPERTY::LINE_TERMINATORS.to_arcterm()
        ),
        (
            vocab::csvw::PROPERTY::QUOTE_CHARS.1.to_string(),
            vocab::csvw::PROPERTY::QUOTE_CHARS.to_arcterm()
        ),
        (
            vocab::csvw::PROPERTY::SKIP_ROWS.1.to_string(),
            vocab::csvw::PROPERTY::SKIP_ROWS.to_arcterm()
        ),
        (
            vocab::csvw::PROPERTY::SKIP_COLUMNS.1.to_string(),
            vocab::csvw::PROPERTY::SKIP_COLUMNS.to_arcterm()
        ),
        (
            vocab::csvw::PROPERTY::SKIP_BLANK_ROWS.1.to_string(),
            vocab::csvw::PROPERTY::SKIP_BLANK_ROWS.to_arcterm()
        ),
        (
            vocab::csvw::PROPERTY::SKIP_INITIAL_SPACE.1.to_string(),
            vocab::csvw::PROPERTY::SKIP_INITIAL_SPACE.to_arcterm()
        ),
    ];
}

pub fn extract_csvw_source(
    subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<Source> {
    let url_pred = vocab::csvw::PROPERTY::URL.to_arcterm();
    let url = rcterm_to_string(&get_object(graph, subject, &url_pred)?);
    let dialect_pred = vocab::csvw::PROPERTY::DIALECT.to_arcterm();
    let dialect_iri = get_object(graph, subject, &dialect_pred)?;
    let mut config =
        extract_parse_config(&dialect_iri, graph, &*PARSE_CONFIGS_PREDICATES)?;

    config.insert("url".to_string(), url);
    Ok(Source {
        source_type: SourceType::CSVW,
        config,
    })
}
