use sophia_api::graph::Graph;
use sophia_api::prelude::Any;
use sophia_api::term::{FromTerm, Term};
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;

use super::store::{get_object, get_object_with_ps, get_subjects};
use super::{Extractor, ExtractorResult, RcTerm};
use crate::new_rml::extractors::store::get_objects;
use crate::new_rml::extractors::{FromVocab, TermMapExtractor};
use crate::new_rml::rml_model::v2::core::expression_map::term_map::SubjectMap;
use crate::new_rml::rml_model::v2::core::{
    AbstractLogicalSourceEnum, JoinCondition,
    PredicateObjectMap, TriplesMap,
};
use crate::new_rml::rml_model::v2::RefAttributeGetter;
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
            .flat_map(|jc| jc.parent.get_ref_attributes())
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
    // There are 3 different ways triples maps are defined:
    // 1. Regular RML:TRIPLES_MAP
    // 2. Special triples maps used with RMLStar, for example AssertedTriplesMap and NonAssertedTriplesMap
    // 3. Implicit triples maps, used in the fnml tests. Here a subjectmap / logicalsource is given but it is not explicitly defined a triplesmap.

    // Case 1: Regular RML:TRIPLES_MAP
    let rml_core_tm_iter = graph.triples_matching(
        Any,
        [vocab::rdf::PROPERTY::TYPE.to_rcterm()],
        [vocab::rml_core::CLASS::TRIPLES_MAP.to_rcterm()],
    );

    let explicit_tms: ExtractorResult<Vec<TriplesMap>> = rml_core_tm_iter
        .filter_map(|triple| triple.ok())
        .map(|triple| {
            TriplesMap::extract_self(RcTerm::from_term(triple.s()), graph)
        })
        .collect();

    // TODO: Case 2

    // Case 3: Implicit defined triplesmaps
    // First collect all explicitly defined triplesmap subjects, to filter these triplesmaps out of the implicit ones.
    let explicit_tm_subjects: std::collections::HashSet<_> = graph
        .triples_matching(
            Any,
            [vocab::rdf::PROPERTY::TYPE.to_rcterm()],
            [vocab::rml_core::CLASS::TRIPLES_MAP.to_rcterm()],
        )
        .filter_map(|triple| triple.ok())
        .map(|triple| RcTerm::from_term(triple.s()))
        .collect();

    let logical_source_pred =
        vocab::rml_core::PROPERTY::LOGICAL_SOURCE.to_rcterm();
    let subject_map_pred = vocab::rml_core::PROPERTY::SUBJECT_MAP.to_rcterm();
    let mut implicit_tm_subjects = std::collections::HashSet::new();

    graph
        .triples_matching(Any, [logical_source_pred.clone()], Any)
        .flatten()
        .for_each(|triple| {
            let subj = RcTerm::from_term(triple.s());
            // Check if this subject also has a subject map
            let has_subject_map = graph
                .triples_matching(
                    [subj.clone()],
                    [subject_map_pred.clone()],
                    Any,
                )
                .next()
                .is_some();

            if has_subject_map && !explicit_tm_subjects.contains(&subj) {
                implicit_tm_subjects.insert(subj);
            }
        });

    // Combine explicit and implicit triplesmaps
    let mut all_tms = explicit_tms?;
    let implicit_tms: ExtractorResult<Vec<TriplesMap>> = implicit_tm_subjects
        .into_iter()
        .map(|subj| TriplesMap::extract_self(subj, graph))
        .collect();
    all_tms.extend(implicit_tms?);

    Ok(all_tms)
}
