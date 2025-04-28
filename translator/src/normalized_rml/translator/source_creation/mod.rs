mod util;

use std::collections::{HashMap, HashSet};

use anyhow::Result;
use operator::formats::ReferenceFormulation;
use operator::{Field, Source};
use oxigraph::model::{Quad, Subject, SubjectRef, Term, TermRef};
use oxigraph::store::Store;
use util::get_queries_from_template;

use super::data::QueryAttrMap;
use super::util::{get_quads, termref_to_literal};
use crate::normalized_rml::error::oxigraph::{
    OxigraphError, OxigraphErrorKind,
};
use crate::normalized_rml::translator::util::{
    get_object, rooted_subgraph, termref_to_subjref,
};
use crate::normalized_rml::FromVocab;

pub fn create_source_operator(
    triples_map_iri: SubjectRef,
    store: &Store,
) -> Result<(Source, QueryAttrMap)> {
    let query_to_attr_map = extract_queries(triples_map_iri, store)?;

    let ls_object = get_object(
        triples_map_iri,
        vocab::rml::PROPERTY::LOGICALSOURCE.to_named_node().as_ref(),
        store,
    )?;
    let logical_source_iri = termref_to_subjref(ls_object.as_ref())?;
    let source = get_object(
        logical_source_iri,
        vocab::rml::PROPERTY::SOURCE.to_named_node().as_ref(),
        store,
    )
    .iter()
    .filter_map(|term| termref_to_literal(term.as_ref()).ok())
    .map(|lit| lit.value().to_string())
    .next();

    let reference_formulation = match source {
        Some(_) => {
            let ref_form_node = get_object(
                logical_source_iri,
                vocab::rml::PROPERTY::REFERENCEFORMULATION
                    .to_named_node()
                    .as_ref(),
                store,
            )?;
            get_reference_formulation_from_term(ref_form_node.as_ref())?
        }
        None => ReferenceFormulation::CSVRows,
    };

    let mut reference = None;
    if reference_formulation != ReferenceFormulation::CSVRows {
        reference = get_object(
            logical_source_iri,
            vocab::rml::PROPERTY::ITERATOR.to_named_node().as_ref(),
            store,
        )
        .ok()
        .map(|t| t.to_string())
        .and_then(|t| {
            t.strip_suffix("\"")
                .and_then(|t| t.strip_prefix("\""))
                .map(|t| t.to_string())
        });
    }

    let fields =
        create_fields_from_map(&query_to_attr_map, &reference_formulation);

    Ok((
        Source {
            config:        get_source_config(logical_source_iri, store)?,
            source_type:   operator::IOType::File,
            root_iterator: operator::Iterator {
                reference,
                reference_formulation,
                fields,
                alias: None,
            },
        },
        query_to_attr_map,
    ))
}

fn get_reference_formulation_from_term(
    term: TermRef,
) -> Result<ReferenceFormulation> {
    let ref_form_iri = match term {
        TermRef::NamedNode(named_node) => Ok(named_node),
        _ => {
            Err(OxigraphErrorKind::GenericError(format!(
                "the given term is not a valid reference formulation iri {}",
                term
            )))
        }
    }?;

    match ref_form_iri {
        val if val == vocab::query::CLASS::CSV.to_named_node() => {
            Ok(ReferenceFormulation::CSVRows)
        }
        val if val == vocab::query::CLASS::JSONPATH.to_named_node() => {
            Ok(ReferenceFormulation::JSONPath)
        }
        val if val == vocab::query::CLASS::XPATH.to_named_node() => {
            Ok(ReferenceFormulation::XMLPath)
        }
        val => {
            Err(OxigraphErrorKind::GenericError(format!(
                "the reference formulation IRI is not supported {}",
                val
            ))
            .into())
        }
    }
}

fn create_fields_from_map(
    query_to_attr_map: &QueryAttrMap,
    reference_formulation: &ReferenceFormulation,
) -> Vec<Field> {
    query_to_attr_map
        .iter()
        .map(|(query, attr)| {
            Field {
                alias:                 attr.to_string(),
                constant:              None,
                iterator:              None,
                reference:             Some(query.to_string()),
                reference_formulation: reference_formulation.clone(),
                inner_fields:          vec![],
            }
        })
        .collect()
}

