use std::fmt::Debug;

use sophia_api::graph::Graph;
use sophia_api::prelude::Any;
use sophia_api::term::{FromTerm, Term};
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use super::store::get_objects_with_ps;
use super::term_map_extractor::term_map_from_constant_term;
use super::{stringify_rcterm, Extractor};
use crate::new_rml::extractors::FromVocab;
use crate::new_rml::rml_model::v2::core::expression_map::{
    ExpressionMap, ExpressionMapKind,
};
use crate::new_rml::rml_model::v2::fnml::FunctionExecution;

impl Extractor<ExpressionMap> for ExpressionMap {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<ExpressionMap>
    where
        TTerm: Term + Clone,
    {
        let (value_pred, value_term) =
            ExpressionMap::extract_self(subject_ref.borrow_term(), graph_ref)?;
        let kind = if value_pred
            == vocab::rml_fnml::PROPERTY::FUNCTION_EXECUTION.to_rcterm()
        {
            let returns = get_objects_with_ps(
                graph_ref,
                subject_ref.borrow_term(),
                &[
                    &vocab::rml_fnml::PROPERTY::RETURN_MAP.to_rcterm(),
                    &vocab::rml_fnml::PROPERTY::RETURN.to_rcterm(),
                ],
            )
            .iter()
            .filter_map(|term| term_map_from_constant_term(term).ok())
            .filter_map(|tm| tm.try_get_node())
            .collect();

            ExpressionMapKind::FunctionExecution {
                execution: FunctionExecution::extract_self(
                    &value_term,
                    graph_ref,
                )?,
                returns,
            }
        } else {
            ExpressionMapKind::NonFunction(
                stringify_rcterm(value_term).unwrap(),
            )
        };

        Ok(ExpressionMap {
            map_type_pred_iri: value_pred,
            kind,
        })
    }
}

impl Extractor<(RcTerm, RcTerm)> for ExpressionMap {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<(RcTerm, RcTerm)>
    where
        TTerm: Term,
    {
        get_expr_value_enum(
            subject_ref.borrow_term(),
            graph_ref,
            &[
                &vocab::r2rml::PROPERTY::TEMPLATE.to_rcterm(),
                &vocab::rml_core::PROPERTY::TEMPLATE.to_rcterm(),
            ],
        )
        .or(get_expr_value_enum(
            subject_ref.borrow_term(),
            graph_ref,
            &[
                &vocab::rml::PROPERTY::REFERENCE.to_rcterm(),
                &vocab::rml_core::PROPERTY::REFERENCE.to_rcterm(),
            ],
        ))
        .or(get_expr_value_enum(
            subject_ref.borrow_term(),
            graph_ref,
            &[&vocab::rml_fnml::PROPERTY::FUNCTION_EXECUTION.to_rcterm()],
        ))
        .or(get_expr_value_enum(
            subject_ref.borrow_term(),
            graph_ref,
            &[
                &vocab::r2rml::PROPERTY::CONSTANT.to_rcterm(),
                &vocab::rml_core::PROPERTY::CONSTANT.to_rcterm(),
            ],
        ))
        .ok_or(super::error::ParseError::GenericError(format!(
            "Cannot parse term map type for {:?}",
            subject_ref
        )))
    }
}

fn get_expr_value_enum<TS, TP>(
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
