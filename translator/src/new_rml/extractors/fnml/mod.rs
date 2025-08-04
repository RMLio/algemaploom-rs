use super::Extractor;
use crate::new_rml::rml_model::v2::fnml::FunctionExpressionMap;

mod function_execution;
mod input_map;

impl Extractor<FunctionExpressionMap> for FunctionExpressionMap {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<FunctionExpressionMap>
    where
        TTerm: sophia_api::prelude::Term + Clone,
    {
        
        todo!()
    }
}
