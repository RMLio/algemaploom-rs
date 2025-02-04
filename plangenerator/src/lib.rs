use std::cell::RefCell;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::marker::PhantomData;
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::Result;
use operator::{Fragmenter, Operator};
use petgraph::dot::Dot;
use petgraph::graph::{DiGraph, NodeIndex};

use crate::data_type::{
    DiGraphOperators, PlanEdge, PlanNode, RcRefCellDiGraph,
    RcRefCellVSourceIdxs, DEFAULT_FRAGMENT,
};
use crate::error::PlanError;
use crate::states::Init;
mod data_type;
pub mod error;
pub mod states;

#[derive(Debug, Clone)]
pub struct Plan<T> {
    _t:                    PhantomData<T>,
    pub graph:             RcRefCellDiGraph,
    pub sources:           RcRefCellVSourceIdxs,
    pub last_node_idx:     Option<NodeIndex>,
    pub fragment_node_idx: Option<NodeIndex>,
    pub fragment_string:   Rc<String>,
}

impl Plan<()> {
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

    pub fn next_idx<O>(&self, idx: Option<NodeIndex>) -> Plan<O> {
        Plan {
            _t:                PhantomData,
            graph:             Rc::clone(&self.graph),
            sources:           Rc::clone(&self.sources),
            fragment_string:   Rc::clone(&self.fragment_string),
            fragment_node_idx: self.fragment_node_idx.clone(),
            last_node_idx:     idx,
        }
    }

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
            fragment_node_idx: self.fragment_node_idx.clone(),
            last_node_idx:     idx,
        }
    }

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

    pub fn write_pretty(&mut self, path: PathBuf) -> Result<()> {
        self.write_fmt(path, &|dot| format!("{}", dot))?;
        Ok(())
    }

    pub fn write(&mut self, path: PathBuf) -> Result<()> {
        self.write_fmt(path, &|dot| format!("{:?}", dot))?;
        Ok(())
    }

    pub fn write_json(&self, path: PathBuf) -> Result<()> {
        write_string_to_file(path, self.to_json_string()?)
    }

    pub fn from_file_path(path: PathBuf) -> Result<Plan<Init>> {
        todo!()
    }

    pub fn to_string(&self) -> Result<String> {
        let graph = &*self.graph.borrow();
        let json_string = serde_json::to_string(&graph).unwrap();

        Ok(json_string)
    }

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

    use super::*;

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

        let _ = plan
            .source(source.clone())
            .apply(&project_op, "Projection")?
            .apply(&rename_op, "Rename")?;

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
}
