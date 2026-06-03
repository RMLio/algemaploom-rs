use crate::display::PrettyDisplay;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Projection {
    pub projection_attributes: HashSet<String>,
}

impl PrettyDisplay for Projection {
    fn pretty_string(&self) -> anyhow::Result<String> {
        let attributes = self
            .projection_attributes
            .iter()
            .fold(String::new(), |acc, val| acc + val + ", ");

        Ok(format!("Projected attributes: {}", attributes))
    }
}

impl Hash for Projection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for val in self.projection_attributes.iter() {
            val.hash(state);
        }
    }
}