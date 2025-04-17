use sophia_api::graph::MutableGraph;
use sophia_api::prelude::Any;
use sophia_api::term::Term;
use sophia_inmem::graph::FastGraph;

use super::error::ParseError;
use super::store::{get_objects, get_subgraph_subject};
use super::{Extractor, ExtractorResult, FromVocab};
use crate::new_rml::extractors::store::get_object;
use crate::new_rml::rml_model::v2::io::target::{Target, TargetKind};

impl Extractor<Target> for Target {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<Target>
    where
        TTerm: Term,
    {
        let mode = get_object(
            graph_ref,
            subject_ref.borrow_term(),
            &vocab::rml_io::PROPERTY::MODE.to_rcterm(),
        )
        .ok();

        let compression = get_object(
            graph_ref,
            subject_ref.borrow_term(),
            &vocab::rml_io::PROPERTY::COMPRESSION.to_rcterm(),
        )
        .ok();

        let encoding = get_object(
            graph_ref,
            subject_ref.borrow_term(),
            &vocab::rml_io::PROPERTY::ENCODING.to_rcterm(),
        )
        .ok();

        let kind = extract_target_kind(subject_ref, graph_ref)?;

        Ok(Target {
            encoding,
            compression,
            mode,
            kind,
        })
    }
}

fn extract_target_kind<TTerm>(
    subject_ref: TTerm,
    graph_ref: &FastGraph,
) -> ExtractorResult<TargetKind>
where
    TTerm: Term,
{
    // FIXME: There can be multiple classes defined for a single target
    let source_type_new = vocab::rml_io::CLASS::SOURCE.to_rcterm();

    let target_class = get_objects(
        graph_ref,
        subject_ref.borrow_term(),
        &vocab::rdf::PROPERTY::TYPE.to_rcterm(),
    )
    .into_iter()
    .filter(|t| *t != source_type_new)
    .next()
    .ok_or(ParseError::GenericError(format!(
        "RML Target requires a type {:?}",
        subject_ref
    )))?;

    let mut metadata =
        get_subgraph_subject(graph_ref, subject_ref.borrow_term())?;

    metadata.remove_matching(
        [subject_ref.borrow_term()],
        [vocab::rml_io::PROPERTY::COMPRESSION.to_rcterm()],
        Any,
    );
    metadata.remove_matching(
        [subject_ref.borrow_term()],
        [vocab::rml_io::PROPERTY::ENCODING.to_rcterm()],
        Any,
    );

    metadata.remove_matching(
        [subject_ref.borrow_term()],
        [vocab::rml_io::PROPERTY::MODE.to_rcterm()],
        Any,
    );

    Ok(TargetKind {
        type_iri: target_class,
        metadata: metadata.into(),
    })
}
