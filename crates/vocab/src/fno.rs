pub const PREFIX: &str = "fno";
pub const IRI: &str = "https://w3id.org/function/ontology#";

pub mod PROPERTY {

    use super::*;
    use crate::PAIR;

    pub const EXECUTES: PAIR = (IRI, "executes");
}
