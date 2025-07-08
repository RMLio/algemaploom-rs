use std::collections::HashMap;

use operator::IOType;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use super::{rcterm_to_string, Extractor, ExtractorResult};
use crate::rml::parser::extractors::store::get_object;
use crate::rml::parser::extractors::FromVocab;
use crate::rml::parser::rml_model::source_target::{LogicalTarget, LdesInformation};

fn extract_output_target(
    target_subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<(IOType, HashMap<String, String>)> {
    if let Ok(output_path_iri) = get_object(
        graph,
        target_subject,
        &vocab::void::PROPERTY::DATA_DUMP.to_rcterm(),
    ) {
        let path = rcterm_to_string(&output_path_iri);

        return Ok((IOType::File, HashMap::from([("path".to_string(), path)])));
    }

    if let Ok(sparql_endpoint_iri) = get_object(
        graph,
        target_subject,
        &vocab::void::PROPERTY::SPARQL_ENDPOINT.to_rcterm(),
    ) {
        let sparql_path = rcterm_to_string(&sparql_endpoint_iri);

        return Ok((
            IOType::SPARQLEndpoint,
            HashMap::from([("sparql_uri".to_string(), sparql_path)]),
        ));
    }

    Err(super::error::ParseError::GenericError(format!(
        "Void dataset extraction failed for {:?}",
        target_subject
    )).into())
}

impl Extractor<LogicalTarget> for LogicalTarget {
    fn extract_self(
        subject: &RcTerm,
        graph: &FastGraph,
    ) -> ExtractorResult<LogicalTarget> {
        let target_pred = vocab::rmlt::PROPERTY::TARGET.to_rcterm();
        let serialization_pred =
            vocab::rmlt::PROPERTY::SERIALIZATION.to_rcterm();
        let compression_pred = vocab::rmlt::PROPERTY::COMPRESSION.to_rcterm();

        let compression = get_object(graph, subject, &compression_pred).ok();
        let serialization = get_object(graph, subject, &serialization_pred)
            .unwrap_or(vocab::formats::CLASS::NTRIPLES.to_rcterm());

        let target = get_object(graph, subject, &target_pred)?;
        let (output_type, config) =
            extract_output_target(&target, graph)?;

        // Extract LDES information if this is an LDES target
        let ldes = if let Ok(_) = get_object(graph, subject, &vocab::rmlt::PROPERTY::LDES.to_rcterm()) {
            Some(LdesInformation::extract_self(subject, graph)?)
        } else {
            None
        };

        Ok(LogicalTarget {
            identifier: rcterm_to_string(subject),
            compression,
            serialization,
            ldes,
            output_type,
            config,
        })
    }
}



#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    use sophia_api::graph::Graph;
    use sophia_api::prelude::Any;
    use sophia_api::term::FromTerm;
    use sophia_api::triple::Triple;
    use sophia_term::RcTerm;

    use crate::rml::parser::extractors::io::load_graph_bread;
    use crate::rml::parser::extractors::{
        ExtractorResult, FromVocab, TermMapExtractor,
    };
    use crate::rml::parser::rml_model::term_map::{SubjectMap, TermMapType};
    use crate::{load_graph, test_case};

    #[test]
    fn extract_logical_target_ldes() -> ExtractorResult<()> {
        use crate::rml::parser::rml_model::source_target::LogicalTarget;
        use super::Extractor;
        
        let graph = load_graph!("rmlmapper-custom/rml-ldes/bluebike/base.rml.ttl")?;
        
        let ldes_target_ref = RcTerm::from_term(
            sophia_api::prelude::Iri::new_unchecked("http://example.com/rules/#LDESLogicalTarget")
        );
        
        let logical_target = LogicalTarget::extract_self(&ldes_target_ref, &graph)?;
        
        assert!(logical_target.ldes.is_some());
        let ldes_info = logical_target.ldes.as_ref().unwrap();
        
        let expected_base_iri = RcTerm::from_term(
            sophia_api::prelude::Iri::new_unchecked("https://blue-bike.be/ldes.ttl")
        );
        println!("ldes_info: {:?}", ldes_info);
        assert_eq!(ldes_info.ldes_base_iri, expected_base_iri);
        assert_eq!(ldes_info.ldes_generate_immutable_iri, false);
        
        assert!(ldes_info.ldes_eventstream.contains_key("timestampPath"));
        assert_eq!(
            ldes_info.ldes_eventstream.get("timestampPath").unwrap(),
            "http://purl.org/dc/terms/created"
        );
        
        assert!(ldes_info.ldes_eventstream.contains_key("versionOfPath"));
        assert_eq!(
            ldes_info.ldes_eventstream.get("versionOfPath").unwrap(),
            "http://purl.org/dc/terms/isVersionOf"
        );
        
        assert!(ldes_info.ldes_eventstream.contains_key("treeShape"));
        assert_eq!(
            ldes_info.ldes_eventstream.get("treeShape").unwrap(),
            "https://blue-bike.be/shape.ttl"
        );

        Ok(())
    }
}

