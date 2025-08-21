use sophia_api::graph::Graph;
use sophia_api::serializer::{QuadSerializer, Stringifier};
use sophia_api::term::Term;
use sophia_inmem::graph::FastGraph;
use sophia_turtle::serializer::nq::NqSerializer;

use crate::new_rml::extractors::error::ParseError;
use crate::new_rml::extractors::expression_map::get_expr_value_enum;
use crate::new_rml::extractors::store::get_subgraph_subject;
use crate::new_rml::extractors::{
    stringify_term, Extractor, ExtractorResult, FromVocab,
};
use crate::new_rml::rml_model::v2::core::expression_map::BaseExpressionMapEnum;

impl Extractor<BaseExpressionMapEnum> for BaseExpressionMapEnum {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &FastGraph,
    ) -> ExtractorResult<BaseExpressionMapEnum>
    where
        TTerm: Term + Clone,
    {
        if let Some((_, obj)) = get_expr_value_enum(
            subject_ref.borrow_term(),
            graph_ref,
            &[&vocab::rml_core::PROPERTY::TEMPLATE.to_rcterm()],
        ) {
            Ok(BaseExpressionMapEnum::Template(
                stringify_term(obj).unwrap(),
            ))
        } else if let Some((_, obj)) = get_expr_value_enum(
            subject_ref.borrow_term(),
            graph_ref,
            &[&vocab::rml_core::PROPERTY::REFERENCE.to_rcterm()],
        ) {
            Ok(BaseExpressionMapEnum::Reference(
                stringify_term(obj).unwrap(),
            ))
        } else if let Some((_, obj)) = get_expr_value_enum(
            subject_ref.borrow_term(),
            graph_ref,
            &[&vocab::rml_core::PROPERTY::CONSTANT.to_rcterm()],
        ) {
            Ok(BaseExpressionMapEnum::Constant(
                stringify_term(obj).unwrap(),
            ))
        } else {
            let sub_graph =
                get_subgraph_subject(graph_ref, subject_ref.clone())?;
            let mut serializer = NqSerializer::new_stringifier();
            let result = serializer
                .serialize_dataset(&sub_graph.as_dataset())
                .unwrap()
                .as_str();
            Err(ParseError::GenericError(format!(
                "Expression map {:?} is not a base expression map with triples subgraph: \n {:?}",
                subject_ref,
                result
            ))
            .into())
        }
    }
}
