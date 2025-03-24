use std::rc::Rc;

use sophia_term::RcTerm;

use super::core::expression_map::ExpressionMap;
use super::core::{JoinCondition, RMLIterable};
use super::io::source::LogicalSource;

#[derive(Debug, Clone)]
pub struct LogicalView {
    pub identifier:           RcTerm,
    pub view_on:              LogicalSource,
    pub fields:               Vec<RMLField>,
    pub struct_annotations:   Vec<StructuralAnnotation>,
    pub join_kind_view_pairs: Vec<(RcTerm, LogicalViewJoin)>,
}

#[derive(Debug, Clone)]
pub struct RMLField {
    pub name:   String,
    pub kind:   RMLFieldKind,
    pub fields: Vec<RMLField>,
}

#[derive(Debug, Clone)]
pub enum RMLFieldKind {
    Iterable(RMLIterable),
    Expression(ExpressionMap),
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
