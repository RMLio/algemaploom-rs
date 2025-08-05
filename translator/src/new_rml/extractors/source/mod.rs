use sophia_api::graph::MutableGraph;
use sophia_api::prelude::Any;
use sophia_api::term::{FromTerm, Term, TermKind};
use sophia_inmem::graph::FastGraph;

use super::error::ParseError;
use super::store::{get_objects, get_subgraph_subject};
use super::{Extractor, ExtractorResult, RcTerm};
use crate::new_rml::extractors::store::get_object;
use crate::new_rml::extractors::FromVocab;
use crate::new_rml::rml_model::v2::io::source::{Source, SourceKind};

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
                .filter_map(|lit| lit.lexical_form().map(|i| i.to_string()))
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
    let source_pred = vocab::rml_io::CLASS::SOURCE.to_rcterm();
    // FIXME: There can be multiple classes defined for a single source
    let source_type = get_objects(graph, subject.borrow_term(), &type_pred)
        .into_iter()
        .find(|obj_term| *obj_term != source_pred)
        .unwrap();

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
    let _ = metadata.remove_matching(
        [subject.borrow_term()],
        [vocab::rml_io::PROPERTY::ENCODING.to_rcterm()],
        Any,
    );
    let _ = metadata.remove_matching(
        [subject.borrow_term()],
        [vocab::rml_io::PROPERTY::COMPRESSION.to_rcterm()],
        Any,
    );
    let _ = metadata.remove_matching(
        [subject.borrow_term()],
        [vocab::rml_io::PROPERTY::NULL.to_rcterm()],
        Any,
    );

    Ok(SourceKind {
        subj_iri: RcTerm::from_term(subject),
        type_iri: source_type_iri.into(),
        metadata: metadata.into(),
    })
}

