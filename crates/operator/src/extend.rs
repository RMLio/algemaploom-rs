use crate::display::PrettyDisplay;
use crate::extend::function::Function;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod function;
pub mod term_type;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Extend {
    #[serde(flatten)]
    pub extend_pairs: HashMap<String, Function>,
}

impl Extend {
    pub fn extend_with(self, other: Self) -> Self {
        let mut this_pairs = self.extend_pairs;
        let other_pairs = other.extend_pairs;

        this_pairs.extend(other_pairs);

        Extend {
            extend_pairs: this_pairs,
        }
    }
}

impl PrettyDisplay for Extend {
    fn pretty_string(&self) -> anyhow::Result<String> {
        let vec_pairs: Vec<_> = self
            .extend_pairs
            .iter()
            .map(|pair| format!("{} -> {:?}", pair.0, pair.1))
            .collect();

        Ok(format!("Extended pairs: \n {}", vec_pairs.join("\n")))
    }
}
