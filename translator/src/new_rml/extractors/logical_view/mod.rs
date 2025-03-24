mod field;
mod join_view;
mod struct_annotation;

use sophia_api::graph::Graph;
use sophia_api::prelude::Any;
use sophia_api::term::{FromTerm, Term};
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use super::store::get_objects;
use super::Extractor;
use crate::new_rml::extractors::store::get_object;
use crate::new_rml::extractors::FromVocab;
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
        let logical_source_term = get_object(
            graph_ref,
            subject_ref.borrow_term(),
            &vocab::rml_lv::PROPERTY::VIEW_ON.to_rcterm(),
        )?;
        let view_on =
            LogicalSource::extract_self(&logical_source_term, graph_ref)?;
        let fields = get_objects(
            graph_ref,
            subject_ref.borrow_term(),
            &vocab::rml_lv::PROPERTY::FIELD.to_rcterm(),
        )
        .iter()
        .filter_map(|term| RMLField::extract_self(term, graph_ref).ok())
        .collect();

        let struct_annotations = get_objects(
            graph_ref,
            subject_ref.borrow_term(),
            &vocab::rml_lv::PROPERTY::STRUCTURAL_ANNOTATION.to_rcterm(),
        )
        .iter()
        .filter_map(|term| {
            StructuralAnnotation::extract_self(term, graph_ref).ok()
        })
        .collect();

        let join_kind_view_pairs = get_joins(subject_ref.borrow_term(), graph_ref);

        Ok(Self {
            identifier: RcTerm::from_term(subject_ref),
            view_on,
            fields,
            struct_annotations,
            join_kind_view_pairs,
        })
    }
}

fn get_joins<TTerm>(
    subject_ref: TTerm,
    graph_ref: &FastGraph,
) -> Vec<(RcTerm, LogicalViewJoin)>
where
    TTerm: Term,
{
    let ijoin_p = vocab::rml_lv::PROPERTY::INNER_JOIN.to_rcterm();
    let ljoin_p = vocab::rml_lv::PROPERTY::LEFT_JOIN.to_rcterm();
    let triples =
        graph_ref.triples_matching([subject_ref], [ijoin_p, ljoin_p], Any);

    let mut result = Vec::new();

    for trip in triples.filter_map(|trip_res| trip_res.ok()) {
        let pair = LogicalViewJoin::extract_self(trip.o(), graph_ref)
            .map(move |vjoin| (RcTerm::from_term(trip.p()), vjoin))
            .unwrap();
        result.push(pair);
    }

    result
}
