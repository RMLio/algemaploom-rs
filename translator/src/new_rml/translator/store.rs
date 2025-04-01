use std::collections::HashMap;

use sophia_term::RcTerm;
use uuid::Uuid;

use crate::new_rml::rml_model::v2::core::expression_map::term_map::{
    GraphMap, ObjectMap, PredicateMap, SubjectMap,
};
use crate::new_rml::rml_model::v2::core::TriplesMap;
use crate::new_rml::rml_model::Document;

#[derive(Debug, Clone, Default)]
pub struct SearchStore<'a> {
    reference_attr_map: HashMap<String, String>,
    tm_id_quad_var_map: HashMap<RcTerm, String>,
    sm_search_map:      HashMap<RcTerm, &'a SubjectMap>,
    pm_search_map:      HashMap<RcTerm, &'a PredicateMap>,
    om_search_map:      HashMap<RcTerm, &'a ObjectMap>,
    gm_search_map:      HashMap<RcTerm, &'a GraphMap>,
    tm_search_map:      HashMap<RcTerm, &'a TriplesMap>,
}

impl SearchStore<'_> {
    pub fn add_reference_name(&mut self, reference: &str) -> &String {
        self.reference_attr_map
            .entry(reference.to_string())
            .or_insert_with(|| Uuid::new_v4().to_string())
    }

    pub fn from_document(document: &Document) -> SearchStore<'_> {
        let tm_iter = document.triples_maps.iter();

        let mut tm_search_map = HashMap::new();
        let mut sm_search_map = HashMap::new();
        let mut pm_search_map = HashMap::new();
        let mut om_search_map = HashMap::new();
        let mut gm_search_map = HashMap::new();
        let mut tm_id_quad_var_map = HashMap::new();

        for tm in tm_iter {
            let tm_count: u32 = 0;
            tm_search_map.insert(tm.identifier.clone(), tm);
            let sm = &tm.subject_map;
            let sm_ident = sm.term_map.identifier.clone();
            sm_search_map.insert(sm_ident.clone(), sm);

            tm_id_quad_var_map
                .insert(sm_ident.clone(), format!("sm_{}", tm_count));

            let sm_gms: Vec<_> = sm
                .graph_maps
                .iter()
                .map(|gm| (gm.term_map.identifier.clone(), gm))
                .collect();

            tm_id_quad_var_map.extend(sm_gms.iter().enumerate().map(
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

                tm_id_quad_var_map.extend(pom_gms_var_iter);
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
                tm_id_quad_var_map.extend(pm_om_id_var_chain);
            }
        }

        SearchStore {
            tm_id_quad_var_map,
            sm_search_map,
            pm_search_map,
            om_search_map,
            gm_search_map,
            tm_search_map,
            ..Default::default()
        }
    }

    pub fn get_reference_attr(&self, reference: &str) -> Option<&String> {
        self.reference_attr_map.get(reference)
    }

    pub fn get_tm_variable(&self, identifier: &RcTerm) -> Option<&String> {
        self.tm_id_quad_var_map.get(identifier)
    }

    pub fn get_sm(&self, identifier: &RcTerm) -> Option<&SubjectMap> {
        self.sm_search_map.get(identifier).copied()
    }

    pub fn get_om(&self, identifier: &RcTerm) -> Option<&ObjectMap> {
        self.om_search_map.get(identifier).copied()
    }

    pub fn get_pm(&self, identifier: &RcTerm) -> Option<&PredicateMap> {
        self.pm_search_map.get(identifier).copied()
    }

    pub fn get_gm(&self, identifier: &RcTerm) -> Option<&GraphMap> {
        self.gm_search_map.get(identifier).copied()
    }
}
