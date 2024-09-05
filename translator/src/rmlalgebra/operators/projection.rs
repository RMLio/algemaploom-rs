use operator::{Operator, Projection};
use rml_interpreter::rml_model::join::JoinCondition;
use rml_interpreter::rml_model::term_map::{SubjectMap, TermMapInfo};
use rml_interpreter::rml_model::{PredicateObjectMap, TriplesMap};

use crate::rmlalgebra::types::SearchMap;
use crate::rmlalgebra::util::{
    extract_attributes_in_tm_infos, extract_ptm_join_conditions_attributes,
};
use crate::OperatorTranslator;

#[derive(Debug, Clone)]
pub struct ProjectionTranslator<'a> {
    pub tm_infos:       &'a [&'a TermMapInfo],
    pub join_condition: Vec<&'a JoinCondition>,
    pub is_parent:      bool,
}

impl<'a> OperatorTranslator<Operator> for ProjectionTranslator<'a> {
    fn translate(&self) -> Operator {
        let mut projection_attributes =
            extract_attributes_in_tm_infos(self.tm_infos);
        for jc in &self.join_condition {
            if self.is_parent {
                projection_attributes.extend(jc.parent_attributes.clone());
            } else {
                projection_attributes.extend(jc.child_attributes.clone());
            }
        }

        Operator::ProjectOp {
            config: Projection {
                projection_attributes,
            },
        }
    }
}
