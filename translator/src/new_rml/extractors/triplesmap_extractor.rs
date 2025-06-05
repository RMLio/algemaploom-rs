use sophia_api::graph::Graph;
use sophia_api::prelude::Any;
use sophia_api::term::{FromTerm, Term};
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;
use vocab::ToString;

use super::store::{get_object, get_object_with_ps, get_subjects};
use super::{Extractor, ExtractorResult, RcTerm};
use crate::new_rml::extractors::store::get_objects;
use crate::new_rml::extractors::{FromVocab, TermMapExtractor};
use crate::new_rml::rml_model::v2::core::expression_map::term_map::SubjectMap;
use crate::new_rml::rml_model::v2::core::{
    AbstractLogicalSource, AbstractLogicalSourceEnum, JoinCondition,
    PredicateObjectMap, TriplesMap,
};
use crate::rml::parser::extractors::rcterm_to_string;

impl Extractor<TriplesMap> for TriplesMap {
    fn extract_self<TTerm>(
        subject: TTerm,
        graph: &FastGraph,
    ) -> ExtractorResult<TriplesMap>
    where
        TTerm: Term + Clone,
    {
        let subject_map =
            SubjectMap::extract_from_container(graph, subject.borrow_term())?;

        let ls_new_pred =
            &vocab::rml_core::PROPERTY::LOGICAL_SOURCE.to_rcterm();
        let logical_source_subj =
            get_object_with_ps(graph, subject.borrow_term(), &[ls_new_pred])?;
        let abs_logical_source = AbstractLogicalSourceEnum::extract_self(
            &logical_source_subj,
            graph,
        )?;

        let pom = vocab::rml_core::PROPERTY::PREDICATE_OBJECT_MAP.to_rcterm();
        let po_maps_res: ExtractorResult<Vec<_>> =
            get_objects(graph, subject.borrow_term(), &pom)
                .into_iter()
                .map(|pom_subj| {
                    PredicateObjectMap::extract_self(&pom_subj, graph)
                })
                .collect();
        let po_maps = po_maps_res?;

        // Find all the attributes of this triples maps (tm) which are used in the rr:parent of the
        // referencing object maps in the child triples map (tm')
        let child_ref_obj_maps = get_subjects(
            graph,
            &vocab::rml_core::PROPERTY::PARENT_TRIPLES_MAP.to_rcterm(),
            &subject,
        );

        let join_conditions =
            child_ref_obj_maps.into_iter().flat_map(|ref_obj| {
                get_objects(
                    graph,
                    ref_obj,
                    vocab::rml_core::PROPERTY::JOIN_CONDITION.to_rcterm(),
                )
            });

        let ref_obj_attributes = join_conditions
            .flat_map(|jc| JoinCondition::extract_self(jc, graph))
            .flat_map(|jc| jc.parent.get_value().cloned())
            .collect();

        let base_iri = get_object(
            graph,
            subject.borrow_term(),
            vocab::rml_core::PROPERTY::BASE_IRI.to_rcterm(),
        )
        .ok()
        .map(|base_iri_rcterm| rcterm_to_string(&base_iri_rcterm))
        .unwrap_or_default();

        Ok(TriplesMap {
            identifier: RcTerm::from_term(subject),
            abs_logical_source,
            ref_obj_attributes,
            subject_map,
            base_iri,
            predicate_object_map_vec: po_maps,
        })
    }
}

pub fn extract_triples_maps(
    graph: &FastGraph,
) -> ExtractorResult<Vec<TriplesMap>> {
    let rml_core_subject_map_vec = [
        vocab::rml_core::PROPERTY::SUBJECT_MAP.to_rcterm(),
        vocab::rml_core::PROPERTY::SUBJECT.to_rcterm(),
    ];

    let rml_core_tm_iter =
        graph.triples_matching(Any, rml_core_subject_map_vec, Any);

    rml_core_tm_iter
        .filter_map(|triple| triple.ok())
        .map(|triple| {
            TriplesMap::extract_self(RcTerm::from_term(triple.s()), graph)
        })
        .collect()
}
