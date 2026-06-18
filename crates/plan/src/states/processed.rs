use super::Processed;
use crate::data_type::{EdgeDirection, PlanEdge, PlanNode, RcRefCellPlan};
use crate::error::PlanError;
use crate::states::Serialized;
use crate::Plan;
use operator::serializer::Serializer;
use operator::{Operator, Source};

impl Plan<Processed> {
    /// .
    ///
    /// # Panics
    ///
    /// Panics if the current underlying graph has already been borrowed.
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn union(
        &mut self,
        other: RcRefCellPlan<Processed>,
    ) -> Result<Plan<Processed>, PlanError> {
        let mut graph = self.graph.borrow_mut(); // might panic here!
        let union_node = PlanNode {
            id:       format!("Union_{}", graph.node_count()),
            operator: Operator::UnionOp,
        };

        let node_idx = graph.add_node(union_node);
        let self_node = self.current_cursor_idx.unwrap();
        let left_edge = PlanEdge {
            direction: EdgeDirection::Left,
            ..Default::default()
        };
        graph.add_edge(self_node, node_idx, left_edge);

        if let Ok(other_plan) = other.try_borrow_mut() {
            let right_node = other_plan.current_cursor_idx.unwrap();
            let right_edge = PlanEdge {
                direction: EdgeDirection::Right,
                ..Default::default()
            };
            graph.add_edge(right_node, node_idx, right_edge);
        } else {
            let right_edge = PlanEdge {
                direction: EdgeDirection::Right,
                ..Default::default()
            };
            graph.add_edge(self_node, self_node, right_edge);
        }

        Ok(self.next_idx(Some(node_idx)))
    }

    pub fn apply(
        &mut self,
        operator: &Operator,
        node_id_prefix: &str,
    ) -> Result<Plan<Processed>, PlanError> {
        self.non_empty_plan_check()?;

        self.current_cursor_idx
            .ok_or(PlanError::DanglingApplyOperator(operator.clone()))?;

        let mut stop = false;
        //blacklist check for illegal operator argument
        match operator {
            Operator::SourceOp { .. }
            | Operator::TargetOp { .. }
            | Operator::SerializerOp { .. } => {
                return Err(PlanError::WrongApplyOperator(operator.clone()))
            },
            Operator::ExtendOp { config } => {
                // Check if the previous node is also an extend op. If so, merge!
                let mut graph = self.graph.borrow_mut();
                let _ = graph.node_weight_mut(self.current_cursor_idx.unwrap()).map(|node| {
                    if let Operator::ExtendOp { config: prev_config } = &mut node.operator {
                        // Add the new extend pairs to the previous extend operator's config
                        prev_config.extend_pairs.extend(config.extend_pairs.clone());
                        stop = true;
                    }
                });
                ()
            },
            _ => (),
        }

        if !stop {
            let id_num = self.node_count();
            let plan_node = PlanNode {
                id: format!("{}_{}", node_id_prefix, id_num),
                operator: operator.clone(),
            };
            let plan_edge = PlanEdge::default();
            let new_node_idx = self.add_node_with_edge(plan_node, plan_edge);
            Ok(self.next_idx(Some(new_node_idx)))
        } else {
            Ok(self.next_idx(self.current_cursor_idx))
        }
    }

    pub fn merge_source(&mut self, other_source: &Source) -> Result<(), PlanError> {
        // Get the (only) source
        let sources = &mut *self.sources.borrow_mut();
        if sources.len() != 1 {
            return Err(PlanError::GenericError("More than one source operator exists in the plan.\
            Merge source operator can only be applied if there is exactly one source operator in the plan.".to_string()));
        }
        let source_index = sources[0];
        let mut graph = self.graph.borrow_mut();
        let source_op_node = graph.node_weight_mut(source_index).unwrap();
        let mut config = match &source_op_node.operator {
            Operator::SourceOp { config } => config.clone(),
            _ => unreachable!(),
        };
        config.merge(other_source);
        source_op_node.operator = Operator::SourceOp { config };

        Ok(())
    }


    pub fn serialize(
        &mut self,
        serializer: Serializer,
    ) -> Result<Plan<Serialized>, PlanError> {
        self.non_empty_plan_check()?;
        self.current_cursor_idx
            .ok_or(PlanError::DanglingApplyOperator(
                Operator::SerializerOp {
                    config: serializer.clone(),
                },
            ))?;

        let node_count = self.node_count();
        let plan_node = PlanNode {
            id:       format!("Serialize_{}", node_count),
            operator: Operator::SerializerOp { config: serializer },
        };

        let plan_edge = PlanEdge ::default();

        let node_idx = self.add_node_with_edge(plan_node, plan_edge);
        Ok(self.next_idx(Some(node_idx)))
    }
}
