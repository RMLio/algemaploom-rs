use std::collections::HashMap;

use operator::IOType;
use sophia_api::term::TTerm;
use sophia_inmem::graph::FastGraph;
use sophia_term::iri::Iri;
use sophia_term::RcTerm;

use super::{Extractor, ExtractorResult};
use crate::rml::parser::extractors::store::get_object;
use crate::rml::parser::extractors::FromVocab;
use crate::rml::parser::rml_model::source_target::LogicalTarget;

fn extract_output_target(
    target_subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<(IOType, HashMap<String, String>)> {
    if let Ok(output_path_iri) = get_object(
        graph,
        target_subject,
        &vocab::void::PROPERTY::DATA_DUMP.to_rcterm(),
    ) {
        let path = output_path_iri.value().to_string();

        return Ok((IOType::File, HashMap::from([("path".to_string(), path)])));
    }

    if let Ok(sparql_endpoint_iri) = get_object(
        graph,
        target_subject,
        &vocab::void::PROPERTY::SPARQL_ENDPOINT.to_rcterm(),
    ) {
        let sparql_path = sparql_endpoint_iri.value().to_string();

        return Ok((
            IOType::SPARQLEndpoint,
            HashMap::from([("sparql_uri".to_string(), sparql_path)]),
        ));
    }

    Err(super::error::ParseError::GenericError(format!(
        "Void dataset extraction failed for {}",
        target_subject
    )))
}

impl Extractor<LogicalTarget> for LogicalTarget {
    fn extract_self(
        subject: &sophia_term::RcTerm,
        graph: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<LogicalTarget> {
        let target_pred = vocab::rmlt::PROPERTY::TARGET.to_rcterm();
        let serialization_pred =
            vocab::rmlt::PROPERTY::SERIALIZATION.to_rcterm();
        let compression_pred = vocab::rmlt::PROPERTY::COMPRESSION.to_rcterm();

        let compression = get_object(graph, subject, &compression_pred)
            .ok()
            .map(|iri| Iri::new(iri.value()).unwrap());
        let serialization_term =
            get_object(graph, subject, &serialization_pred)
                .unwrap_or(vocab::formats::CLASS::NTRIPLES.to_rcterm());
        let serialization = Iri::new(serialization_term.value()).unwrap();

        let target = get_object(graph, subject, &target_pred).unwrap();
        let (output_type, config) =
            extract_output_target(&target, graph).unwrap();

        Ok(LogicalTarget {
            identifier: subject.value().to_string(),
            compression,
            serialization,
            output_type,
            config,
        })
    }
}
