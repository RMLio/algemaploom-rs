use std::collections::{HashMap, HashSet};

use plangenerator::data_type::RcRefCellPlan;
use plangenerator::states::{Init, Processed};
use plangenerator::Plan;
use sophia_term::RcTerm;
use uuid::Uuid;

use super::source::AbstractLogicalSourceTranslator;
use super::OperatorTranslator;
use crate::new_rml::error::NewRMLTranslationResult;
use crate::new_rml::rml_model::v2::core::expression_map::term_map::{
    GraphMap, ObjectMap, PredicateMap, SubjectMap,
};
use crate::new_rml::rml_model::v2::core::{
    AbstractLogicalSource, AbstractLogicalSourceEnum, TriplesMap,
};
use crate::new_rml::rml_model::Document;

#[derive(Debug, Clone, Default)]
pub struct SearchStore<'a> {
    pub base_iri:               String,
    pub root_plan:              Option<Plan<Init>>,
    pub reference_attr_map:     HashMap<String, String>,
    pub termm_id_quad_var_map:  HashMap<RcTerm, String>,
    pub tm_id_join_map:         HashMap<RcTerm, HashSet<RcTerm>>,
    pub abs_ls_search_map:      HashMap<RcTerm, &'a AbstractLogicalSourceEnum>,
    pub ls_id_sourced_plan_map: HashMap<RcTerm, RcRefCellPlan<Processed>>,
    pub sm_search_map:          HashMap<RcTerm, &'a SubjectMap>,
    pub pm_search_map:          HashMap<RcTerm, &'a PredicateMap>,
    pub om_search_map:          HashMap<RcTerm, &'a ObjectMap>,
    pub gm_search_map:          HashMap<RcTerm, &'a GraphMap>,
    pub tm_search_map:          HashMap<RcTerm, &'a TriplesMap>,
}

