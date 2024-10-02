pub const PREFIX: &str = "rml";
pub const IRI: &str = "http://semweb.mmlab.be/ns/rml#";

pub mod PROPERTY {

    use super::IRI;
    use crate::PAIR;

    pub const REFERENCE: PAIR = (IRI, "reference");
    pub const LOGICALSOURCE: PAIR = (IRI, "logicalSource");
    pub const ITERATOR: PAIR = (IRI, "iterator");
    pub const REFERENCEFORMULATION: PAIR = (IRI, "referenceFormulation");
    pub const SOURCE: PAIR = (IRI, "source");
    pub const LOGICALTARGET: PAIR = (IRI, "logicalTarget");
    pub const QUERY: PAIR = (IRI, "query");
}
