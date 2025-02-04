use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::Rc;

use anyhow::Result;
use operator::display::PrettyDisplay;
use operator::Operator;
use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Plan;

pub type DiGraphOperators = DiGraph<PlanNode, PlanEdge>;
pub type RcRefCellDiGraph = Rc<RefCell<DiGraphOperators>>;

type VSourceIdxs = Vec<NodeIndex>;
pub type RcRefCellVSourceIdxs = Rc<RefCell<VSourceIdxs>>;

pub type RcRefCellPlan<T> = Rc<RefCell<Plan<T>>>;

pub const DEFAULT_FRAGMENT: &'static str = "default";
// Plan states in unit structs

#[derive(Clone, Serialize, Deserialize)]
pub struct PlanEdge {
    pub fragment:  String,
    pub direction: EdgeDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeDirection {
    Left,
    Right,
    Center,
}

impl Default for PlanEdge {
    fn default() -> Self {
        Self {
            fragment:  DEFAULT_FRAGMENT.to_string(),
            direction: EdgeDirection::Center,
        }
    }
}

impl Display for PlanEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fragment:{}", self.fragment)
    }
}

impl Debug for PlanEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{{\"fragment\": {}}}", self.fragment))
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PlanNode {
    pub id:       String,
    pub operator: Operator,
}

impl Debug for PlanNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = json!({"id": self.id, "operator": self.operator});
        f.write_str(&serde_json::to_string(&json).unwrap())
    }
}

impl PrettyDisplay for PlanNode {
    fn pretty_string(&self) -> Result<String> {
        let content = self.operator.pretty_string()?;

        Ok(format!("Id: {}\n{}", self.id, content))
    }
}

impl Display for PlanNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id:{} \n{}",
            self.id,
            self.operator.pretty_string().unwrap()
        )
    }
}
