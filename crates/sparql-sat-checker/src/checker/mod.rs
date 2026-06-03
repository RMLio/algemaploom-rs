use crate::extractor::rml::trmap_sub_expression::TrMapSubExpression;

use error::{CheckerError, CheckerErrorKind, CheckerResult};
use operator::extend::function::Function;
use operator::extend::term_type::TermType;
use plan::data_type::DiGraphOperators;
use spargebra::term::{Literal, NamedNode, NamedNodePattern, TermPattern, TriplePattern};
use translator_normalized_rml::{OBJECT_ATTR, PREDICATE_ATTR, SUBJECT_ATTR};
use util::{extract_extend_expr_from_operator, spo_from_trmap_sub_expression};

pub mod error;
pub mod util;

#[derive(Debug, PartialEq, Eq)]
pub enum SatisfiableState {
    Maybe,
    No,
}

fn subject_satisfiable_over_pattern(
    extend_expr: &Function,
    term_pattern: &TermPattern,
) -> CheckerResult<SatisfiableState> {
    match term_pattern {
        TermPattern::NamedNode(named_node) => extexpr_satisfiable_over_iri(extend_expr, named_node),
        TermPattern::Literal(_literal) => Ok(SatisfiableState::No),
        _ => Ok(SatisfiableState::Maybe),
    }
}

fn predicate_satisfiable_over_pattern(
    extend_expr: &Function,
    term_pattern: &NamedNodePattern,
) -> CheckerResult<SatisfiableState> {
    match term_pattern {
        NamedNodePattern::NamedNode(named_node) => {
            extexpr_satisfiable_over_iri(extend_expr, named_node)
        }
        NamedNodePattern::Variable(_variable) => Ok(SatisfiableState::Maybe),
    }
}

fn object_satisfiable_over_pattern(
    extend_expr: &Function,
    term_pattern: &TermPattern,
) -> CheckerResult<SatisfiableState> {
    match term_pattern {
        TermPattern::NamedNode(named_node) => extexpr_satisfiable_over_iri(extend_expr, named_node),
        TermPattern::Literal(literal) => extexpr_satisfiable_over_literal(extend_expr, literal),
        _ => Ok(SatisfiableState::Maybe),
    }
}

fn concat_to_regex_str(extend: &Function, acc: &mut String) {
    match extend {
        Function::TypedConstant {
            value,
            term_type: _,
        } => {
            acc.push_str(&regex::escape(value));
        }
        Function::Constant { value } => {
            acc.push_str(&regex::escape(value));
        }
        Function::Reference { value: _ } => {
            acc.push_str(".+");
        }
        Function::Concatenate {
            left_value,
            separator,
            right_value,
        } => {
            concat_to_regex_str(left_value, acc);
            if !separator.is_empty() {
                acc.push_str(&regex::escape(separator));
            }
            concat_to_regex_str(right_value, acc);
        }
        _ => (),
    }
}
fn create_regex_check_match(
    inner_function: &std::rc::Rc<Function>,
    value: &str,
) -> Result<SatisfiableState, CheckerError> {
    let mut regex_expr = String::from("^");
    concat_to_regex_str(inner_function, &mut regex_expr);
    regex_expr.push('$');
    let regex = regex::Regex::new(&regex_expr)
        .map_err(|err| CheckerErrorKind::RegexError {
            iri: value.to_string(),
            regex_expr,
            err,
        })
        .map_err(|kind| CheckerError { kind })?;
    if !regex.is_match(value) {
        Ok(SatisfiableState::No)
    } else {
        Ok(SatisfiableState::Maybe)
    }
}

fn extexpr_satisfiable_over_iri(
    extend: &Function,
    iri: &NamedNode,
) -> CheckerResult<SatisfiableState> {
    match extend {
        Function::TypedConstant { value, term_type } => {
            if (*term_type != TermType::IRI)
                || (*term_type == TermType::IRI && value != iri.as_str())
            {
                Ok(SatisfiableState::No)
            }  else {
                Ok(SatisfiableState::Maybe)
            }
        }
        Function::Iri {
            base_iri: _,
            inner_function,
        } => {
            let iri_str = iri.as_str();
            create_regex_check_match(inner_function, iri_str)
        }
        _ => Ok(SatisfiableState::No),
    }
}

