use sophia_api::term::Term;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use super::error::ParseError;
use super::{Extractor, ExtractorResult, TermMapExtractor};
use crate::extractors::FromVocab;
use crate::rml_model::v2::core::expression_map::term_map::{
    CommonTermMapInfo, PredicateMap,
};
use crate::rml_model::v2::TermMapEnum;

impl TermMapExtractor<TermMapEnum> for PredicateMap {
    fn create_shortcut_map(
        term_map_info: CommonTermMapInfo,
    ) -> ExtractorResult<TermMapEnum> {
        if !term_map_info.is_iri_term_type() {
            return Err(ParseError::GenericError(
                "Constant-valued PredicateMap has to have an IRI as value"
                    .to_string(),
            ));
        }
        Ok(TermMapEnum::PredicateMap(PredicateMap { term_map_info }))
    }

    fn extract_self_term_map<TS>(
        subj_ref: TS,
        graph_ref: &FastGraph,
    ) -> super::ExtractorResult<TermMapEnum>
    where
        TS: Term + Clone,
    {
        let term_map_info =
            CommonTermMapInfo::extract_self(subj_ref, graph_ref)?;
        Ok(TermMapEnum::PredicateMap(PredicateMap { term_map_info }))
    }

    fn get_shortcut_preds() -> Vec<RcTerm> {
        vec![
            vocab::r2rml::property::PREDICATE.to_rcterm(),
            vocab::rml_core::property::PREDICATE.to_rcterm(),
        ]
    }

    fn get_map_preds() -> Vec<RcTerm> {
        vec![
            vocab::r2rml::property::PREDICATEMAP.to_rcterm(),
            vocab::rml_core::property::PREDICATE_MAP.to_rcterm(),
        ]
    }
}
