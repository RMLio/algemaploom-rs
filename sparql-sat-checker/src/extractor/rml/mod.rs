use std::collections::HashSet;

use error::RMLResult;
use operator::Operator;
use petgraph::visit::{IntoNodeReferences, Reversed};
use plan::data_type::DiGraphOperators;
use trmap_sub_expression::TrMapSubExpression;

use crate::is_graph;
pub mod error;
pub mod trmap_sub_expression;

pub fn extract_trmap_sub_expressions_from_plan(
    graph: &DiGraphOperators,
) -> RMLResult<Vec<TrMapSubExpression>> {
    // Change the direction of the edge from leaf -> root to leaf <- root
    let reversed_graph = Reversed(graph);
    let mut result = Vec::new();

    // Only check for nodes that are extend operator for graph maps since the children now will
    // represent trmap sub expressions
    for (idx, _) in graph
        .node_references()
        .filter(|(_, node)| is_graph(&node.operator))
    {
        result.push(TrMapSubExpression::try_new(&reversed_graph, idx)?);
    }
    Ok(result)
}
