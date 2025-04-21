use sophia_api::term::Term;

use super::error::ParseError;
use super::store::{get_object, get_object_with_ps};
use super::{Extractor, FromVocab, TermMapExtractor};
use crate::new_rml::rml_model::v2::core::expression_map::term_map::{
    GraphMap, ObjectMap, PredicateMap,
};
use crate::new_rml::rml_model::v2::core::{PredicateObjectMap, RefObjectMap};

impl Extractor<PredicateObjectMap> for PredicateObjectMap {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<PredicateObjectMap>
    where
        TTerm: Term + Clone,
    {
        let predicate_map_vec = PredicateMap::extract_many_from_container(
            graph_ref,
            subject_ref.borrow_term(),
        )?;
        let mut object_pred_vec = ObjectMap::get_const_preds();
        object_pred_vec.append(&mut ObjectMap::get_map_preds());

        let object_terms: Vec<_> = object_pred_vec
            .iter()
            .filter_map(|pred| {
                get_object(graph_ref, subject_ref.borrow_term(), pred).ok()
            })
            .collect();

        if object_terms.is_empty() {
            return Err(ParseError::GenericError(format!(
                "PredicateObject map {:?} contains 0 object maps",
                subject_ref
            ))
            .into());
        }

        let (ref_obj_map_terms, obj_map_terms): (Vec<_>, Vec<_>) =
            object_terms.iter().partition(|term| {
                get_object_with_ps(
                    graph_ref,
                    *term,
                    &[
                        &vocab::r2rml::PROPERTY::PARENTTRIPLESMAP.to_rcterm(),
                        &vocab::rml_core::PROPERTY::PARENT_TRIPLES_MAP
                            .to_rcterm(),
                    ],
                )
                .is_ok()
            });

        let ref_object_map: Vec<_> = ref_obj_map_terms
            .into_iter()
            .filter_map(|term| RefObjectMap::extract_self(term, graph_ref).ok())
            .collect();

        let object_map_vec = ObjectMap::extract_many_from_container(
            graph_ref,
            subject_ref.borrow_term(),
        )
        .ok()
        .unwrap_or(vec![]);

        if object_map_vec.is_empty() && ref_object_map.is_empty() {
            return Err(ParseError::GenericError(format!(
                "Predicate Object Map {:?} has 0 object maps",
                subject_ref
            ))
            .into());
        }

        let graph_map_vec =
            GraphMap::extract_many_from_container(graph_ref, subject_ref)
                .ok()
                .into_iter()
                .flatten()
                .collect();

        Ok(PredicateObjectMap {
            predicate_map_vec,
            object_map_vec,
            ref_object_map,
            graph_map_vec,
        })
    }
}
