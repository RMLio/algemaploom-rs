use sophia_api::term::Term;

use super::store::get_object_with_ps;
use super::Extractor;
use crate::extractors::FromVocab;
use crate::rml_model::v2::core::expression_map::term_map::CommonTermMapInfo;
use crate::rml_model::v2::fnml::{FunctionExecution, FunctionExpressionMap};

mod function_execution;
mod input_map;

impl Extractor<FunctionExpressionMap> for FunctionExpressionMap {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<FunctionExpressionMap>
    where
        TTerm: Term + Clone,
    {
        // Extract rml:functionExecution
        let execution_iri = get_object_with_ps(
            graph_ref,
            subject_ref.borrow_term(),
            &[&vocab::rml_fnml::PROPERTY::FUNCTION_EXECUTION.to_rcterm()],
        )?;
        
        let func_execution = FunctionExecution::extract_self(&execution_iri, graph_ref)?;

        // Extract optional rml:return
        let return_map = get_object_with_ps(
            graph_ref,
            subject_ref.borrow_term(),
            &[&vocab::rml_fnml::PROPERTY::RETURN.to_rcterm()],
        )
        .ok()
        .and_then(|return_iri| CommonTermMapInfo::from_constant_value(return_iri).ok())
        .map(Box::new);

        Ok(FunctionExpressionMap {
            return_map,
            func_execution,
        })
    }
}
