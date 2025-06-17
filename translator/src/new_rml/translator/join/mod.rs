use std::collections::HashMap;

use operator::{Extend, Operator, Rename, Serializer, Target};
use plan::states::join::join;

use super::store::SearchStore;
use super::OperatorTranslator;
use crate::new_rml::error::NewRMLTranslationResult;
use crate::new_rml::extractors::error::ParseError;
use crate::new_rml::rml_model::v2::core::expression_map::term_map::{
    GraphMap, PredicateMap, SubjectMap,
};
use crate::new_rml::rml_model::v2::core::{RefObjectMap, TriplesMap};
use crate::new_rml::rml_model::v2::AttributeAliaser;
use crate::new_rml::translator::error::TranslationError;
use crate::new_rml::translator::extend::extend_from_term_map;

#[derive(Debug, Clone)]
pub struct JoinTranslator {}

impl OperatorTranslator for JoinTranslator {
    type Input = TriplesMap;

    type Output = ();

    fn translate_with_store(
        store: &super::store::SearchStore,
        child_trip_map: &Self::Input,
    ) -> NewRMLTranslationResult<Self::Output> {
        let parent_tms_refoms =
            child_trip_map.get_parent_tms_pred_refom_pairs();
        let child_logical_source_id =
            child_trip_map.abs_logical_source.get_identifier();

        let child_plan = store
            .ls_id_sourced_plan_map
            .get(&child_logical_source_id)
            .ok_or(ParseError::GenericError(format!(
                "Search store cannot found the associated plan for the logical source id: {:?}",
                child_logical_source_id
            )))?;

        for (parent_tm_id, (pred_vec, ref_om, graph_vec)) in parent_tms_refoms {
            let parent_tm =
                store.tm_search_map.get(&parent_tm_id).unwrap_or_else(|| {
                    panic!(
                        "Given triples map id {:?} does not exist!",
                        parent_tm_id
                    )
                });
            let parent_logical_source_id =
                parent_tm.abs_logical_source.get_identifier();

            let parent_plan = store
                    .ls_id_sourced_plan_map
                    .get(&parent_logical_source_id)
                    .ok_or(ParseError::GenericError(format!(
                        "Search store cannot found the associated plan for the logical source id: {:?}",
                        child_logical_source_id
                    )))?;

            let alias = "join_alias";
            // Join the plans and progress the cursur
            let mut aliased_plan =
                join(child_plan.clone(), parent_plan.clone())?.alias("")?;

            let ptm_rename_op = Rename {
                alias:        Some(alias.to_string()),
                rename_pairs: HashMap::new(),
            };

            aliased_plan = aliased_plan.apply_to_right_fragment(
                Operator::RenameOp {
                    config: ptm_rename_op,
                },
                "RenameOp".into(),
                alias.into(),
            )?;

            let join_conditions = &ref_om.join_condition;
            let child_attributes = join_conditions
                .iter()
                .flat_map(|jc| jc.child.get_value())
                .collect();
            let parent_attributes = join_conditions
                .iter()
                .flat_map(|jc| jc.parent.get_value())
                .map(|val| format!("{}.{}", alias, val))
                .collect();

            let mut joined = aliased_plan
                .where_by(child_attributes)?
                .compared_to(parent_attributes)?;

            let extend_op = extend_op_from_join(
                &child_trip_map.subject_map,
                &ref_om,
                &pred_vec,
                &child_trip_map.base_iri,
                &graph_vec,
                alias,
                store,
            )?;

            let mut extended_plan = joined.apply(&extend_op, "ExtendOp")?;

            let serializer = Serializer {
                template: todo!(),
                options:  None,
                format:   todo!(),
            };

            extended_plan
                .serialize(serializer)?
                .sink(&Target::default())?;
        }
        Ok(())
    }
}

pub fn extend_op_from_join(
    child_subj_map: &SubjectMap,
    ref_objmap: &RefObjectMap,
    pred_vec: &[PredicateMap],
    child_base_iri: &str,
    graph_maps: &[GraphMap],
    alias: &str,
    store: &SearchStore,
) -> NewRMLTranslationResult<Operator> {
    let extension_func_subj =
        extend_from_term_map(store, child_base_iri, &child_subj_map.term_map)?;

    let extension_func_predicates_res: NewRMLTranslationResult<Vec<_>> =
        pred_vec
            .iter()
            .map(|pm| extend_from_term_map(store, child_base_iri, &pm.term_map))
            .collect();
    let extension_func_predicates = extension_func_predicates_res?;

    let extension_func_graphs_res: NewRMLTranslationResult<Vec<_>> = graph_maps
        .iter()
        .map(|gm| extend_from_term_map(store, child_base_iri, &gm.term_map))
        .collect();
    let extension_func_graphs = extension_func_graphs_res?;

    let ptm = store.tm_search_map.get(&ref_objmap.ptm_iri).ok_or(
        TranslationError::JoinError(
            "Reference object map's parent triples maps IRI cannot be found/searched"
                .to_string(),
        ),
    )?;
    let aliased_ptm_subj_term_map =
        ptm.subject_map.term_map.alias_attribute(alias);
    let extension_func_refoms_subj =
        extend_from_term_map(store, &ptm.base_iri, &aliased_ptm_subj_term_map)?;


    let mut extend_pairs = HashMap::new();
    extend_pairs.insert(extension_func_subj.0, extension_func_subj.1);
    extend_pairs
        .insert(extension_func_refoms_subj.0, extension_func_refoms_subj.1);

    extension_func_predicates
        .into_iter()
        .for_each(|(attr, func)| {
            extend_pairs.insert(attr, func);
        });

    extension_func_graphs.into_iter().for_each(|(attr, func)| {
        extend_pairs.insert(attr, func);
    });

    let extend = Extend { extend_pairs };
    Ok(Operator::ExtendOp { config: extend })
}
