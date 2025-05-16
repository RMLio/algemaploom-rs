//!
//! Provides different functionalities to create an execution plan consisting
//! of operators from the mapping algebra.
//!
//! The execution plan is represented as a *graph* in order to be flexible and
//! support future expansions where the formal semantics cannot be captured as
//! an execution *tree*.
//!
//!
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::marker::PhantomData;
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::Result;
use data_type::RcRefCellPlan;
use operator::{Fragmenter, Operator};
use petgraph::dot::Dot;
use petgraph::graph::{DiGraph, NodeIndex};

use crate::data_type::{
    DiGraphOperators, PlanEdge, PlanNode, RcRefCellDiGraph,
    RcRefCellVSourceIdxs, DEFAULT_FRAGMENT,
};
use crate::error::PlanError;
use crate::states::Init;
pub mod data_type;
pub mod error;
pub mod states;

/// Represents a plan in state [T](states) with functions to transition state
/// changes.
///
/// The field `graph` is repeatedly modified to build up the mapping plan with 
/// the addition of mapping algebra operators.
#[derive(Debug, Clone)]
pub struct Plan<T> {
    _t:        PhantomData<T>,
    /// Underlying graph data structure from [petgraph](petgraph::graph::DiGraph).
    pub graph: RcRefCellDiGraph,

    /// Node indexes of the source operators in the graph.
    pub sources:           RcRefCellVSourceIdxs,

    /// Index of the previous node on which the plan is being built upon.
    pub last_node_idx:     Option<NodeIndex>,

    /// Index of the previous **fragment** node on which the plan is being built upon.
    pub fragment_node_idx: Option<NodeIndex>,

    /// String label of the current fragment on which the plan is being built upon.
    pub fragment_string:   Rc<String>,
}

impl<T> From<Plan<T>> for RcRefCellPlan<T> {
    fn from(value: Plan<T>) -> Self {
        Rc::new(RefCell::new(value))
    }
}

impl Plan<()> {
    /// Creates a new empty mapping plan
    pub fn new() -> Plan<Init> {
        Plan {
            _t:                PhantomData,
            graph:             Rc::new(RefCell::new(DiGraph::new())),
            sources:           Rc::new(RefCell::new(Vec::new())),
            fragment_string:   Rc::new(DEFAULT_FRAGMENT.to_string()),
            fragment_node_idx: None,
            last_node_idx:     None,
        }
    }
}

impl<T> Plan<T> {
    fn update_prev_fragment_node(&mut self, new_fragment: &str) {
        let mut graph = self.graph.borrow_mut();
        let fragment_node = graph
            .node_weight_mut(self.fragment_node_idx.unwrap())
            .unwrap();

        let mut update_fragment = match fragment_node.operator.clone() {
            Operator::FragmentOp { config } => config,

            _ => {
                Fragmenter {
                    from: self.get_fragment_str(),
                    to:   vec![self.get_fragment_str()],
                }
            }
        };

        update_fragment.to.push(new_fragment.to_string());

        fragment_node.operator = Operator::FragmentOp {
            config: update_fragment,
        };
    }

    fn get_fragment_op(&self) -> Option<Fragmenter> {
        if let Some(idx) = self.fragment_node_idx {
            let graph = self.graph.borrow();
            let fragment_node = graph.node_weight(idx).unwrap();

            return match &fragment_node.operator {
                Operator::FragmentOp { config } => Some(config.clone()),
                _ => None,
            };
        }

        None
    }

    fn target_fragment_valid(
        &self,
        target_fragment: &str,
    ) -> Result<(), PlanError> {
        let fragment_op = self.get_fragment_op();
        let current_fragment = &*self.fragment_string;

        if fragment_op.is_none() && target_fragment != current_fragment {
            return Err(PlanError::GenericError(format!(
                "Target fragment {} is NOT equal to current fragment {} and there aren't any previous fragmenter",
                target_fragment, current_fragment
            )));
        } else if let Some(fragmenter) = fragment_op {
            if !fragmenter.target_fragment_exist(target_fragment) {
                return Err(PlanError::GenericError(format!(
                    "Target fragment {} doesn't exists as part of the output fragments of the previous fragmenter",
                    target_fragment
                )));
            }
        }

        Ok(())
    }

