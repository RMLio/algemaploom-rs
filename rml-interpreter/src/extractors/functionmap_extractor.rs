use sophia_api::term::TTerm;

use super::store::get_objects;
use super::{Extractor, FromVocab};
use crate::rml_model::term_map::FunctionMap;
use crate::rml_model::PredicateObjectMap;

impl Extractor<FunctionMap> for FunctionMap {
    fn extract_self(
        subject_ref: &sophia_term::RcTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<FunctionMap> {
        let pom_pred = vocab::r2rml::PROPERTY::PREDICATEOBJECTMAP.to_term();

        let po_maps = get_objects(graph_ref, subject_ref, &pom_pred)
            .into_iter()
            .filter_map(|pom_subj| {
                PredicateObjectMap::extract_self(&pom_subj, graph_ref).ok()
            });

        let executes_pred_iri = vocab::fno::PROPERTY::EXECUTES.to_term();
        let (execute_poms, params_poms): (Vec<_>, Vec<_>) =
            po_maps.partition(|pom| {
                pom.predicate_maps
                    .iter()
                    .filter(|pm| pm.tm_info.term_value == executes_pred_iri)
                    .count()
                    == 1
            });

        let function_iri = execute_poms
            .into_iter()
            .flat_map(|pom| pom.object_maps)
            .map(|om| om.tm_info.term_value.value().to_string())
            .nth(0)
            .unwrap();

        let param_om_pairs: Vec<_> = params_poms
            .into_iter()
            .map(|mut pom| {
                (
                    pom.predicate_maps.pop().unwrap(),
                    pom.object_maps.pop().unwrap(),
                )
            })
            .map(|(pm, om)| (pm.tm_info.term_value.value().to_string(), om))
            .collect();

        Ok(FunctionMap {
            identifier: subject_ref.value().to_string(),
            function_iri,
            param_om_pairs,
        })
    }
}