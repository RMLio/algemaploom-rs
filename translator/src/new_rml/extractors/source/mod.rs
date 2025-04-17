//mod csvw_source;
//mod file_source;
//mod kafka_source;
//mod rdb_source;
//mod tcp_source;

use std::collections::HashMap;

use sophia_api::graph::MutableGraph;
use sophia_api::prelude::Any;
use sophia_api::term::{Term, TermKind};
use sophia_inmem::graph::FastGraph;

use super::error::ParseError;
use super::store::{get_objects, get_subgraph_subject};
use super::{stringify_rcterm, Extractor, ExtractorResult, RcTerm};
use crate::new_rml::extractors::store::get_object;
use crate::new_rml::extractors::FromVocab;
use crate::new_rml::rml_model::v2::io::source::{Source, SourceKind};

pub fn extract_parse_config(
    dialect_subject: &RcTerm,
    graph: &FastGraph,
    predicates: &Vec<(String, RcTerm)>,
) -> ExtractorResult<HashMap<String, String>> {
    let mut result = HashMap::new();
    let _ = predicates.iter().try_for_each(
        |(key, config_pred)| -> ExtractorResult<()> {
            // Retrieve the config value for the subject-predicate pair
            let config_val = get_object(graph, dialect_subject, config_pred);

            if let Ok(val) = config_val {
                result.insert(key.to_string(), stringify_rcterm(val).unwrap());
            }

            Ok(())
        },
    );

    Ok(result)
}
impl Extractor<Source> for Source {
    fn extract_self<TS>(
        subject_ref: TS,
        graph_ref: &FastGraph,
    ) -> ExtractorResult<Source>
    where
        TS: Term,
    {
        if subject_ref.kind() == TermKind::Literal
            || subject_ref.kind() == TermKind::Variable
        {
            return Err(
                ParseError::GenericError(
                    "Subject term for source extraction cannot be a literal nor a variable"
                    .to_string()).into());
        }

        let encoding_pred = vocab::rml_io::PROPERTY::ENCODING.to_rcterm();
        let encoding =
            get_object(graph_ref, subject_ref.borrow_term(), &encoding_pred)
                .ok();

        let compression_pred = vocab::rml_io::PROPERTY::COMPRESSION.to_rcterm();
        let compression =
            get_object(graph_ref, subject_ref.borrow_term(), &compression_pred)
                .ok();

        let nullable_pred = vocab::rml_io::PROPERTY::NULL.to_rcterm();
        let nullable_vec =
            get_objects(graph_ref, subject_ref.borrow_term(), &nullable_pred)
                .into_iter()
                .map(|lit| lit.lexical_form().map(|i| i.to_string()))
                .flatten()
                .collect();

        let kind = extract_typed_source(subject_ref, graph_ref)?;

        Ok(Source {
            encoding,
            compression,
            nullable_vec,
            kind,
        })
    }
}

fn extract_typed_source<TS>(
    subject: TS,
    graph: &FastGraph,
) -> ExtractorResult<SourceKind>
where
    TS: Term,
{
    let type_pred = vocab::rdf::PROPERTY::TYPE.to_rcterm();
    // FIXME: There can be multiple classes defined for a single source
    let source_type = get_object(graph, subject.borrow_term(), &type_pred)?;

    let source_type_iri = match source_type {
        RcTerm::Iri(iri) => Ok(iri),
        _ => {
            Err(ParseError::GenericError(
                "Object of predicate 'a' cannot be Literal".to_string(),
            ))
        }
    }?;

    let mut metadata: FastGraph =
        get_subgraph_subject(graph, subject.borrow_term())?;

    // Remove the properties that have been saved previously in Source struct
    metadata.remove_matching(
        [subject.borrow_term()],
        [vocab::rml_io::PROPERTY::ENCODING.to_rcterm()],
        Any,
    );
    metadata.remove_matching(
        [subject.borrow_term()],
        [vocab::rml_io::PROPERTY::COMPRESSION.to_rcterm()],
        Any,
    );
    metadata.remove_matching(
        [subject.borrow_term()],
        [vocab::rml_io::PROPERTY::NULL.to_rcterm()],
        Any,
    );

    Ok(SourceKind {
        type_iri: source_type_iri.into(),
        metadata: metadata.into(),
    })
}

//fn old_config_extract(
//    subject: &RcTerm,
//    graph: &FastGraph,
//    match_result: RcTerm,
//) -> Result<HashMap<String, String>, ParseError> {
//    let config = match match_result {
//        iri_string if iri_string == vocab::csvw::CLASS::TABLE.to_rcterm() => {
//            csvw_source::extract_csvw_source(subject, graph)
//        }
//
//        iri_string
//            if iri_string == vocab::d2rq::CLASS::DATABASE.to_rcterm() =>
//        {
//            rdb_source::extract_rdb_source(subject, graph)
//        }
//        iri_string
//            if iri_string == vocab::rmls::CLASS::KAFKASTREAM.to_rcterm() =>
//        {
//            kafka_source::extract_kafka_source(subject, graph)
//        }
//
//        iri_string
//            if iri_string
//                == vocab::rmls::CLASS::TCPSOCKETSTREAM.to_rcterm() =>
//        {
//            tcp_source::extract_tcp_source(subject, graph)
//        }
//
//        invalid_iri => {
//            Err(ParseError::GenericError(format!(
//                "Source type extraction not yet supported {:#?}",
//                invalid_iri
//            )))
//        }
//    }?;
//    Ok(config)
//}
