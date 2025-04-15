use std::collections::HashMap;
use std::marker::{PhantomData, PhantomPinned};

use operator::Serializer;

use super::store::SearchStore;
use super::OperatorTranslator;
use crate::new_rml::error::NewRMLTranslationResult;
use crate::new_rml::rml_model::v2::core::expression_map::term_map::GraphMap;
use crate::new_rml::rml_model::v2::core::{PredicateObjectMap, TriplesMap};

#[derive(Debug, Clone)]
pub struct SerializerOperatorTranslator<'a> {
    _phantom: PhantomData<&'a PhantomPinned>,
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
                .tm_id_quad_var_map
                .get(&tm.subject_map.term_map.identifier)
                .unwrap();

            let sm = tm
                .subject_map
                .term_map
                .get_constant_value()
                .unwrap_or_else(|| sm_var.to_string());

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

            if !is_part_of_graph {
                graph_pattern
                    .extend(triples.iter().map(|trip| format!("{}.", trip)));
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
    triples: &Vec<String>,
    graph_map_vec: &Vec<GraphMap>,
) {
    for gm in graph_map_vec {
        let gm_part = gm.term_map.get_constant_value().unwrap_or_else(|| {
            store
                .tm_id_quad_var_map
                .get(&gm.term_map.identifier)
                .unwrap()
                .to_string()
        });

        for triple in triples {
            graph_pattern.push(format!("{} {}.", triple, gm_part));
        }
    }
}

fn cproduct_pm_om_vars<'a>(
    store: &'a SearchStore,
    pom: &'a PredicateObjectMap,
) -> impl Iterator<Item = (String, String)> + 'a {
    let pm_var_iter = pom.predicate_map_vec.iter().map(|pm| {
        let pm_var = store
            .tm_id_quad_var_map
            .get(&pm.term_map.identifier)
            .unwrap();
        pm.term_map
            .get_constant_value()
            .unwrap_or_else(|| pm_var.to_string())
    });

    let om_var_iter = pom.object_map_vec.iter().map(|om| {
        let om_var = store
            .tm_id_quad_var_map
            .get(&om.term_map.identifier)
            .unwrap();
        om.term_map
            .get_constant_value()
            .unwrap_or_else(|| om_var.to_string())
    });

    pm_var_iter.flat_map(move |pm_var| {
        om_var_iter
            .clone()
            .map(move |om_var| (pm_var.clone(), om_var))
    })
}
