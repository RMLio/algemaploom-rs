pub const PREFIX: &str = "rmlt";
pub const IRI: &str = "http://semweb.mmlab.be/ns/rml-target#";

pub mod CLASS {
    use super::IRI;
    use crate::PAIR;

    pub const LOGICALTARGET: PAIR = (IRI, "LogicalTarget");
}

pub mod PROPERTY {
    use super::IRI;
    use crate::PAIR;

    pub const TARGET: PAIR = (IRI, "target");
    pub const SERIALIZATION: PAIR = (IRI, "serialization");
    pub const COMPRESSION: PAIR = (IRI, "compression");
    pub const LDES: PAIR = (IRI, "ldes");
    pub const LDESBASE: PAIR = (IRI, "ldesBaseIRI");  
    pub const LDESIMMUTABLE: PAIR = (IRI, "ldesGenerateImmutableIRI");
}
