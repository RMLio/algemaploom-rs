use sophia_api::term::Term;
use sophia_term::RcTerm;

use super::{Extractor, FromVocab, TermMapExtractor};
use crate::new_rml::extractors::error::ParseError;
use crate::new_rml::rml_model::v2::core::expression_map::term_map::{GraphMap, CommonTermMapInfo};
use crate::new_rml::rml_model::v2::TermMapEnum;

impl TermMapExtractor<TermMapEnum> for GraphMap {
    fn create_shortcut_map(term_map_info: CommonTermMapInfo) -> TermMapEnum {
        if term_map_info.is_literal_term_type() {
            panic!("Constant-valued GraphMap has to be either an IRI or a BlankNode");
        }
        TermMapEnum::GraphMap(Self { term_map_info })
    }

    fn create_term_map<TTerm>(
        subj_ref: TTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<TermMapEnum>
    where
        TTerm: Term + Clone,
    {
        let term_map_info = CommonTermMapInfo::extract_self(subj_ref, graph_ref)?;
        if term_map_info.is_literal_term_type() {
            Err(ParseError::GenericError("GraphMap has to have a term type of either an IRI, UnsafeIRI, URI, UnsafeURI or a BlankNode".to_string()).into())
        } else {
            Ok(TermMapEnum::GraphMap(GraphMap { term_map_info }))
        }
    }

    fn get_shortcut_preds() -> Vec<RcTerm> {
        vec![
            vocab::rml_core::PROPERTY::GRAPH.to_rcterm(),
            vocab::r2rml::PROPERTY::GRAPH.to_rcterm(),
        ]
    }

    fn get_map_preds() -> Vec<RcTerm> {
        vec![
            vocab::rml_core::PROPERTY::GRAPH_MAP.to_rcterm(),
            vocab::r2rml::PROPERTY::GRAPHMAP.to_rcterm(),
        ]
    }
}
