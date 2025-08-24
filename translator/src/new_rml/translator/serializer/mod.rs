use std::collections::HashSet;
use std::marker::{PhantomData, PhantomPinned};

use operator::Serializer;
use vocab::ToString;

use super::store::SearchStore;
use super::OperatorTranslator;
use crate::new_rml::error::NewRMLTranslationResult;
use crate::new_rml::extractors::{stringify_term, FromVocab};
use crate::new_rml::rml_model::v2::core::expression_map::term_map::{
    CommonTermMapInfo, GraphMap,
};
use crate::new_rml::rml_model::v2::core::{PredicateObjectMap, TriplesMap};
use crate::new_rml::rml_model::v2::TermMapEnum;

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
        let mut graph_pattern: HashSet<String> = HashSet::new();
        for tm in tm_vec {
            let mut triples: Vec<String> = vec![];

            let sm_var = store
                .termm_id_quad_var_map
                .get(&tm.subject_map.as_ref().identifier)
                .map(|var| format_var(var))
                .unwrap();

            let sm = tm
                .subject_map
                .as_ref()
                .get_constant_value()
                .unwrap_or_else(|| sm_var.to_string());

            if tm.subject_map.is_subject_map() {
                let class_triples_iter =
                    tm.subject_map.unwrap_subject_map_ref().classes.iter().map(
                        |class_iri| {
                            format!(
                                "{} <{}> <{}>",
                                sm,
                                vocab::rdf::PROPERTY::TYPE.to_string(),
                                stringify_term(class_iri).unwrap()
                            )
                        },
                    );

                triples.extend(class_triples_iter);
            }

            let mut is_part_of_graph = false;
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

            if let Ok(sm) = tm.subject_map.try_unwrap_subject_map_ref() {
                add_graph_to_triple(
                    store,
                    &mut graph_pattern,
                    &triples,
                    &sm.graph_maps,
                );
                if !sm.graph_maps.is_empty() {
                    is_part_of_graph = true;
                }
            }

            if !is_part_of_graph {
                graph_pattern
                    .extend(triples.iter().map(|trip| format!("{} .", trip)));
            }
        }

        Ok(Serializer {
            template: graph_pattern.into_iter().collect::<Vec<_>>().join("\n"),
            options:  None,
            format:   operator::formats::DataFormat::NQuads,
        })
    }
}

fn add_graph_to_triple(
    store: &SearchStore,
    graph_pattern: &mut HashSet<String>,
    triples: &[String],
    graph_map_vec: &[TermMapEnum],
) {
    for gm_enum in graph_map_vec {
        let mut gm_part = store
            .termm_id_quad_var_map
            .get(&gm_enum.as_ref().identifier)
            .map(|var| format_var(var))
            .unwrap()
            .to_string();
        if let Ok(gm) = gm_enum.try_unwrap_graph_map_ref() {
            gm_part = gm.term_map_info.get_constant_value().unwrap_or(gm_part);
        }
        for triple in triples {
            graph_pattern.insert(format!("{} {} .", triple, gm_part));
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
        .map(|pm| get_var_or_constant(store, pm.as_ref()));

    let om_var_iter = pom
        .object_map_vec
        .iter()
        .map(|om| get_var_or_constant(store, om.as_ref()));

    pm_var_iter.flat_map(move |pm_var| {
        om_var_iter
            .clone()
            .map(move |om_var| (pm_var.clone(), om_var))
    })
}

pub fn get_var_or_constant(
    store: &SearchStore<'_>,
    term_map_info: &CommonTermMapInfo,
) -> String {
    let var = store
        .termm_id_quad_var_map
        .get(&term_map_info.identifier)
        .map(|var| format_var(var))
        .unwrap();

    term_map_info
        .get_constant_value()
        .unwrap_or_else(|| var.to_string())
}
