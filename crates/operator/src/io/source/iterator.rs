use crate::formats::ReferenceFormulation;
use crate::io::source::field::Field;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

// TODO: Turn Field and Iterator into an Enum since a field itself can be an iterator!
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct Iterator {
    pub reference:             Option<String>,
    pub reference_formulation: ReferenceFormulation,
    pub fields:                Vec<Field>,
    pub alias:                 Option<String>,
}

impl Iterator {
    pub fn merge(&mut self, other: &Iterator) {
        self.fields.extend(other.fields.iter().cloned());
    }
}

impl Hash for Iterator {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.reference.hash(state);
        self.reference_formulation.hash(state);
    }
}
