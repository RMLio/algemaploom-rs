use std::collections::HashSet;

use operator::{Operator, Projection};
use rml_interpreter::rml_model::term_map::SubjectMap;
use rml_interpreter::rml_model::{PredicateObjectMap, TriplesMap};

use crate::rmlalgebra::types::SearchMap;
use crate::rmlalgebra::util::extract_attributes_in_sm_poms;
use crate::OperatorTranslator;

#[derive(Debug, Clone)]
pub struct ProjectionTranslator<'a> {
    pub tm_iri:       &'a str,
    pub sm:           &'a SubjectMap,
    pub poms:         &'a [PredicateObjectMap],
    pub search_map:   &'a SearchMap<'a>,
    pub child_tm_iri: Option<&'a str>,
}

impl<'a> OperatorTranslator<Operator> for ProjectionTranslator<'a> {
    fn translate(&self) -> Operator {
        let mut projection_attributes =
            extract_attributes_in_sm_poms(self.sm, self.poms);

        if let Some(child_tm_iri) = self.child_tm_iri {
            let other_tms: Vec<&TriplesMap> = self
                .search_map
                .tm_rccellplan_map
                .get(child_tm_iri)
                .map(|pair| pair.0)
                .into_iter()
                .collect();

            let jc_attributes =
                extract_ptm_join_conditions_attributes(other_tms, self.tm_iri);

            projection_attributes.extend(jc_attributes);
        }

        Operator::ProjectOp {
            config: Projection {
                projection_attributes,
            },
        }
    }
}

pub fn extract_ptm_join_conditions_attributes<'a>(
    tms: Vec<&'a TriplesMap>,
    target_ptm: &'a str,
) -> HashSet<String> {
    let mut result = HashSet::new();
    for tm in tms {
        let poms = &tm.po_maps;
        for pom in poms {
            for om in &pom.object_maps {
                if let Some(ptm_iri) = &om.parent_tm {
                    let ptm_iri_string = ptm_iri.to_string();
                    if ptm_iri_string == target_ptm {
                        let value = om
                            .join_condition
                            .as_ref()
                            .map(|jc| {
                                HashSet::from_iter(
                                    jc.parent_attributes.clone().into_iter(),
                                )
                            })
                            .unwrap_or(HashSet::new());

                        result.extend(value);
                    }
                }
            }
        }
    }

    result
}
