use std::collections::HashMap;

use crate::value::Value;
pub type SolutionMapping = HashMap<String, Value>;
pub type SolutionSequence = Vec<SolutionMapping>;
#[deprecated(note="This is a very old definition of mapping tuples. Use SolutionMapping for now instead")]
pub type MappingTuple = HashMap<String, SolutionSequence>;
