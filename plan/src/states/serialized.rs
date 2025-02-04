use anyhow::Result;
use operator::{Operator, Target};

use crate::data_type::{PlanEdge, PlanNode};
use crate::error::PlanError;
use crate::states::{Serialized, Sunk};
use crate::Plan;

impl Plan<Serialized> {
    pub fn sink(&mut self, sink: &Target) -> Result<Plan<Sunk>, PlanError> {
        if self.last_node_idx.is_none() {
            return Err(PlanError::EmptyPlan);
        }

        let graph = &mut *self.graph.borrow_mut();
        let plan_node = PlanNode {
            id:       format!("Sink_{}", graph.node_count()),
            operator: Operator::TargetOp {
                config: sink.clone(),
            },
        };

        let node_idx = graph.add_node(plan_node);
        let prev_node_idx = self.last_node_idx.unwrap();

        let plan_edge = PlanEdge {
            fragment: self.get_fragment_str().to_string(),
            ..Default::default()
        };
        graph.add_edge(prev_node_idx, node_idx, plan_edge);

        Ok(self.next_idx(Some(node_idx)))
    }
}
