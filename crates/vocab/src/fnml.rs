pub const PREFIX: &str = "fnml";
pub const IRI: &str = "http://semweb.mmlab.be/ns/fnml#";

pub mod PROPERTY {
    use super::*;
    use crate::PAIR;

    pub const FUNCTION_VALUE: PAIR = (IRI, "functionValue");
}