impl SearchStore<'_> {
    /// Returns a vector containing pairs where the left value is the identifier of the
    /// [`AbstractLogicalSource`]
    /// and the right value is a vector of the associated
    /// [`TriplesMap`]'s
    /// identifiers [`SearchStore`].
    ///
    pub fn partition_lsid_tmid(&self) -> Vec<(RcTerm, Vec<RcTerm>)> {
        let mut result: HashMap<RcTerm, Vec<RcTerm>> = HashMap::new();

        for tm in self.tm_search_map.values() {
            let abs_ls_id = tm.abs_logical_source.get_identifier();
            let value = &tm.identifier;

            result
                .entry(abs_ls_id)
                .and_modify(|tms| tms.push(value.clone()))
                // RcTerm's cloning (low cost ref counter addition)
                .or_insert(vec![value.clone()]);
        }

        result.into_iter().collect()
    }

    pub fn add_reference_name(&mut self, reference: &str) -> &String {
        self.reference_attr_map
            .entry(reference.to_string())
            .or_insert_with(|| Uuid::new_v4().to_string())
    }

    pub fn from_document(
        document: &Document,
    ) -> NewRMLTranslationResult<SearchStore<'_>> {
        let mut tm_search_map = HashMap::new();
        let mut abs_ls_search_map = HashMap::new();
        let mut sm_search_map = HashMap::new();
        let mut pm_search_map = HashMap::new();
        let mut om_search_map = HashMap::new();
        let mut gm_search_map = HashMap::new();
        let mut termm_id_quad_var_map = HashMap::new();
        let mut tm_id_join_map = HashMap::new();

        for tm in document.triples_maps.iter() {
            let tm_count: u32 = 0;
            abs_ls_search_map.insert(
                tm.abs_logical_source.get_identifier(),
                &tm.abs_logical_source,
            );
            let tm_id = &tm.identifier;
            tm_search_map.insert(tm_id.clone(), tm);
            tm_id_join_map
                .insert(tm_id.clone(), tm.get_parent_triples_maps_ids());

            let sm = &tm.subject_map;
            let sm_ident = sm.term_map.identifier.clone();
            sm_search_map.insert(sm_ident.clone(), sm);

            termm_id_quad_var_map
                .insert(sm_ident.clone(), format!("sm_{}", tm_count));

            let sm_gms: Vec<_> = sm
                .graph_maps
                .iter()
                .map(|gm| (gm.term_map.identifier.clone(), gm))
                .collect();

            termm_id_quad_var_map.extend(sm_gms.iter().enumerate().map(
                |(gm_idx, (gm_ident, _))| {
                    (gm_ident.clone(), format!("sm_{}_gm_{}", tm_count, gm_idx))
                },
            ));

            gm_search_map.extend(sm_gms);

            for (pom_idx, pom) in tm.predicate_object_map_vec.iter().enumerate()
            {
                let pom_gms: Vec<_> = pom
                    .graph_map_vec
                    .iter()
                    .map(|gm| (gm.term_map.identifier.clone(), gm))
                    .collect();
                let pom_gms_var_iter = pom_gms.iter().enumerate().map(
                    |(gm_idx, (gm_ident, _))| {
                        (
                            gm_ident.clone(),
                            format!(
                                "pom_{}_{}_gm_{}",
                                tm_count, pom_idx, gm_idx
                            ),
                        )
                    },
                );

                termm_id_quad_var_map.extend(pom_gms_var_iter);
                gm_search_map.extend(pom_gms);

                pm_search_map = pom
                    .predicate_map_vec
                    .iter()
                    .map(|pm| (pm.term_map.identifier.clone(), pm))
                    .collect();

                let pm_var_iter = pom.predicate_map_vec.iter().enumerate().map(
                    |(pm_idx, pm)| {
                        (
                            pm.term_map.identifier.clone(),
                            format!(
                                "pom_{}_{}_pm_{}",
                                tm_count, pom_idx, pm_idx
                            ),
                        )
                    },
                );

                om_search_map = pom
                    .object_map_vec
                    .iter()
                    .map(|om| (om.term_map.identifier.clone(), om))
                    .collect();
                let om_var_iter = pom.object_map_vec.iter().enumerate().map(
                    |(om_idx, om)| {
                        (
                            om.term_map.identifier.clone(),
                            format!(
                                "pom_{}_{}_om_{}",
                                tm_count, pom_idx, om_idx
                            ),
                        )
                    },
                );

                let pm_om_id_var_chain = pm_var_iter.chain(om_var_iter);
                termm_id_quad_var_map.extend(pm_om_id_var_chain);
            }
        }

        let mut root_plan = Plan::new();
        let ls_id_sourced_plan_map =
            create_ls_id_sourced_plan_map(&mut root_plan, &abs_ls_search_map)?;

        Ok(SearchStore {
            base_iri: document
                .default_base_iri
                .clone()
                .unwrap_or("http://example.com/base/".to_string()),
            termm_id_quad_var_map,
            sm_search_map,
            pm_search_map,
            om_search_map,
            gm_search_map,
            tm_search_map,
            ls_id_sourced_plan_map,
            abs_ls_search_map,
            root_plan: Some(root_plan),
            reference_attr_map: HashMap::new(),
            tm_id_join_map: HashMap::new(),
        })
    }
}

fn create_ls_id_sourced_plan_map(
    plan: &mut Plan<Init>,
    abs_ls_search_map: &HashMap<RcTerm, &AbstractLogicalSourceEnum>,
) -> NewRMLTranslationResult<HashMap<RcTerm, RcRefCellPlan<Processed>>> {
    let mut ls_id_sourced_plan_map = HashMap::new();
    for abs_ls in abs_ls_search_map.values().copied() {
        let source = AbstractLogicalSourceTranslator::translate(abs_ls)?;
        let sourced_plan: RcRefCellPlan<Processed> = plan.source(source).into();

        ls_id_sourced_plan_map.insert(abs_ls.get_identifier(), sourced_plan);
    }
    Ok(ls_id_sourced_plan_map)
}
