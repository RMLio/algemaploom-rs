use sophia_api::graph::Graph;
use sophia_api::prelude::Any;
use sophia_api::term::{FromTerm, Term};
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;
use vocab::ToString;

use super::{rcterm_to_string, Extractor, ExtractorResult, RcTerm};
use crate::rml::parser::extractors::store::{get_object, get_objects};
use crate::rml::parser::extractors::{FromVocab, TermMapExtractor};
use crate::rml::parser::rml_model::source_target::LogicalSource;
use crate::rml::parser::rml_model::term_map::SubjectMap;
use crate::rml::parser::rml_model::{PredicateObjectMap, TriplesMap};

impl Extractor<TriplesMap> for TriplesMap {
    fn extract_self(
        subject: &RcTerm,
        graph: &FastGraph,
    ) -> ExtractorResult<TriplesMap> {
        let subject_map = SubjectMap::extract_from_container(graph, subject)?;

        let ls_term = vocab::rml::PROPERTY::LOGICALSOURCE.to_rcterm();
        let logical_source_subj = get_object(graph, subject, &ls_term)?;
        let logical_source =
            LogicalSource::extract_self(&logical_source_subj, graph)?;

        let pom = vocab::r2rml::PROPERTY::PREDICATEOBJECTMAP.to_rcterm();
        let po_maps_res: ExtractorResult<Vec<_>> =
            get_objects(graph, subject, &pom)
                .into_iter()
                .map(|pom_subj| {
                    PredicateObjectMap::extract_self(&pom_subj, graph)
                })
                .collect();
        let po_maps = po_maps_res?;

        Ok(TriplesMap {
            identifier: rcterm_to_string(subject),
            logical_source,
            subject_map,
            po_maps,
        })
    }
}

pub fn extract_triples_maps(
    graph: &FastGraph,
) -> ExtractorResult<Vec<TriplesMap>> {
    let old_rml_tm_iter = graph.triples_matching(
        Any,
        [
            vocab::r2rml::PROPERTY::SUBJECTMAP.to_rcterm(),
            vocab::r2rml::PROPERTY::SUBJECT.to_rcterm(),
        ],
        Any,
    );
    let rml_core_tm_iter = graph.triples_matching(
        Any,
        [
            vocab::rml_core::PROPERTY::SUBJECT_MAP.to_rcterm(),
            vocab::rml_core::PROPERTY::SUBJECT.to_rcterm(),
        ],
        Any,
    );

    old_rml_tm_iter
        .chain(rml_core_tm_iter)
        .filter_map(|triple| triple.ok())
        .map(|sim_triple| RcTerm::from_term(sim_triple.s()))
        .map(|subj| TriplesMap::extract_self(&subj, graph))
        .collect()

    // TODO: if it really needs to be valid at thid point, check for a logical source for old RML
}
