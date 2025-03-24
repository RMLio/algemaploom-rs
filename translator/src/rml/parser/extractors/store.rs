use std::fmt::Debug;

use sophia_api::graph::Graph;
use sophia_api::prelude::Any;
use sophia_api::term::{FromTerm, Term};
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;

use super::error::ParseError;
use super::RcTerm;

pub fn get_subject<TP, TO>(
    graph: &FastGraph,
    pred: TP,
    obj: TO,
) -> Result<RcTerm, ParseError>
where
    TP: Term + Debug + Copy,
    TO: Term + Debug + Copy,
{
    graph
        .triples_matching(Any, [pred], [obj])
        .filter_map(|res| res.ok())
        .next()
        .map(|trip| RcTerm::from_term(trip.o()))
        .ok_or(ParseError::GenericError(format!(
            "Subject not found in graph with obj {:?} and pred {:?}",
            pred, obj
        )))
}

pub fn get_objects<TS, TP>(
    graph: &FastGraph,
    subject: TS,
    pred: TP,
) -> Vec<RcTerm>
where
    TS: Term + Debug + Copy,
    TP: Term + Debug + Copy,
{
    graph
        .triples_matching([subject], [pred], Any)
        .filter_map(|trip_res| {
            trip_res.ok().map(|trip| RcTerm::from_term(trip.o()))
        })
        .collect()
}

pub fn get_object<TS, TP>(
    graph: &FastGraph,
    subject: TS,
    pred: TP,
) -> Result<RcTerm, ParseError>
where
    TS: Term + Debug + Copy,
    TP: Term + Debug + Copy,
{
    let mut objects = get_objects(graph, subject, pred);

    objects.pop().ok_or(ParseError::GenericError(format!(
        "Object not found in graph with subj {:?} and pred {:?}",
        subject, pred
    )))
}
