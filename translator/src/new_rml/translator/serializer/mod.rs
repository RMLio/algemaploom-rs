use std::marker::{PhantomData, PhantomPinned};

use operator::Serializer;
use vocab::ToString;

use super::store::SearchStore;
use super::OperatorTranslator;
use crate::new_rml::error::NewRMLTranslationResult;
use crate::new_rml::extractors::{stringify_rcterm, FromVocab};
use crate::new_rml::rml_model::v2::core::expression_map::term_map::{
    GraphMap, CommonTermMapInfo,
};
use crate::new_rml::rml_model::v2::core::{PredicateObjectMap, TriplesMap};

#[derive(Debug, Clone)]
pub struct SerializerOperatorTranslator<'a> {
    _phantom: PhantomData<&'a PhantomPinned>,
}

fn format_var(var: &str) -> String {
    format!("?{}", var)
}

impl<'a> OperatorTranslator for SerializerOperatorTranslator<'a> {
    type Input = Vec<&'a TriplesMap>;

    type Output = Serializer;

    fn translate_with_store(
        store: &SearchStore,
        tm_vec: &Self::Input,
    ) -> NewRMLTranslationResult<Self::Output> {
        let mut graph_pattern: Vec<String> = vec![];
        for tm in tm_vec {
            let mut triples: Vec<String> = vec![];

            let sm_var = store
                .termm_id_quad_var_map
                .get(&tm.subject_map.term_map.identifier)
                .map(|var| format_var(var))
                .unwrap();

            let sm = tm
                .subject_map
                .term_map
                .get_constant_value()
                .unwrap_or_else(|| sm_var.to_string());

            let class_triples_iter =
                tm.subject_map.classes.iter().map(|class_iri| {
                    format!(
                        "{} <{}> <{}>",
                        sm,
                        vocab::rdf::PROPERTY::TYPE.to_string(),
                        stringify_rcterm(class_iri).unwrap()
                    )
                });

            triples.extend(class_triples_iter);

            let mut is_part_of_graph = !tm.subject_map.graph_maps.is_empty();
            for pom in &tm.predicate_object_map_vec {
                // TODO: Handles reference object maps too <15-04-25, Min Oo> //

                let cproduct_pm_om_vars = cproduct_pm_om_vars(store, pom);

                for (pm_var, om_var) in cproduct_pm_om_vars {
                    triples.push(format!("{} {} {}", sm, pm_var, om_var));
                }

                if !pom.graph_map_vec.is_empty() {
                    is_part_of_graph = true
                }

                add_graph_to_triple(
                    store,
                    &mut graph_pattern,
                    &triples,
                    &pom.graph_map_vec,
                );
            }

            if !is_part_of_graph {
                graph_pattern
                    .extend(triples.iter().map(|trip| format!("{} .", trip)));
            }

            add_graph_to_triple(
                store,
                &mut graph_pattern,
                &triples,
                &tm.subject_map.graph_maps,
            );
        }

        Ok(Serializer {
            template: graph_pattern.join("\n"),
            options:  None,
            format:   operator::formats::DataFormat::NQuads,
        })
    }
}

fn add_graph_to_triple(
    store: &SearchStore,
    graph_pattern: &mut Vec<String>,
    triples: &[String],
    graph_map_vec: &[GraphMap],
) {
    for gm in graph_map_vec.iter().filter(|gm| !gm.is_default_graph()) {
        let gm_part = gm.term_map.get_constant_value().unwrap_or_else(|| {
            store
                .termm_id_quad_var_map
                .get(&gm.term_map.identifier)
                .map(|var| format_var(var))
                .unwrap()
                .to_string()
        });

        for triple in triples {
            graph_pattern.push(format!("{} {} .", triple, gm_part));
        }
    }
}

fn cproduct_pm_om_vars<'a>(
    store: &'a SearchStore,
    pom: &'a PredicateObjectMap,
) -> impl Iterator<Item = (String, String)> + 'a {
    let pm_var_iter = pom
        .predicate_map_vec
        .iter()
        .map(|pm| get_var_or_constant(store, &pm.term_map));

    let om_var_iter = pom
        .object_map_vec
        .iter()
        .map(|om| get_var_or_constant(store, &om.term_map));

    pm_var_iter.flat_map(move |pm_var| {
        om_var_iter
            .clone()
            .map(move |om_var| (pm_var.clone(), om_var))
    })
}

pub fn get_var_or_constant(
    store: &SearchStore<'_>,
    term_map: &CommonTermMapInfo,
) -> String {
    let var = store
        .termm_id_quad_var_map
        .get(&term_map.identifier)
        .map(|var| format_var(var))
        .unwrap();

    term_map
        .get_constant_value()
        .unwrap_or_else(|| var.to_string())
}
