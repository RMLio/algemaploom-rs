use std::collections::HashSet;
use std::rc::Rc;

use chumsky::chain::Chain;
use sophia_term::RcTerm;

use super::core::expression_map::{BaseExpressionMapEnum, ExpressionMapEnum};
use super::core::{AbstractLogicalSourceEnum, JoinCondition, RMLIterable};
use super::io::source::Source;

#[derive(Debug, Clone)]
pub struct LogicalView {
    pub identifier:           RcTerm,
    pub view_on:              Box<AbstractLogicalSourceEnum>,
    pub fields:               Vec<RMLField>,
    pub struct_annotations:   Vec<StructuralAnnotation>,
    pub join_kind_view_pairs: Vec<(RcTerm, LogicalViewJoin)>,
}

impl LogicalView {
    pub fn is_same_logical_view(&self, other: &LogicalView) -> bool {
        (self.identifier == other.identifier)
            || self.view_on.is_same_source(&other.view_on)
    }
    pub fn get_iterable(&self) -> RMLIterable {
        match &*self.view_on {
            AbstractLogicalSourceEnum::LogicalSource(logical_source) => {
                logical_source.iterable.clone()
            }
            AbstractLogicalSourceEnum::LogicalView(logical_view) => {
                logical_view.get_iterable()
            }
        }
    }
    pub fn get_source(&self) -> Source {
        match &*self.view_on {
            AbstractLogicalSourceEnum::LogicalSource(logical_source) => {
                logical_source.source.clone()
            }
            AbstractLogicalSourceEnum::LogicalView(logical_view) => {
                logical_view.get_source()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct RMLField {
    pub name:          String,
    pub kind:          RMLFieldKind,
    pub absolute_name: String,
    pub fields:        Vec<RMLField>,
}

impl RMLField {
    pub fn from_ref_str(ref_str: &str) -> RMLField {
        RMLField {
            name:          ref_str.to_string(),
            absolute_name: ref_str.to_string(),
            kind:          RMLFieldKind::Expression(
                ExpressionMapEnum::new_ref_str(ref_str),
            ),
            fields:        vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub enum RMLFieldKind {
    Iterable(RMLIterable),
    Expression(ExpressionMapEnum),
}

impl RMLFieldKind {
    pub fn is_simple_field(&self) -> bool {
        match self {
            RMLFieldKind::Iterable(_) => true,
            RMLFieldKind::Expression(expression_map_enum) => {
                if let Ok(base_expr) =
                    expression_map_enum.try_unwrap_base_expression_map_ref()
                {
                    matches!(
                        base_expr,
                        BaseExpressionMapEnum::Reference(_)
                            | BaseExpressionMapEnum::Constant(_)
                    )
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct StructuralAnnotation {
    pub kind:          RcTerm,
    pub on_fields:     Vec<RcTerm>,
    pub target_fields: Vec<RcTerm>,
    pub target_views:  Vec<RcTerm>,
}

#[derive(Debug, Clone)]
pub enum StructuralAnnotationKind {
    Unique,
    ForeignKey,
    NotNull,
    IriSafe,
    PrimaryKey,
    Inclusion { target_view: LogicalView },
}

#[derive(Debug, Clone)]
pub struct LogicalViewJoin {
    pub join_condition: JoinCondition,
    pub parent_view:    Rc<LogicalView>,
    pub fields:         Vec<RMLField>,
}

#[derive(Debug, Clone)]
pub enum JoinKind {
    InnerJoin,
    LeftJoin,
}
