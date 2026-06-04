use std::collections::{HashMap, HashSet};

use operator::io::io_type::IOType;
use operator::{formats::DataFormat, Serializer, Target};
use petgraph::visit::IntoNodeReferences;
use plan::data_type::{DiGraphOperators, PlanEdge, PlanNode};
use translator_normalized_rml::{GRAPH_ATTR, OBJECT_ATTR, PREDICATE_ATTR, SUBJECT_ATTR};

use crate::{extractor::rml::trmap_sub_expression::TrMapSubExpression, is_graph};
pub mod error;
pub mod reverse_rml;
pub mod util;

pub fn prune_graph_using_trmap_subexprs(
    mut graph: DiGraphOperators,
    trmap_sub_exprs: &[TrMapSubExpression],
) -> DiGraphOperators {
    let relevent_node_indices: HashSet<_> = trmap_sub_exprs
        .iter()
        .flat_map(|trmap| &trmap.node_indices)
        .collect();

    graph.retain_nodes(|_, idx| relevent_node_indices.contains(&idx));
    let mut projection_nodes_vec: Vec<_> = graph
        .node_references()
        .filter(|(_, node)| is_graph(&node.operator))
        //need to copy/own the relevant nodes to be able to mutate the graph
        .map(|(idx, node_ref)| (idx, node_ref.to_owned()))
        .collect();

    if !projection_nodes_vec.is_empty() {
        let (mut current_idx, _current_node) = projection_nodes_vec.pop().unwrap();

        //If vec is still not empty after pop, union operators need to be added
        if !projection_nodes_vec.is_empty() {
            let mut count = 0;
            let union_str = "Union_";
            while let Some((next_idx, _)) = projection_nodes_vec.pop() {
                let union = PlanNode {
                    id: format!("{}{}", union_str, count),
                    operator: operator::Operator::UnionOp,
                };
                let union_idx = graph.add_node(union.clone());
                graph.add_edge(current_idx, union_idx, PlanEdge::default());
                graph.add_edge(next_idx, union_idx, PlanEdge::default());

                current_idx = union_idx;
                count += 1;
            }
        }
        let template = format!(
            "?{} ?{} ?{} ?{} .",
            SUBJECT_ATTR, PREDICATE_ATTR, OBJECT_ATTR, GRAPH_ATTR
        );
        let serializer = Serializer {
            template,
            options: None,
            format: DataFormat::NQuads,
        };

        let serializer_idx = graph.add_node(PlanNode {
            id: "Serializer".to_string(),
            operator: operator::Operator::SerializerOp { config: serializer },
        });
        graph.add_edge(current_idx, serializer_idx, PlanEdge::default());
        current_idx = serializer_idx;

        let sink = Target {
            configuration: HashMap::new(),
            target_type: IOType::StdOut,
            data_format: DataFormat::NQuads,
        };
        let sink_idx = graph.add_node(PlanNode {
            id: "TargetSink".to_string(),
            operator: operator::Operator::TargetOp { config: sink },
        });
        graph.add_edge(current_idx, sink_idx, PlanEdge::default());
    }
    graph
}
