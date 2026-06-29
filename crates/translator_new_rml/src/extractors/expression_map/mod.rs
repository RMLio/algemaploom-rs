use sophia_api::graph::Graph;
use sophia_api::prelude::Any;
use sophia_api::term::{FromTerm, Term};
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;
use std::fmt::Debug;

use super::error::ParseError;
use super::Extractor;
use crate::rml_model::v2::core::expression_map::{
    BaseExpressionMapEnum, ExpressionMapEnum,
};
use crate::rml_model::v2::fnml::FunctionExpressionMap;

mod base_expression_enum;

impl Extractor<ExpressionMapEnum> for ExpressionMapEnum {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &FastGraph,
    ) -> super::ExtractorResult<ExpressionMapEnum>
    where
        TTerm: Term + Clone,
    {
        let mut err_str = format!("{:?}: Unable to extract expression map:\n", subject_ref);
        // Try base expression map first
        match BaseExpressionMapEnum::extract_self(subject_ref.clone(), graph_ref) {
            Ok(base_expr_enum) => {
                return Ok(ExpressionMapEnum::BaseExpressionMap(base_expr_enum));
            }
            Err(err) => {
                err_str.push_str(&format!("Failed to extract base expression map: {}\n", err));
            }
        }
        
        // Try function expression map
        match FunctionExpressionMap::extract_self(subject_ref.clone(), graph_ref) {
            Ok(function_expr_map) => {
                return Ok(ExpressionMapEnum::FunctionExpressionMap(function_expr_map));
            }
            Err(err) => {
                err_str.push_str(&format!("Failed to extract function expression map: {}\n", err));
            }
        }

        // Nothing could be extracted => error
        Err(ParseError::GenericError(err_str).into())
    }
}

/// Given a subject IRI (s), a list of predicattes (P) and a graph (G)
/// return an optional pair (p,o) of predicate (p) and object (o) such that
/// (s,p,o) ∈ G with p ∈ P
pub fn get_expr_value_enum<TS, TP>(
    subject_ref: TS,
    graph_ref: &FastGraph,
    preds: &[TP],
) -> Option<(RcTerm, RcTerm)>
where
    TP: Term + Debug,
    TS: Term + Debug,
{
    graph_ref
        .triples_matching([subject_ref], preds, Any)
        .filter_map(|trip_res| trip_res.ok())
        .map(|trip| (RcTerm::from_term(trip.p()), RcTerm::from_term(trip.o())))
        .next()
}
