use sophia_api::term::Term;
use sophia_term::RcTerm;

use super::{Extractor, FromVocab, TermMapExtractor};
use crate::new_rml::extractors::error::ParseError;
use crate::new_rml::rml_model::v2::core::expression_map::term_map::{GraphMap, TermMap};

impl TermMapExtractor<GraphMap> for GraphMap {
    fn create_shortcut_map(term_map: TermMap) -> GraphMap {
        if term_map.is_literal_term_type() {
            panic!("Constant-valued GraphMap has to be either an IRI or a BlankNode");
        }
        Self { term_map }
    }

    fn create_term_map<TTerm>(
        subj_ref: TTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<GraphMap>
    where
        TTerm: Term + Clone,
    {
        let term_map = TermMap::extract_self(subj_ref, graph_ref)?;
        if term_map.is_literal_term_type() {
            Err(ParseError::GenericError("GraphMap has to have a term type of either an IRI or a BlankNode".to_string()).into())
        } else {
            Ok(GraphMap { term_map })
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