    fn get_fragment_str(&self) -> String {
        (*self.fragment_string).clone()
    }

    fn node_count(&self) -> usize {
        self.graph.borrow().node_count()
    }

    fn non_empty_plan_check(&self) -> Result<(), PlanError> {
        if self.node_count() == 0 {
            return Err(PlanError::EmptyPlan);
        }
        Ok(())
    }

    fn add_node_with_edge(
        &mut self,
        plan_node: PlanNode,
        plan_edge: PlanEdge,
    ) -> NodeIndex {
        let mut graph = self.graph.borrow_mut();

        let node_idx = graph.add_node(plan_node);
        let prev_node_idx = self.last_node_idx.unwrap();
        graph.add_edge(prev_node_idx, node_idx, plan_edge);
        node_idx
    }

    /// Update last_node_idx with given node's index to ensure that next 
    /// operator addition on the plan continues from the given node.
    pub fn next_idx<O>(&self, idx: Option<NodeIndex>) -> Plan<O> {
        Plan {
            _t:                PhantomData,
            graph:             Rc::clone(&self.graph),
            sources:           Rc::clone(&self.sources),
            fragment_string:   Rc::clone(&self.fragment_string),
            fragment_node_idx: self.fragment_node_idx,
            last_node_idx:     idx,
        }
    }

    /// Update last_node_idx with given node's index and update the fragment 
    /// string of the plan to ensure that next 
    /// operator addition on the plan continues from the given node, and that it 
    /// is a part of the updated fragment.
    pub fn next_idx_fragment<O>(
        &self,
        idx: Option<NodeIndex>,
        fragment_string: &str,
    ) -> Plan<O> {
        Plan {
            _t:                PhantomData,
            graph:             Rc::clone(&self.graph),
            sources:           Rc::clone(&self.sources),
            fragment_string:   Rc::new(fragment_string.to_string()),
            fragment_node_idx: self.fragment_node_idx,
            last_node_idx:     idx,
        }
    }

    /// Serializes the plan with the given [dot](Dot) formatter, `fmt`, to a file 
    /// at the given `path`.
    pub fn write_fmt(
        &mut self,
        path: PathBuf,
        fmt: &dyn Fn(Dot<&DiGraphOperators>) -> String,
    ) -> Result<()> {
        let graph = &*self.graph.borrow_mut();
        let dot_string = fmt(Dot::with_config(graph, &[]));
        write_string_to_file(path, dot_string)?;
        Ok(())
    }


    /// Serializes the plan using the [Display](std::fmt::Display) trait which 
    /// makes the serialized plan more readable and less verbose. 
    pub fn write_pretty(&mut self, path: PathBuf) -> Result<()> {
        self.write_fmt(path, &|dot| format!("{}", dot))?;
        Ok(())
    }

    /// Serializes the plan using the [Debug](std::fmt::Debug) trait which 
    /// makes the serialized plan more verbose for debugging purpose. 
    pub fn write(&mut self, path: PathBuf) -> Result<()> {
        self.write_fmt(path, &|dot| format!("{:?}", dot))?;
        Ok(())
    }

    /// Serializes the plan in JSON format to a file at the given `path`. 
    /// Delegates the actual serialization to [Plan::to_json_string()].  
    pub fn write_json(&self, path: PathBuf) -> Result<()> {
        write_string_to_file(path, self.to_json_string()?)
    }

    /// Parses the mapping plan from a file at the given `path`. 
    /// 
    /// # Required 
    /// The serialized plan has to be in **JSON** format parsable with [serde_json]. 
    ///
    /// # Error 
    /// Returns error if there is an IO error or the given input file cannot be 
    /// parsed with [serde_json]. 
    pub fn from_file_path(path: PathBuf) -> Result<Plan<Init>> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        let graph: DiGraph<PlanNode, PlanEdge> = serde_json::from_str(&buf)?;