fn extexpr_satisfiable_over_literal(
    extend_expr: &Function,
    literal: &Literal,
) -> CheckerResult<SatisfiableState> {
    let lit_value = literal.value();
    match extend_expr {
        Function::TypedConstant { value, term_type } => {
            if (*term_type != TermType::Literal)
                || (value != lit_value && *term_type == TermType::Literal)
            {
                Ok(SatisfiableState::No)
            } else {
                Ok(SatisfiableState::Maybe)
            }
        }
        Function::Literal {
            inner_function,
            dtype_function,
            // ignore language type maps since we do not handle them in the ESWC paper
            langtype_function: _,
        } => {
            let lit_dtype = literal.datatype().into_owned();
            // Dtype_function here MUST BE of type Function::TypedConstant
            let dtype_function = dtype_function.as_ref().unwrap();
            if let Ok(SatisfiableState::Maybe) =
                extexpr_satisfiable_over_iri(dtype_function, &lit_dtype)
            {
                create_regex_check_match(inner_function, lit_value)
            } else {
                Ok(SatisfiableState::No)
            }
        }
        Function::Iri {
            base_iri: _,
            inner_function: _,
        } => Ok(SatisfiableState::No),
        Function::BlankNode { inner_function: _ } => Ok(SatisfiableState::No),
        _ => Ok(SatisfiableState::Maybe),
    }
}

fn is_trmap_satisfiable_over_tp(
    graph: &DiGraphOperators,
    trmap: &TrMapSubExpression,
    tp: &TriplePattern,
) -> CheckerResult<SatisfiableState> {
    let subject_index = trmap.subject_index;
    let subject_operator = &graph.node_weight(subject_index).unwrap().operator;
    let subject_extend = extract_extend_expr_from_operator(subject_operator, SUBJECT_ATTR)?;
    let subj_sat = subject_satisfiable_over_pattern(subject_extend, &tp.subject)?;

    if subj_sat == SatisfiableState::No {
        return Ok(SatisfiableState::No);
    }

    let predicate_index = trmap.predicate_index;
    let predicate_operator = &graph.node_weight(predicate_index).unwrap().operator;
    let predicate_extend = extract_extend_expr_from_operator(predicate_operator, PREDICATE_ATTR)?;
    let pred_sat = predicate_satisfiable_over_pattern(predicate_extend, &tp.predicate)?;
    if pred_sat == SatisfiableState::No {
        return Ok(SatisfiableState::No);
    }

    let object_index = trmap.object_index;
    let object_operator = &graph.node_weight(object_index).unwrap().operator;
    let object_extend = extract_extend_expr_from_operator(object_operator, OBJECT_ATTR)?;
    let obj_sat = object_satisfiable_over_pattern(object_extend, &tp.object)?;
    if obj_sat == SatisfiableState::No {
        return Ok(SatisfiableState::No);
    }

    Ok(SatisfiableState::Maybe)
}

pub fn maybe_satisfiable_trmaps_sub_expressions<'a>(
    graph: &'a DiGraphOperators,
    trmaps: &'a [TrMapSubExpression],
    tpatterns: &'a [TriplePattern],
) -> Vec<&'a TrMapSubExpression> {
    trmaps
        .iter()
        .filter(|trmap| {
            let res = tpatterns.iter().any(|pattern| {
                matches!(
                    is_trmap_satisfiable_over_tp(graph, trmap, pattern),
                    Ok(SatisfiableState::Maybe)
                )
            });

            log::debug!(
                "Satisfiable {:?} for Checking for satisfiability between trmap {:#?} and triple patterns {:#?}",
                res, 
                spo_from_trmap_sub_expression(graph, trmap),
                tpatterns
            );
            res
        })
        .collect()
}
