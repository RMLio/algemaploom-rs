use std::fmt::Debug;

use sophia_api::graph::Graph;
use sophia_api::prelude::Any;
use sophia_api::term::{FromTerm, Term};
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use super::error::ParseError;
use super::{stringify_rcterm, Extractor};
use crate::new_rml::rml_model::v2::core::expression_map::{
    BaseExpressionMapEnum, ExpressionMapEnum,
};

mod base_expression_enum;

impl Extractor<ExpressionMapEnum> for ExpressionMapEnum {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<ExpressionMapEnum>
    where
        TTerm: Term + Clone,
    {
        if let Ok(base_expr_enum) =
            BaseExpressionMapEnum::extract_self(subject_ref, graph_ref)
        {
            Ok(ExpressionMapEnum::BaseExpressionMap(base_expr_enum))
        } else {
            Err(ParseError::GenericError("Function expression map extraction not implemented yet!".to_string()).into())
        }
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
