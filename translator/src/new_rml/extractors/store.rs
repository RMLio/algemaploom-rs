use std::collections::HashSet;
use std::convert::Infallible;
use std::fmt::Debug;

use sophia_api::graph::{CollectibleGraph, Graph};
use sophia_api::prelude::Any;
use sophia_api::term::matcher::TermMatcher;
use sophia_api::term::{FromTerm, SimpleTerm, Term};
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;

use super::error::ParseError;
use super::RcTerm;

pub fn get_subgraph_subject<TS>(
    graph: &FastGraph,
    subj: TS,
) -> Result<FastGraph, ParseError>
where
    TS: Term + Debug,
{
    let mut result: Vec<_> = graph
        .triples_matching([subj], Any, Any)
        .filter_map(|trip| trip.ok())
        .map(|trip| (trip.s().clone(), trip.p().clone(), trip.o().clone()))
        .collect();

    let mut target_nodes_to_visit: Vec<_> = result
        .iter()
        .filter(|trip| {
            match trip.2.kind() {
                sophia_api::term::TermKind::Iri
                | sophia_api::term::TermKind::BlankNode => true,
                _ => false,
            }
        })
        .map(|trip| trip.2.clone())
        .collect();

    let mut visited: HashSet<SimpleTerm<'_>> = HashSet::new();

    while !target_nodes_to_visit.is_empty() {
        if let Some(target_node) = target_nodes_to_visit.pop() {
            if !visited.contains(&target_node) {
                visited.insert(target_node.clone());
                let sub_triples: Vec<_> = graph
                    .triples_matching([target_node], Any, Any)
                    .filter_map(|trip_res| trip_res.ok())
                    .map(|trip| {
                        (trip.s().clone(), trip.p().clone(), trip.o().clone())
                    })
                    .collect();

                let mut new_target_nodes =
                    sub_triples.iter().filter_map(|trip| {
                        match trip.2.kind() {
                            sophia_api::term::TermKind::Iri
                            | sophia_api::term::TermKind::BlankNode => {
                                Some(trip.2.clone())
                            }
                            _ => None,
                        }
                    });

                target_nodes_to_visit.extend(&mut new_target_nodes);
                result.extend(sub_triples);
            }
        }
    }

    Ok(FastGraph::from_triple_source(
        result
            .into_iter()
            .map(|trip| Ok::<_, Infallible>([trip.0, trip.1, trip.2])),
    )
    .unwrap())
}

pub fn get_subject<TP, TO>(
    graph: &FastGraph,
    pred: TP,
    obj: TO,
) -> Result<RcTerm, ParseError>
where
    TP: Term + Debug,
    TO: Term + Debug,
{
    graph
        .triples_matching(Any, [pred.borrow_term()], [obj.borrow_term()])
        .next()
        .map(|trip_res| trip_res.map(|trip| RcTerm::from_term(trip.o())).ok())
        .flatten()
        .ok_or(ParseError::GenericError(format!(
            "Subject not found in graph with obj {:?} and pred {:?}",
            pred, obj
        )))
}

pub fn get_objects<TS, TP>(
    graph_ref: &FastGraph,
    subject_ref: TS,
    pred: TP,
) -> Vec<RcTerm>
where
    TS: Term + Debug,
    TP: Term + Debug,
{
    graph_ref
        .triples_matching([subject_ref], [pred], Any)
        .filter_map(|trip_res| trip_res.ok().map(|trip| trip.o().into_term()))
        .collect()
}
pub fn get_object<TS, TP>(
    graph_ref: &FastGraph,
    subject_ref: TS,
    pred: TP,
) -> Result<RcTerm, ParseError>
where
    TS: Term + Debug,
    TP: Term + Debug,
{
    let mut objects = get_objects(graph_ref, subject_ref.borrow_term(), pred.borrow_term());

    objects.pop().ok_or(ParseError::GenericError(format!(
        "Object not found in graph with subj {:?} and pred {:?}",
        subject_ref, pred
    )))
}

pub fn get_objects_with_ps<TS, TP>(
    graph: &FastGraph,
    subject: TS,
    pred_vec: &[TP],
) -> Vec<RcTerm>
where
    TS: Term + Debug,
    TP: Term + Debug,
{
    pred_vec
        .iter()
        .flat_map(|pred| get_objects(graph, subject.borrow_term(), pred.borrow_term()))
        .collect()
}

pub fn get_object_with_ps<TS, TP>(
    graph: &FastGraph,
    subject: TS,
    pred_vec: &[TP],
) -> Result<RcTerm, ParseError>
where
    TS: Term + Debug,
    TP: Term + Debug,
{
    let mut object_opt = get_objects_with_ps(graph, subject.borrow_term(), pred_vec);
    object_opt.pop().ok_or(ParseError::GenericError(format!(
        "Object not found in graph with subj {:?} and preds {:?}",
        subject, pred_vec
    )))
}
