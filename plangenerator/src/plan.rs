use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::marker::PhantomData;
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::Result;
use operator::display::PrettyDisplay;
use operator::{Fragmenter, Join, Operator, Serializer, Source, Target};
use petgraph::dot::Dot;
use petgraph::graph::{DiGraph, NodeIndex};
use serde::Serialize;
use serde_json::json;

use crate::error::PlanError;

pub type DiGraphOperators = DiGraph<PlanNode, PlanEdge>;
pub type RcRefCellDiGraph = Rc<RefCell<DiGraphOperators>>;

type VSourceIdxs = Vec<NodeIndex>;
pub type RcRefCellVSourceIdxs = Rc<RefCell<VSourceIdxs>>;

pub type RcRefCellPlan<T> = Rc<RefCell<Plan<T>>>;

const DEFAULT_FRAGMENT: &'static str = "default";
// Plan states in unit structs

#[derive(Debug, Clone)]
pub struct Init {}
#[derive(Debug, Clone)]
pub struct Processed {}
#[derive(Debug, Clone)]
pub struct Serialized {}
#[derive(Debug, Clone)]
pub struct Sunk {}

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
        let graph = &*self.graph.borrow();
        let json_string = serde_json::to_string(&graph).unwrap();
        write_string_to_file(path, json_string)
    }

    pub fn to_string(&self) -> Result<String> {
        let graph = &*self.graph.borrow();
        let json_string = serde_json::to_string(&graph).unwrap();

        Ok(json_string)
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
        let idx = Some(graph.add_node(plan_node));
        sources.push(idx.unwrap());
        self.next_idx(idx)
    }
}

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

pub fn join(
    left_plan: RcRefCellPlan<Processed>,
    right_plan: RcRefCellPlan<Processed>,
) -> Result<NotAliasedJoinedPlan<Processed>, PlanError> {
    Ok(NotAliasedJoinedPlan {
        left_plan:  Rc::clone(&left_plan),
        right_plan: Rc::clone(&right_plan),
    })
}

fn add_join_fragmenter(
    plan: &mut Plan<Processed>,
    alias: &str,
) -> Result<Plan<Processed>, PlanError> {
    if plan.fragment_node_idx == plan.last_node_idx {
        plan.update_prev_fragment_node(alias);
        Ok(plan.clone())
    } else {
        let default_fragment = plan.get_fragment_str();
        let fragmenter = Fragmenter {
            from: default_fragment.clone(),
            to:   vec![default_fragment, alias.to_string()],
        };
        plan.fragment(fragmenter)
    }
}