        let mut plan = Plan::new();
        plan.graph = Rc::new(RefCell::new(graph));
        Ok(plan)
    }

    #[deprecated(note="please use `to_json_string` method instead")]
    pub fn to_string(&self) -> Result<String> {
        let graph = &*self.graph.borrow();
        let json_string = serde_json::to_string(&graph).unwrap();

        Ok(json_string)
    }

    /// Serializes the plan to a [String] in **JSON** format with [serde_json].
    ///
    /// # Error 
    /// Returns an error if [serde_json] fails to serialize the plan.
    pub fn to_json_string(&self) -> Result<String> {
        let graph = &*self.graph.borrow();
        let json_str = serde_json::to_string(&graph)?;
        Ok(json_str)
    }
}

fn write_string_to_file(
    path: PathBuf,
    content: String,
) -> Result<(), anyhow::Error> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    write!(writer, "{}", content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use operator::{Iterator, Projection, Rename, Source};
    use petgraph::algo::is_isomorphic_matching;
    use states::Processed;

    use super::*;

    fn generate_dummy_processed_plan(
    ) -> std::result::Result<Plan<Processed>, PlanError> {
        let mut plan = Plan::new();
        let source = Source {
            config:        HashMap::new(),
            source_type:   operator::IOType::File,
            root_iterator: Iterator::default(),
        };

        let project_op = Operator::ProjectOp {
            config: Projection {
                projection_attributes: HashSet::new(),
            },
        };
        let rename_op = Operator::RenameOp {
            config: Rename {
                rename_pairs: HashMap::from([(
                    "first".to_string(),
                    "last".to_string(),
                )]),
            },
        };

        let plan = plan
            .source(source.clone())
            .apply(&project_op, "Projection")?
            .apply(&rename_op, "Rename")?;

        Ok(plan)
    }

    #[test]
    fn test_plan_source() {
        let mut plan = Plan::new();
        let source = Source {
            config:        HashMap::new(),
            source_type:   operator::IOType::File,
            root_iterator: Iterator::default(),
        };
        plan.source(source.clone());
        let graph = plan.graph.borrow();

        assert!(graph.node_count() == 1);
        assert!(graph.edge_count() == 0);
        let retrieved_node = graph.node_weights().next();

        assert!(retrieved_node.is_some());
        let source_op = Operator::SourceOp { config: source };
        assert!(retrieved_node.unwrap().operator == source_op);
    }

    #[test]
    fn test_plan_apply() -> std::result::Result<(), PlanError> {
        let plan = generate_dummy_processed_plan()?;
        let graph = plan.graph.borrow();

        assert!(
            graph.node_count() == 3,
            "Number of nodes should be 3 but it is instead: {}",
            graph.node_count()
        );
        assert!(
            graph.edge_count() == 2,
            "Number of edges should be 2 but it is instead: {}",
            graph.edge_count()
        );

        Ok(())
    }

    #[test]
    fn test_plan_serialization() -> Result<(), PlanError> {
        let plan = generate_dummy_processed_plan()?;
        plan.non_empty_plan_check()?;

        let plan_json_string = plan.to_json_string().map_err(|_| {
            PlanError::GenericError(format!(
                "Something went wrong while serializing to json: {:?}",
                plan
            ))
        })?;

        let plan_serialized: DiGraphOperators =
            serde_json::from_str(&plan_json_string)
                .map_err(|err| PlanError::GenericError(err.to_string()))?;

        let node_match_fn = |node1: &PlanNode, node2: &PlanNode| -> bool {
            node1.operator == node2.operator
        };

        let edge_match_fn =
            |edge1: &PlanEdge, edge2: &PlanEdge| -> bool { edge1 == edge2 };

        let plan_graph: &DiGraphOperators = &plan.graph.borrow_mut();
        assert!(is_isomorphic_matching(
            plan_graph,
            &plan_serialized,
            node_match_fn,
            edge_match_fn,
        ));

        Ok(())
    }
}
