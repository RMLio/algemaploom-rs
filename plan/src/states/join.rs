use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;
use operator::{Fragmenter, Join, Operator};

use crate::data_type::{EdgeDirection, PlanEdge, PlanNode, RcRefCellPlan};
use crate::error::PlanError;
use crate::states::Processed;
use crate::Plan;

/// Joins the given two mutable plans and transitions to the state [NotAliasedJoinedPlan].
///
/// # Note
/// **Only the left_plan** will be mutated and updated such that the next call
/// to [Plan::apply] will use the inserted join operator as the parent node.
///
/// # Errors
///
/// Will never return an error
pub fn join(
    left_plan: RcRefCellPlan<Processed>,
    right_plan: RcRefCellPlan<Processed>,
) -> Result<NotAliasedJoinedPlan<Processed>, PlanError> {
    // TODO: Change this function's return to not use Results
    Ok(NotAliasedJoinedPlan {
        left_plan:  Rc::clone(&left_plan),
        right_plan: Rc::clone(&right_plan),
    })
}

//TODO: Remove [Result] usage since it will never return an error
fn add_join_fragmenter(
    plan: &mut Plan<Processed>,
    alias: &str,
) -> Result<Plan<Processed>, PlanError> {
    if plan.fragment_node_idx == plan.current_cursor_idx {
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

    

    /// Try to add a fragment operator to both left and right plan 
    /// that has a label with the given alias. 
    /// 
    ///
    /// The addition of the aliasing fragment label ensures that if there is no
    /// clash/overlap in the attributes in the mapping tuples from the two plans.
    ///
    pub fn alias(
        &mut self,
        alias: &str,
    ) -> Result<AliasedJoinedPlan<Processed>, PlanError> {
        //TODO: Remove [Result] usage since it will never return an error
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
    
    /// Apply the given operator to **right plan** for the given fragment.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying [Plan::apply_to_fragment] method call
    /// fail.
    pub fn apply_to_right_fragment(
        self,
        operator: Operator,
        op_name: Cow<str>,
        fragment_str: Cow<str>,
    ) -> Result<AliasedJoinedPlan<Processed>, PlanError> {
        let applied_right = self.right_plan.borrow_mut().apply_to_fragment(
            &operator,
            &op_name,
            &fragment_str,
        )?;

        Ok(AliasedJoinedPlan {
            right_plan: Rc::new(RefCell::new(applied_right)),
            ..self
        })
    }

    /// Apply the given operator to **left plan** for the given fragment.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying [Plan::apply_to_fragment] method call
    /// fail.
    pub fn apply_to_left_fragment(
        self,
        operator: Operator,
        op_name: Cow<str>,
        fragment_str: Cow<str>,
    ) -> Result<AliasedJoinedPlan<Processed>, PlanError> {
        let applied_left = self.left_plan.borrow_mut().apply_to_fragment(
            &operator,
            &op_name,
            &fragment_str,
        )?;

        Ok(AliasedJoinedPlan {
            left_plan: Rc::new(RefCell::new(applied_left)),
            ..self
        })
    }

    fn add_join_op_to_plan(&mut self, join_op: Operator) -> Plan<Processed> {
        let fragment_str = &self.alias;
        let node_idx;
        // Curly braces here are required for the block to drop the borrowed left_plan
        {
            let left_plan = self.left_plan.borrow_mut();
            let mut graph = left_plan.graph.borrow_mut();

            let join_node = PlanNode {
                id:       format!("Join_{}", graph.node_count()),
                operator: join_op,
            };

            node_idx = graph.add_node(join_node);

            let left_node = left_plan.current_cursor_idx.unwrap();
            let left_edge = PlanEdge {
                fragment:  fragment_str.to_string(),
                direction: EdgeDirection::Left,
            };

            graph.add_edge(left_node, node_idx, left_edge);
        }

        // Curly braces here are required for the block to drop the borrowed left_plan
        {
            let left_plan = self.left_plan.borrow_mut();
            let graph = &mut left_plan.graph.borrow_mut();

            if let Ok(plan) = self.right_plan.try_borrow_mut() {
                // right_plan is not the same as the left_plan so it can be borrwed
                let right_node = plan.current_cursor_idx.unwrap();
                let right_edge = PlanEdge {
                    fragment:  fragment_str.to_string(),
                    direction: EdgeDirection::Right,
                };

                graph.add_edge(right_node, node_idx, right_edge);
            } else {
                // This case happens when left_plan and right_plan are the same
                // (self-join situation)
                let right_node = left_plan.current_cursor_idx.unwrap();
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

    
    /// Add the attributes from the **left plan** which will be used to check during
    /// the join operation.
    ///
    pub fn where_by<A>(
        &mut self,
        attributes: Vec<A>,
    ) -> Result<WhereByPlan<Processed>, PlanError>
    where
        A: Into<String>,
    {
        // TODO:  Remove the usage of Result since this never returns an error<16-05-25, Min Oo> //
        let left_attributes: Vec<String> =
            attributes.into_iter().map(|a| a.into()).collect();
        Ok(WhereByPlan {
            joined_plan: self.clone(),
            left_attributes,
        })
    }

    /// Returns the cross join of this [`AliasedJoinedPlan<Processed>`].
    pub fn cross_join(&mut self) -> Result<Plan<Processed>, PlanError> {
        // TODO:  Remove the usage of Result since this never returns an error<16-05-25, Min Oo> //
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

    /// Returns the natural join of this [`AliasedJoinedPlan<Processed>`].
    ///
    pub fn natural_join(&mut self) -> Result<Plan<Processed>, PlanError> {
        // TODO:  Remove the usage of Result since this never returns an error<16-05-25, Min Oo> //
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
    
    /// Add the attributes from the **right plan** which will be used to check during
    /// the join operation and apply an equi-join operator to the plan at the end.
    ///
    pub fn equal_to<A>(
        &mut self,
        attributes: Vec<A>,
    ) -> Result<Plan<Processed>, PlanError>
    where
        A: Into<String>,
    {
        // TODO:  Remove the usage of Result since this never returns an error<16-05-25, Min Oo> //
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
