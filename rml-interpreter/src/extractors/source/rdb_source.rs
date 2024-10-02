use std::collections::HashMap;

use lazy_static::lazy_static;
use sophia_api::term::TTerm;
use sophia_inmem::graph::FastGraph;

use crate::extractors::store::get_object;
use crate::extractors::{ExtractorResult, FromVocab, RcTerm};
use crate::rml_model::source_target::{Source, SourceType};
use crate::TermString;

lazy_static! {
    static ref PARSE_CONFIGS_PREDICATES: Vec<(String, TermString)> = vec![
        (
            vocab::d2rq::PROPERTY::DATABASE.1.to_string(),
            vocab::d2rq::PROPERTY::DATABASE.to_term()
        ),
        (
            vocab::d2rq::PROPERTY::COLUMN.1.to_string(),
            vocab::d2rq::PROPERTY::COLUMN.to_term()
        ),
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
            vocab::r2rml::PROPERTY::TABLE.1.to_string(),
            vocab::r2rml::PROPERTY::TABLE.to_term()
        ),
        (
            vocab::d2rq::PROPERTY::SQLQUERY.1.to_string(),
            vocab::d2rq::PROPERTY::SQLQUERY.to_term()
        )];
}

pub fn extract_parse_config(
    dialect_subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<HashMap<String, String>> {
    let mut result = HashMap::new();

    let _ = PARSE_CONFIGS_PREDICATES.iter().try_for_each(
        |(key, config_pred)| -> ExtractorResult<()> {
            let config_val = get_object(graph, dialect_subject, config_pred)?;
            result.insert(key.to_string(), config_val.value().to_string());
            Ok(())
        },
    );

    Ok(result)
}

pub fn extract_rdb_source(
    subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<Source> {
    let url_pred = vocab::d2rq::PROPERTY::JDBCDSN.to_rcterm();
    let url = get_object(graph, subject, &url_pred)?.value().to_string();
    let dialect_pred = vocab::d2rq::PROPERTY::DATABASE.to_rcterm();
    let dialect_iri = get_object(graph, subject, &dialect_pred)?;
    let mut config = extract_parse_config(&dialect_iri, graph)?;

    config.insert("url".to_string(), url);
    Ok(Source {
        source_type: SourceType::RDB,
        config,
    })
}
