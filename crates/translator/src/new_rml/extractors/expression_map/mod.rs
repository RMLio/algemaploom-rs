use std::fmt::Debug;

use sophia_api::graph::Graph;
use sophia_api::prelude::Any;
use sophia_api::term::{FromTerm, Term};
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use super::error::ParseError;
use super::Extractor;
use crate::new_rml::rml_model::v2::core::expression_map::{
    BaseExpressionMapEnum, ExpressionMapEnum,
};
use crate::new_rml::rml_model::v2::fnml::FunctionExpressionMap;

mod base_expression_enum;

impl Extractor<ExpressionMapEnum> for ExpressionMapEnum {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<ExpressionMapEnum>
    where
        TTerm: Term + Clone,
    {
        // Try base expression map first
        if let Ok(base_expr_enum) =
            BaseExpressionMapEnum::extract_self(subject_ref.clone(), graph_ref)
        {
            return Ok(ExpressionMapEnum::BaseExpressionMap(base_expr_enum));
        }
        
        // Try function expression map
        if let Ok(func_expr_map) =
            FunctionExpressionMap::extract_self(subject_ref, graph_ref)
        {
            return Ok(ExpressionMapEnum::FunctionExpressionMap(func_expr_map));
        }
        
        Err(ParseError::GenericError("Unable to extract expression map (neither base nor function expression map)".to_string()).into())
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
