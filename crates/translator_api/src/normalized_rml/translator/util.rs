use anyhow::Result;
use oxigraph::model::{
    LiteralRef, NamedNodeRef, Quad, Subject, SubjectRef, Term, TermRef,
};
use oxigraph::store::Store;

use crate::normalized_rml::error::oxigraph::{OxigraphError, OxigraphErrorKind};
use crate::normalized_rml::FromVocab;

pub fn termref_to_literal(term_ref: TermRef) -> Result<LiteralRef> {
    match term_ref {
        TermRef::Literal(literal_ref) => Ok(literal_ref),
        _ => {
            Err(OxigraphError {
                kind: OxigraphErrorKind::GenericError(format!(
                    "error while converting term {} to literal",
                    term_ref
                )),
            }
            .into())
        }
    }
}

pub fn termref_to_subjref(term_ref: TermRef) -> Result<SubjectRef> {
    match term_ref {
        TermRef::NamedNode(named_node_ref) => Ok(named_node_ref.into()),
        TermRef::BlankNode(blank_node_ref) => Ok(blank_node_ref.into()),
        TermRef::Triple(triple) => Ok(triple.into()),
        _ => {
            Err(OxigraphError {
                kind: OxigraphErrorKind::GenericError(format!(
                    "error while converting term {} to subject",
                    term_ref
                )),
            }
            .into())
        }
    }
}

pub fn get_object(
    subject: SubjectRef,
    predicate: NamedNodeRef,
    store: &Store,
) -> Result<Term> {
    get_quads(subject, predicate, store)
        .map(|q| q.object)
        .next()
        .ok_or(
            OxigraphErrorKind::GenericError(format!(
                "cannot find the object node for \
                    triples with subject {} and predicate {}",
                subject, predicate
            ))
            .into(),
        )
}

pub fn get_quads(
    subject: SubjectRef,
    predicate: NamedNodeRef,
    store: &Store,
) -> impl Iterator<Item = Quad> {
    store
        .quads_for_pattern(Some(subject), Some(predicate), None, None)
        .filter_map(|res| res.ok())
}

pub fn rooted_subgraph(
    target_node: SubjectRef,
    store: &Store,
) -> Result<Vec<Quad>> {
    let mut result: Vec<_> = vec![];

    //Case 1 of Definition 15
    let mut to_visit: Vec<_> = store
        .quads_for_pattern(Some(target_node), None, None, None)
        .filter_map(|quad| quad.ok())
        .collect();

    // Case 2 of Definition 15
    while let Some(quad) = to_visit.pop() {
        result.push(quad.clone());
        if quad.predicate
            != vocab::r2rml::PROPERTY::PARENTTRIPLESMAP.to_named_node()
        {
            match quad.object {
                x if x.is_named_node() || x.is_blank_node() => {
                    let target_subj: Subject = x.try_into()?;
                    let new_quads = store
                        .quads_for_pattern(
                            Some(target_subj.as_ref()),
                            None,
                            None,
                            None,
                        )
                        .filter_map(|trip_res| trip_res.ok());
                    to_visit.extend(new_quads);
                }
                _ => continue,
            }
        }
    }
    Ok(result)
}
