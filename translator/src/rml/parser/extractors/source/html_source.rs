use lazy_static::lazy_static;
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
        
        (vocab::rml::PROPERTY::ITERATOR.1.to_string(),
         vocab::rml::PROPERTY::ITERATOR.to_arcterm()),

    ];
}

pub fn extract_html_source(
    subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<Source> {
    let config = extract_parse_config(&subject, graph, &*PARSE_CONFIGS_PREDICATES)?;
    Ok(Source {
        source_type: SourceType::HTML,
        config,
    })
}
