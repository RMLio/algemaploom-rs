use sophia_api::term::Term;

use crate::new_rml::extractors::store::{get_objects, get_objects_with_ps};
use crate::new_rml::extractors::{Extractor, ExtractorResult, FromVocab};
use crate::new_rml::rml_model::v2::core::expression_map::term_map::CommonTermMapInfo;
use crate::new_rml::rml_model::v2::fnml::{FunctionExecution, InputMap};

impl Extractor<FunctionExecution> for FunctionExecution {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> ExtractorResult<FunctionExecution>
    where
        TTerm: Term,
    {
        let function = get_objects_with_ps(
            graph_ref,
            subject_ref.borrow_term(),
            &[&vocab::rml_fnml::PROPERTY::FUNCTION.to_rcterm()],
        )
        .into_iter()
        .filter_map(|term| CommonTermMapInfo::from_constant_value(term).ok());

        let function_maps = get_objects_with_ps(
            graph_ref,
            subject_ref.borrow_term(),
            &[&vocab::rml_fnml::PROPERTY::FUNCTION_MAP.to_rcterm()],
        )
        .into_iter()
        .filter_map(|term| {
            CommonTermMapInfo::extract_self(term, graph_ref).ok()
        });

        let _function = function.chain(function_maps).next().unwrap();

        let _input: Vec<_> = get_objects(
            graph_ref,
            subject_ref.borrow_term(),
            vocab::rml_fnml::PROPERTY::INPUT.to_rcterm(),
        )
        .iter()
        .filter_map(|term| InputMap::extract_self(term, graph_ref).ok())
        .collect();

        todo!()
        //Ok(FunctionExecution {
        //    function_map: todo!(),
        //    input: todo!(),
        //})
    }
}
