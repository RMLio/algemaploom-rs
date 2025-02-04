use operator::{Fragmenter, Operator, Serializer};

use super::Processed;
use crate::data_type::{PlanEdge, PlanNode};
use crate::error::PlanError;
use crate::states::Serialized;
use crate::Plan;

impl Plan<Processed> {
    pub fn apply_to_fragment(
        &mut self,
        operator: &Operator,
        node_id_prefix: &str,
        fragment_str: &str,
    ) -> Result<Plan<Processed>, PlanError> {
        self.non_empty_plan_check()?;
        self.target_fragment_valid(fragment_str)?;

        self.last_node_idx
            .ok_or(PlanError::DanglingApplyOperator(operator.clone()))?;

        //blacklist check for illegal operator argument
        match operator {
            Operator::SourceOp { .. }
            | Operator::FragmentOp { .. }
            | Operator::TargetOp { .. }
            | Operator::SerializerOp { .. } => {
                return Err(PlanError::WrongApplyOperator(operator.clone()))
            }
            _ => (),
        };

        let id_num = self.node_count();

        let plan_node = PlanNode {
            id:       format!("{}_{}", node_id_prefix, id_num),
            operator: operator.clone(),
        };

        let plan_edge = PlanEdge {
            fragment: fragment_str.to_string(),
            ..Default::default()
        };

        let new_node_idx = self.add_node_with_edge(plan_node, plan_edge);

        Ok(self.next_idx_fragment(Some(new_node_idx), fragment_str))
    }

    pub fn apply(
        &mut self,
        operator: &Operator,
        node_id_prefix: &str,
    ) -> Result<Plan<Processed>, PlanError> {
        let fragment_str = &self.get_fragment_str();
        self.apply_to_fragment(operator, node_id_prefix, fragment_str)
    }

    pub fn fragment(
        &mut self,
        fragmenter: Fragmenter,
    ) -> Result<Plan<Processed>, PlanError> {
        self.non_empty_plan_check()?;
        self.target_fragment_valid(&fragmenter.from)?;
        self.last_node_idx.ok_or(PlanError::DanglingApplyOperator(
            Operator::FragmentOp {
                config: fragmenter.clone(),
            },
        ))?;

        let id_num = self.node_count();

        let fragment_node = PlanNode {
            id:       format!("Fragmenter_{}", id_num),
            operator: Operator::FragmentOp {
                config: fragmenter.clone(),
            },
        };

        let edge = PlanEdge {
            fragment: fragmenter.from.clone(),
            ..Default::default()
        };
        let node_idx = self.add_node_with_edge(fragment_node, edge);

        self.fragment_node_idx = Some(node_idx);

        Ok(self.next_idx(Some(node_idx)))
    }

    pub fn serialize_with_fragment(
        &mut self,
        serializer: Serializer,
        fragment_str: &str,
    ) -> Result<Plan<Serialized>, PlanError> {
        self.non_empty_plan_check()?;
        self.target_fragment_valid(fragment_str)?;
        self.last_node_idx.ok_or(PlanError::DanglingApplyOperator(
            Operator::SerializerOp {
                config: serializer.clone(),
            },
        ))?;

        let node_count = self.node_count();
        let plan_node = PlanNode {
            id:       format!("Serialize_{}", node_count),
            operator: Operator::SerializerOp { config: serializer },
        };

        let plan_edge = PlanEdge {
            fragment: fragment_str.to_string(),
            ..Default::default()
        };

        let node_idx = self.add_node_with_edge(plan_node, plan_edge);
        Ok(self.next_idx_fragment(Some(node_idx), fragment_str))
    }

    pub fn serialize(
        &mut self,
        serializer: Serializer,
    ) -> Result<Plan<Serialized>, PlanError> {
        self.serialize_with_fragment(serializer, &self.get_fragment_str())
    }
}