#[derive(Debug, Clone)]
pub struct NotAliasedJoinedPlan<T> {
    left_plan:  RcRefCellPlan<T>,
    right_plan: RcRefCellPlan<T>,
}
impl NotAliasedJoinedPlan<Processed> {
    pub fn alias(
        &mut self,
        alias: &str,
    ) -> Result<AliasedJoinedPlan<Processed>, PlanError> {
        {
            let right_plan = &mut *self.right_plan.borrow_mut();
            *right_plan = add_join_fragmenter(right_plan, alias)?;
        }
        {
            let left_plan = &mut *self.left_plan.borrow_mut();
            *left_plan = add_join_fragmenter(left_plan, alias)?;
        }

        Ok(AliasedJoinedPlan {
            left_plan:  Rc::clone(&self.left_plan),
            right_plan: Rc::clone(&self.right_plan),
            alias:      alias.to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct AliasedJoinedPlan<T> {
    left_plan:  RcRefCellPlan<T>,
    right_plan: RcRefCellPlan<T>,
    alias:      String,
}

impl AliasedJoinedPlan<Processed> {
    pub fn apply_right(
        self,
        operator: Operator,
        op_name: Cow<str>,
    ) -> Result<AliasedJoinedPlan<Processed>, PlanError> {
        let applied_right =
            self.right_plan.borrow_mut().apply(&operator, &op_name)?;

        Ok(AliasedJoinedPlan {
            right_plan: Rc::new(RefCell::new(applied_right)),
            ..self
        })
    }

    pub fn apply_left(
        self,
        operator: Operator,
        op_name: Cow<str>,
    ) -> Result<AliasedJoinedPlan<Processed>, PlanError> {
        let applied_left =
            self.left_plan.borrow_mut().apply(&operator, &op_name)?;

        Ok(AliasedJoinedPlan {
            left_plan: Rc::new(RefCell::new(applied_left)),
            ..self
        })
    }

    fn add_join_op_to_plan(&mut self, join_op: Operator) -> Plan<Processed> {
        let fragment_str = &self.alias;
        let node_idx;
        {
            let left_plan = self.left_plan.borrow_mut();
            let graph = &mut left_plan.graph.borrow_mut();

            let join_node = PlanNode {
                id:       format!("Join_{}", graph.node_count()),
                operator: join_op,
            };

            node_idx = graph.add_node(join_node);

            let left_node = left_plan.last_node_idx.unwrap();
            let left_edge = PlanEdge {
                fragment:  fragment_str.to_string(),
                direction: EdgeDirection::Left,
            };

            graph.add_edge(left_node, node_idx, left_edge);
        }

        {
            let left_plan = self.left_plan.borrow_mut();
            let graph = &mut left_plan.graph.borrow_mut();

            if let Ok(plan) = self.right_plan.try_borrow_mut() {
                let right_node = plan.last_node_idx.unwrap();
                let right_edge = PlanEdge {
                    fragment:  fragment_str.to_string(),
                    direction: EdgeDirection::Right,
                };

                graph.add_edge(right_node, node_idx, right_edge);
            } else {
                // This case happens when left_plan and right_plan are the same
                // (self-join situation)
                let right_node = left_plan.last_node_idx.unwrap();
                let right_edge = PlanEdge {
                    fragment:  fragment_str.to_string(),
                    direction: EdgeDirection::Right,
                };

                graph.add_edge(right_node, node_idx, right_edge);
            }
        }

        let left_plan = self.left_plan.borrow_mut();
        left_plan.next_idx(Some(node_idx))
    }
    pub fn where_by<A>(
        &mut self,
        attributes: Vec<A>,
    ) -> Result<WhereByPlan<Processed>, PlanError>
    where
        A: Into<String>,
    {
        let left_attributes: Vec<String> =
            attributes.into_iter().map(|a| a.into()).collect();
        Ok(WhereByPlan {
            joined_plan: self.clone(),
            left_attributes,
        })
    }

    pub fn cross_join(&mut self) -> Result<Plan<Processed>, PlanError> {
        let join_alias = &self.alias;

        let join_op = Operator::JoinOp {
            config: Join {
                left_right_attr_pairs: vec![],
                join_type:             operator::JoinType::CrossJoin,
                predicate_type:        operator::PredicateType::Equal,
                join_alias:            join_alias.to_string(),
            },
        };

        Ok(self.add_join_op_to_plan(join_op))
    }

    pub fn natural_join(&mut self) -> Result<Plan<Processed>, PlanError> {
        let join_op = Operator::JoinOp {
            config: Join {
                left_right_attr_pairs: vec![],
                join_type:             operator::JoinType::NaturalJoin,
                predicate_type:        operator::PredicateType::Equal,
                join_alias:            self.alias.clone(),
            },
        };

        Ok(self.add_join_op_to_plan(join_op))
    }
}

#[derive(Debug, Clone)]
pub struct WhereByPlan<T> {
    joined_plan:     AliasedJoinedPlan<T>,
    left_attributes: Vec<String>,
}

impl WhereByPlan<Processed> {
    pub fn compared_to<A>(
        &mut self,
        attributes: Vec<A>,
    ) -> Result<Plan<Processed>, PlanError>
    where
        A: Into<String>,
    {
        let joined_plan = &mut self.joined_plan;
        let left_attributes = self.left_attributes.to_owned();
        let right_attributes: Vec<String> =
            attributes.into_iter().map(|a| a.into()).collect();

        let left_right_attr_pairs: Vec<(String, String)> = left_attributes
            .clone()
            .into_iter()
            .zip(right_attributes.clone())
            .collect();

        // TODO: Enable specification of join type and predicate type  <12-03-24, yourname> //
        let join_op = Operator::JoinOp {
            config: Join {
                join_alias: joined_plan.alias.clone(),
                left_right_attr_pairs,
                join_type: operator::JoinType::InnerJoin,
                predicate_type: operator::PredicateType::Equal,
            },
        };

        Ok(joined_plan.add_join_op_to_plan(join_op))
    }
}

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

#[derive(Clone, Serialize)]
pub struct PlanEdge {
    pub fragment:  String,
    pub direction: EdgeDirection,
}

#[derive(Debug, Clone, Serialize)]
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

#[derive(Clone, Serialize)]
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

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use operator::{Iterator, Projection, Rename};

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
