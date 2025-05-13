//!
//! Contains definitions of the data structs to represent a mapping plan
//! consisting of [PlanNode] connected to each other with [PlanEdge] with
//! a specific [EdgeDirection].  
//!
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

/// Type alias for [DiGraph]<[PlanNode], [PlanEdge]>
pub type DiGraphOperators = DiGraph<PlanNode, PlanEdge>;

/// Type alias for [Rc]<[RefCell]<[DiGraphOperators]>>
pub type RcRefCellDiGraph = Rc<RefCell<DiGraphOperators>>;

type VSourceIdxs = Vec<NodeIndex>;

// TODO: Come up with a documentation for this type alias <13-05-25, Min Oo> //
#[doc(hidden)]
pub type RcRefCellVSourceIdxs = Rc<RefCell<VSourceIdxs>>;

/// Type alias for [Rc]<[RefCell]<[Plan<T>]>>
pub type RcRefCellPlan<T> = Rc<RefCell<Plan<T>>>;

/// Default [str] to be used as a label for default fragment.
pub const DEFAULT_FRAGMENT: &str = "default";


/// Edge of the mapping plan labelled with a fragment string and the direction 
/// in which it is connecting the nodes. 
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanEdge {
    pub fragment:  String,
    pub direction: EdgeDirection,
}

/// Enums for the direction of the edges connecting the nodes in the mapping plan. 
/// Useful for handling join operators. 
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// Node of the mapping plan used to represent a mapping algebra [operator](Operator). 
#[derive(Clone, Serialize, Deserialize)]
pub struct PlanNode {
    /// String label identifier of the underlying mapping operator. 
    pub id:       String,

    /// Algebraic mapping [operator](Operator). 
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
