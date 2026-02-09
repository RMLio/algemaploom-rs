use operator::{Extend, Function, Operator};
use plan::data_type::{DiGraphOperators, PlanNode};

use crate::{
    checker::error::{CheckerError, CheckerErrorKind},
    extractor::rml::trmap_sub_expression::TrMapSubExpression,
};

use super::error::CheckerResult;

pub fn extract_extend_expr_from_operator<'a>(
    operator: &'a Operator,
    attribute: &str,
) -> CheckerResult<&'a Function> {
    match operator {
        Operator::ExtendOp { config } => config.extend_pairs.get(attribute).ok_or(CheckerError {
            kind: CheckerErrorKind::InvalidOperatorConversion(format!(
                "cannot extract extend expression from {:?} for attribute {}",
                operator, attribute
            )),
        }),
        _ => Err(CheckerError {
            kind: CheckerErrorKind::InvalidOperatorConversion(format!(
                "cannot convert {:?} into an Extend Operator",
                operator
            )),
        }),
    }
}

pub fn spo_from_trmap_sub_expression<'a>(
    graph: &'a DiGraphOperators,
    trmap: &'a TrMapSubExpression,
) -> (&'a PlanNode, &'a PlanNode, &'a PlanNode) {
    (
        &graph[trmap.subject_index],
        &graph[trmap.predicate_index],
        &graph[trmap.object_index],
    )
}
