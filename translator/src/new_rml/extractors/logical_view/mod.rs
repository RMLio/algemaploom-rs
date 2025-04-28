mod field;
mod join_view;
mod struct_annotation;

use std::collections::HashSet;

use log::debug;
use sophia_api::graph::Graph;
use sophia_api::prelude::Any;
use sophia_api::term::{FromTerm, Term};
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use super::error::ParseError;
use super::store::get_objects;
use super::Extractor;
use crate::new_rml::extractors::store::get_object;
use crate::new_rml::extractors::FromVocab;
use crate::new_rml::rml_model::v2::core::{AbstractLogicalSource, AbstractLogicalSourceEnum};
use crate::new_rml::rml_model::v2::io::source::LogicalSource;
use crate::new_rml::rml_model::v2::lv::{
    LogicalView, LogicalViewJoin, RMLField, StructuralAnnotation,
};

impl Extractor<LogicalView> for LogicalView {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &FastGraph,
    ) -> super::ExtractorResult<LogicalView>
    where
        TTerm: Term,
    {
        let subject_ref = RcTerm::from_term(subject_ref);

        log::debug!("Checking for cyclical join for {:?}", subject_ref);
        // Checks for cyclic joins and throws error if there is one
        check_cyclic_join_or_error(
            subject_ref.clone(),
            graph_ref,
            &mut HashSet::new(),
        )?;
        // Checks for cyclic views and throws error if there is one
        log::debug!("Checking for cyclical view for {:?}", subject_ref);
        check_cyclic_view_or_error(
            subject_ref.clone(),
            graph_ref,
            &mut HashSet::new(),
        )?;

        let logical_source_term = get_object(
            graph_ref,
            &subject_ref,
            vocab::rml_lv::PROPERTY::VIEW_ON.to_rcterm(),
        )?;
        let view_on_abs =
            AbstractLogicalSourceEnum::extract_self(&logical_source_term, graph_ref)?;
        let fields = get_objects(
            graph_ref,
            &subject_ref,
            vocab::rml_lv::PROPERTY::FIELD.to_rcterm(),
        )
        .iter()
        .filter_map(|term| RMLField::extract_self(term, graph_ref).ok())
        .collect();

        let struct_annotations = get_objects(
            graph_ref,
            subject_ref.borrow_term(),
            vocab::rml_lv::PROPERTY::STRUCTURAL_ANNOTATION.to_rcterm(),
        )
        .iter()
        .filter_map(|term| {
            StructuralAnnotation::extract_self(term, graph_ref).ok()
        })
        .collect();

        let join_kind_view_pairs =
            get_joins(subject_ref.borrow_term(), graph_ref)?;

        Ok(Self {
            identifier: RcTerm::from_term(subject_ref),
            view_on: Box::new(view_on_abs),
            fields,
            struct_annotations,
            join_kind_view_pairs,
        })
    }
}

fn check_cyclic_view_or_error(
    subject_ref: RcTerm,
    graph_ref: &FastGraph,
    visited: &mut HashSet<RcTerm>,
) -> super::ExtractorResult<()> {
    visited.insert(subject_ref.clone());
    let views = get_objects(
        graph_ref,
        &subject_ref,
        vocab::rml_lv::PROPERTY::VIEW_ON.to_rcterm(),
    );

    if views.is_empty() {
        return Ok(()); 
    }

    for view in views {
        if visited.contains(&view) {
            return Err(ParseError::GenericError(format!(
                "Cyclic view detected for logical view {:?} with another logical view {:?}",
                subject_ref,
                view
            ))
            .into());
        } else {
            check_cyclic_view_or_error(
                view,
                graph_ref,
                visited,
            )?;
        }
    }

    Ok(())
}

fn check_cyclic_join_or_error(
    subject_ref: RcTerm,
    graph_ref: &FastGraph,
    visited: &mut HashSet<RcTerm>,
) -> super::ExtractorResult<()> {
    visited.insert(subject_ref.clone());
    let join_preds = [
        vocab::rml_lv::PROPERTY::INNER_JOIN.to_rcterm(),
        vocab::rml_lv::PROPERTY::LEFT_JOIN.to_rcterm(),
    ];

    let triples: Vec<_> = graph_ref
        .triples_matching([subject_ref.clone()], join_preds, Any)
        .filter_map(|trip_res| trip_res.ok())
        .collect();

    let parent_lv_vec: Vec<_> = triples
        .iter()
        .flat_map(|trip| {
            graph_ref.triples_matching(
                [trip.o()],
                [vocab::rml_lv::PROPERTY::PARENT_LOGICAL_VIEW.to_rcterm()],
                Any,
            )
        })
        .flatten()
        .map(|trip| RcTerm::from_term(trip.o()))
        .collect();

    for parent_lv in parent_lv_vec.iter() {
        debug!(
            "Checking for cyclic join beewteen {:?} and {:?}",
            subject_ref, parent_lv
        );
        if visited.contains(parent_lv) {
            return Err(ParseError::GenericError(format!(
                "Cyclic join detected for logical view {:?} with parent logical view {:?}",
                subject_ref,
                parent_lv
            ))
            .into());
        } else {
            check_cyclic_join_or_error(parent_lv.clone(), graph_ref, visited)?;
        }
    }
    Ok(())
}

fn get_joins<TTerm>(
    subject_ref: TTerm,
    graph_ref: &FastGraph,
) -> super::ExtractorResult<Vec<(RcTerm, LogicalViewJoin)>>
where
    TTerm: Term,
{
    let ijoin_p = vocab::rml_lv::PROPERTY::INNER_JOIN.to_rcterm();
    let ljoin_p = vocab::rml_lv::PROPERTY::LEFT_JOIN.to_rcterm();
    let triples: Vec<_> = graph_ref
        .triples_matching([subject_ref], [ijoin_p, ljoin_p], Any)
        .filter_map(|trip_res| trip_res.ok())
        .collect();

    let mut result = Vec::new();

    for trip in triples {
        let pair = LogicalViewJoin::extract_self(trip.o(), graph_ref)
            .map(move |vjoin| (RcTerm::from_term(trip.p()), vjoin))
            .unwrap();
        result.push(pair);
    }

    Ok(result)
}
