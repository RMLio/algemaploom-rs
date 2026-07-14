use crate::extend::function::RcExtendFunction;
use crate::formats::ReferenceFormulation;
use serde::{Deserialize, Serialize};

// TODO: Turn Field and Iterator into an Enum since a field itself can be an iterator!
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    pub alias:                 String,
    pub absolute_path:         Option<String>,
    pub expression:            RcExtendFunction,
    pub iterator:              Option<String>,
    pub reference_formulation: ReferenceFormulation,
    pub inner_fields:          Vec<Field>,
}