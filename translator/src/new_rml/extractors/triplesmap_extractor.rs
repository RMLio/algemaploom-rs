use sophia_api::graph::Graph;
use sophia_api::prelude::Any;
use sophia_api::term::{FromTerm, Term};
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;
use vocab::ToString;

use super::store::get_object_with_ps;
use super::{stringify_rcterm, Extractor, ExtractorResult, RcTerm};
use crate::new_rml::extractors::store::{get_object, get_objects};
use crate::new_rml::extractors::{FromVocab, TermMapExtractor};
use crate::new_rml::rml_model::v2::core::expression_map::term_map::SubjectMap;
use crate::new_rml::rml_model::v2::core::{
    AbstractLogicalSource, PredicateObjectMap, TriplesMap,
};

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

        let ls_old_pred = &vocab::rml::PROPERTY::LOGICALSOURCE.to_rcterm();
        let ls_new_pred =
            &vocab::rml_core::PROPERTY::LOGICAL_SOURCE.to_rcterm();
        let logical_source_subj = get_object_with_ps(
            graph,
            subject.borrow_term(),
            &[ls_old_pred, ls_new_pred],
        )?;
        let abs_logical_source =
            AbstractLogicalSource::extract_self(&logical_source_subj, graph)?;

        let pom = vocab::r2rml::PROPERTY::PREDICATEOBJECTMAP.to_rcterm();
        let po_maps_res: ExtractorResult<Vec<_>> =
            get_objects(graph, subject.borrow_term(), &pom)
                .into_iter()
                .map(|pom_subj| {
                    PredicateObjectMap::extract_self(&pom_subj, graph)
                })
                .collect();
        let po_maps = po_maps_res?;

        Ok(TriplesMap {
            base_iri: "".to_string(),
            identifier: RcTerm::from_term(subject),
            abs_logical_source,
            subject_map,
            predicate_object_map_vec: po_maps,
        })
    }
}

pub fn extract_triples_maps(
    graph: &FastGraph,
) -> ExtractorResult<Vec<TriplesMap>> {
    let old_rml_subject_map: RcTerm =
        vocab::r2rml::PROPERTY::SUBJECTMAP.to_rcterm();
    let rml_core_subject_map: RcTerm =
        vocab::rml_core::PROPERTY::SUBJECT_MAP.to_rcterm();

    let old_rml_tm_iter =
        graph.triples_matching(Any, [old_rml_subject_map], Any);
    let rml_core_tm_iter =
        graph.triples_matching(Any, [rml_core_subject_map], Any);

    old_rml_tm_iter
        .chain(rml_core_tm_iter)
        .filter_map(|triple| triple.ok())
        .map(|triple| {
            TriplesMap::extract_self(RcTerm::from_term(triple.s()), graph)
        })
        .collect()

    // TODO: if it really needs to be valid at thid point, check for a logical source for old RML
}