/// Algorithm 2
/// Given an RDF graph of an RML document and an RDF term node of a RML Triples Map.
/// Extract queries from the rooted-subgraph of the given RML Triples Map node.
///
/// Returns a hash map which maps the queries to their associated attributes
///
/// # Errors
///
/// This function will return an error if no queries can be extracted for the
/// triples map.
fn extract_queries(
    triples_map_iri: SubjectRef,
    store: &Store,
) -> Result<QueryAttrMap> {
    let mut query_to_attr_map = HashMap::new();
    let mut queries = HashSet::new();
    let tm_subgraph = rooted_subgraph(triples_map_iri, store)?;

    for reference_quad in tm_subgraph.iter().filter(|trip| {
        trip.predicate == vocab::rml::PROPERTY::REFERENCE.to_named_node()
    }) {
        let query = termref_to_literal(reference_quad.object.as_ref())?
            .value()
            .to_string();
        queries.insert(query);
    }

    for template_quad in tm_subgraph.iter().filter(|trip| {
        trip.predicate == vocab::r2rml::PROPERTY::TEMPLATE.to_named_node()
    }) {
        let template_str =
            termref_to_literal(template_quad.object.as_ref())?.value();
        let template_queries = get_queries_from_template(template_str);

        queries.extend(template_queries.into_iter());
    }

    let child_quads = tm_subgraph.into_iter().filter(|trip| {
        trip.predicate == vocab::r2rml::PROPERTY::CHILD.to_named_node()
    });
    let mut parent_quads = get_parent_quads(triples_map_iri, store);

    parent_quads.extend(child_quads);

    for quad in parent_quads {
        let query = termref_to_literal(quad.object.as_ref())?
            .value()
            .to_string();
        queries.insert(query);
    }

    for query in queries {
        let attribute = uuid::Uuid::new_v4().to_string();
        query_to_attr_map.insert(query, attribute);
    }

    Ok(query_to_attr_map)
}

///
/// Returns triples (quads) that is generated according to the following set
/// builder notations
/// { (s,p,o) ∈ G | p is the IRI rr:parent, o ∈ L and there is an s' ∈ I ∪ B s.t.
///                 (s', rr:joinCondition, s) ∈ G and (s', rr:parentTriplesMap, u_tm) ∈ G }
/// with u_tm being the given triples_map_iri.
///
/// # Panics
///
/// Panics if .
fn get_parent_quads(triples_map_iri: SubjectRef, store: &Store) -> Vec<Quad> {
    // s' ∈ I ∪ B where (s', rr:parentTriplesMap, u_tm) ∈ G
    let referencing_object_map_iris: HashSet<Subject> = store
        .quads_for_pattern(
            None,
            Some(
                vocab::r2rml::PROPERTY::PARENTTRIPLESMAP
                    .to_named_node()
                    .as_ref(),
            ),
            Some(triples_map_iri.into()),
            None,
        )
        .filter_map(|res| res.ok())
        .map(|q| q.subject)
        .collect();

    referencing_object_map_iris
        .iter()
        .flat_map(|subject| {
            // s' ∈ I ∪ B where (s', rr:joinCondition, s) ∈ G
            get_quads(
                subject.as_ref(),
                vocab::r2rml::PROPERTY::JOINCONDITION
                    .to_named_node()
                    .as_ref(),
                store,
            )
        })
        .map(|q| q.object)
        .flat_map(|o| {
            // (s, rr:parent, o) ∈ G
            get_quads(
                termref_to_subjref(o.as_ref()).unwrap(),
                vocab::r2rml::PROPERTY::PARENT.to_named_node().as_ref(),
                store,
            )
        })
        .collect()
}

// Only handles CSV/JSON files for now
fn get_source_config(
    logical_source_iri: SubjectRef,
    store: &Store,
) -> Result<HashMap<String, String>> {
    let mut result = HashMap::new();
    let source_node = get_object(
        logical_source_iri,
        vocab::rml::PROPERTY::SOURCE.to_named_node().as_ref(),
        store,
    )?;

    let path_res: Result<String> = match source_node {
        Term::Literal(literal) => Ok(literal.value().to_string()),
        _ => Err(OxigraphError::new(OxigraphErrorKind::GenericError(format!(
            "error parsing rml:source {}.\n currently only supports CSV/JSON files",
            source_node
        )))
        .into()),
    };
    result.insert("path".to_string(), path_res?);
    Ok(result)
}
