use sophia_api::graph::Graph;
use sophia_api::serializer::{QuadSerializer, Stringifier};
use sophia_api::term::Term;
use sophia_inmem::graph::FastGraph;
use sophia_turtle::serializer::nq::NqSerializer;

use crate::extractors::error::ParseError;
use crate::extractors::expression_map::get_expr_value_enum;
use crate::extractors::store::get_subgraph_subject;
use crate::extractors::{
    stringify_term, Extractor, ExtractorResult, FromVocab,
};
use crate::rml_model::v2::core::expression_map;
use crate::rml_model::v2::core::expression_map::BaseExpressionMapEnum;

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
            &[&vocab::rml_core::property::TEMPLATE.to_rcterm()],
        ) {
            let term_str = stringify_term(obj).unwrap();
            let template = expression_map::base_expr::Template::try_from(term_str)?;
            Ok(BaseExpressionMapEnum::Template(template))
        } else if let Some((_, obj)) = get_expr_value_enum(
            subject_ref.borrow_term(),
            graph_ref,
            &[&vocab::rml_core::property::REFERENCE.to_rcterm()],
        ) {
            Ok(BaseExpressionMapEnum::Reference(
                stringify_term(obj).unwrap(),
            ))
        } else if let Some((_, obj)) = get_expr_value_enum(
            subject_ref.borrow_term(),
            graph_ref,
            &[&vocab::rml_core::property::CONSTANT.to_rcterm()],
        ) {
            Ok(BaseExpressionMapEnum::Constant(obj))
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
