use std::collections::HashMap;

use operator::IOType;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use super::{rcterm_to_string, Extractor, ExtractorResult};
use crate::rml::parser::extractors::store::get_object;
use crate::rml::parser::extractors::FromVocab;
use crate::rml::parser::rml_model::source_target::LdesInformation;

impl Extractor<LdesInformation> for LdesInformation {
    fn extract_self(
        subject: &RcTerm,
        graph: &FastGraph,
    ) -> ExtractorResult<LdesInformation> {
        let ldes_baseiri_pred = vocab::rmlt::PROPERTY::LDESBASE.to_rcterm();
        let ldes_timestamp_path = vocab::ldes::PROPERTY::TIMESTAMPPATH.to_rcterm();
        let ldes_version_of_path = vocab::ldes::PROPERTY::VERSIONOFPATH.to_rcterm();
        let ldes_tree_shape = vocab::tree::PROPERTY::SHAPE.to_rcterm();
        let ldes_eventstream_pred = vocab::rmlt::PROPERTY::LDES.to_rcterm();
        let ldes_generate_immutable_pred = vocab::rmlt::PROPERTY::LDESIMMUTABLE.to_rcterm();
        
        let ldes_base_iri_term = get_object(graph, subject, &ldes_baseiri_pred)?;
        let ldes_base_iri_string = rcterm_to_string(&ldes_base_iri_term);
        let ldes_base_iri = sophia_api::prelude::Iri::new_unchecked(ldes_base_iri_string);
        
        let ldes_generate_immutable_iri = get_object(graph, subject, &ldes_generate_immutable_pred)
            .ok()
            .map(|term| rcterm_to_string(&term) == "true")
            .unwrap_or(false);
        
        let mut ldes_eventstream = HashMap::new();
        
        if let Ok(timestamp_path_term) = get_object(graph, subject, &ldes_timestamp_path) {
            ldes_eventstream.insert(
                "timestampPath".to_string(),
                rcterm_to_string(&timestamp_path_term)
            );
        }
        
        if let Ok(version_of_path_term) = get_object(graph, subject, &ldes_version_of_path) {
            ldes_eventstream.insert(
                "versionOfPath".to_string(),
                rcterm_to_string(&version_of_path_term)
            );
        }
        
        if let Ok(tree_shape_term) = get_object(graph, subject, &ldes_tree_shape) {
            ldes_eventstream.insert(
                "treeShape".to_string(),
                rcterm_to_string(&tree_shape_term)
            );
        }
        
        if let Ok(eventstream_term) = get_object(graph, subject, &ldes_eventstream_pred) {
            ldes_eventstream.insert(
                "eventStream".to_string(),
                rcterm_to_string(&eventstream_term)
            );
        }

        Ok(LdesInformation {
            identifier: rcterm_to_string(subject),
            ldes_eventstream,
            ldes_base_iri,
            ldes_generate_immutable_iri,
        })
    }
}
