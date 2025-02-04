use operator::{Operator, Source};

use crate::data_type::PlanNode;
use crate::states::{Init, Processed};
use crate::Plan;

impl Plan<Init> {
    pub fn source(&mut self, source: Source) -> Plan<Processed> {
        let graph = &mut *self.graph.borrow_mut();
        let source_op = Operator::SourceOp {
            config: source.clone(),
        };
        let sources = &mut *self.sources.borrow_mut();

        let plan_node = PlanNode {
            id:       format!("Source_{}", graph.node_count()),
            operator: source_op,
        };
        let idx = graph.add_node(plan_node);
        sources.push(idx);
        self.next_idx(Some(idx))
    }
}
