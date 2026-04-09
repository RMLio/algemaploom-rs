use oxigraph::model::{NamedNode, Term};
use vocab::PAIR;

pub mod error;
pub mod io;
pub mod translator;

pub const SUBJECT_ATTR: &str = "subject_attr";
pub const PREDICATE_ATTR: &str = "predicate_attr";
pub const OBJECT_ATTR: &str = "object_attr";
pub const GRAPH_ATTR: &str = "graph_attr";

pub trait FromVocab {
    fn to_named_node(&self) -> NamedNode;
    fn to_term(&self) -> Term;
}

impl FromVocab for PAIR<'_> {
    fn to_named_node(&self) -> NamedNode {
        NamedNode::new_unchecked(format!("{}{}", self.0, self.1))
    }

    fn to_term(&self) -> Term {
        Term::NamedNode(self.to_named_node())
    }
}
