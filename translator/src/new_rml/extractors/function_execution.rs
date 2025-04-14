use sophia_api::term::Term;

use super::store::{get_objects, get_objects_with_ps};
use super::term_map_extractor::term_map_from_constant_term;
use super::{Extractor, FromVocab};
use crate::new_rml::rml_model::v2::core::expression_map::term_map::TermMap;
use crate::new_rml::rml_model::v2::fnml::{FunctionExecution, InputMap};

impl Extractor<FunctionExecution> for FunctionExecution {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<FunctionExecution>
    where
        TTerm: Term,
    {
        let function = get_objects_with_ps(
            graph_ref,
            subject_ref.borrow_term(),
            &[&vocab::rml_fnml::PROPERTY::FUNCTION.to_rcterm()],
        )
        .into_iter()
        .filter_map(|term| term_map_from_constant_term(term).ok())
        .filter_map(|tm| tm.try_get_node());

        let function_maps = get_objects_with_ps(
            graph_ref,
            subject_ref.borrow_term(),
            &[&vocab::rml_fnml::PROPERTY::FUNCTION_MAP.to_rcterm()],
        )
        .into_iter()
        .filter_map(|term| TermMap::extract_self(term, graph_ref).ok())
        .filter_map(|tm| tm.try_get_node());

        let function = function.chain(function_maps).next().unwrap();

        let input = get_objects(
            graph_ref,
            subject_ref.borrow_term(),
            &vocab::rml_fnml::PROPERTY::INPUT.to_rcterm(),
        )
        .iter()
        .filter_map(|term| InputMap::extract_self(term, graph_ref).ok())
        .collect();

        Ok(FunctionExecution { function, input })
    }
}
