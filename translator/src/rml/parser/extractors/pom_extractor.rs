use sophia_term::RcTerm;

use super::Extractor;
use crate::rml::parser::extractors::TermMapExtractor;
use crate::rml::parser::rml_model::term_map::{GraphMap, ObjectMap, PredicateMap};
use crate::rml::parser::rml_model::PredicateObjectMap;

impl Extractor<PredicateObjectMap> for PredicateObjectMap {
    fn extract_self(
        subject_ref: &RcTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<PredicateObjectMap> {
        let predicate_maps =
            PredicateMap::extract_many_from_container(graph_ref, subject_ref)?;

        let object_maps =
            ObjectMap::extract_many_from_container(graph_ref, subject_ref)?;
        let graph_maps =
            GraphMap::extract_many_from_container(graph_ref, subject_ref)?;

        Ok(PredicateObjectMap {
            predicate_maps,
            object_maps,
            graph_maps,
        })
    }
}
