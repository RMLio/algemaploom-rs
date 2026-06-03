use crate::display::PrettyDisplay;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// Enums to denote different types of joins
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum JoinType {
    LeftJoin,
    RightJoin,
    InnerJoin,
    CrossJoin,
    NaturalJoin,
}

/// Type of predicate function used in a θ-join operator
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum PredicateType {
    Greater,
    GEqual,
    Less,
    LEqual,
    Equal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Join {
    pub left_right_attr_pairs: Vec<(String, String)>,
    pub join_type:             JoinType,
    pub predicate_type:        PredicateType,
}

impl Default for Join {
    fn default() -> Self {
        Self {
            left_right_attr_pairs: Default::default(),
            join_type:             JoinType::InnerJoin,
            predicate_type:        PredicateType::Equal,
        }
    }
}

impl Hash for Join {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.left_right_attr_pairs.hash(state);
        self.join_type.hash(state);
        self.predicate_type.hash(state);
    }
}

impl PrettyDisplay for Join {
    fn pretty_string(&self) -> anyhow::Result<String> {
        let result = format!(
            "type: {:?}\npredicate_type: {:?}\nattribute_pairs: {}\n",
            self.join_type,
            self.predicate_type,
            serde_json::to_string_pretty(&self.left_right_attr_pairs)?,
        );

        Ok(result)
    }
}