use operator::Operator;

mod checker;
mod extractor;
mod serializer;
mod tests;

fn is_graph(op: &Operator) -> bool {
    match op {
        Operator::ExtendOp { config } => config.extend_pairs.contains_key(GRAPH_ATTR),
        _ => false,
    }
}
pub use checker::maybe_satisfiable_trmaps_sub_expressions;
pub use extractor::rml::{
    extract_trmap_sub_expressions_from_plan, trmap_sub_expression::log_delta_trmap_sub_expressions,
};
pub use extractor::sparql::{
    extract_triple_patterns_from_sparql_file, extract_triple_patterns_from_sparql_str,
};
pub use plan::util as plan_util;
pub use plan::{prune_graph_using_trmap_subexprs, reverse_rml::serialize_trmap_expr_to_rml};
use translator::GRAPH_ATTR;
