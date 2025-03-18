use std::fmt::Display;

use sophia_api::graph::Graph;
use sophia_api::term::TTerm;
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;

use super::error::ParseError;
use super::RcTerm;

pub fn get_subject<TP, TO>(
    graph: &FastGraph,
    pred: &TP,
    obj: &TO,
) -> Result<RcTerm, ParseError>
where
    TP: TTerm + ?Sized + Display,
    TO: TTerm + ?Sized + Display,
{
    graph
        .triples_with_po(pred, obj)
        .next()
        .map(|trip_res| trip_res.map(|trip| trip.o().to_owned()).unwrap())
        .ok_or(ParseError::GenericError(format!(
            "Subject not found in graph with obj {} and pred {}",
            pred, obj
        )))
}


pub fn get_objects<TS, TP>(
    graph: &FastGraph,
    subject: &TS,
    pred: &TP,
) -> Vec<RcTerm>
where
    TS: TTerm + ?Sized + Display,
    TP: TTerm + ?Sized + Display,
{
    graph
        .triples_with_sp(subject, pred)
        .filter_map(|trip_res| trip_res.ok().map(|trip| trip.o().to_owned()))
        .collect()
}
pub fn get_object<TS, TP>(
    graph: &FastGraph,
    subject: &TS,
    pred: &TP,
) -> Result<RcTerm, ParseError>
where
    TS: TTerm + ?Sized + Display,
    TP: TTerm + ?Sized + Display,
{
    let mut objects = get_objects(graph, subject, pred);

    objects.pop().ok_or(ParseError::GenericError(format!(
        "Object not found in graph with subj {} and pred {}",
        subject, pred
    )))
}
