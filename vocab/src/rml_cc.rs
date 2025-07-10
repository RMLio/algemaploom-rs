pub const PREFIX: &str = "rml";
pub const IRI: &str = "http://w3id.org/rml/";

pub mod PROPERTY {
    use super::IRI;
    use crate::PAIR;

    pub const GATHER: PAIR = (IRI, "gather");
    pub const GATHER_AS: PAIR = (IRI, "gatherAs");
    pub const STRATEGY: PAIR = (IRI, "strategy");
    pub const CARTESIAN_PRODUCT: PAIR = (IRI, "cartesianProduct"); 
    pub const APPEND: PAIR = (IRI, "append"); 
}

pub mod CLASS {
    use super::IRI;
    use crate::PAIR;

    pub const GATHER_MAP: PAIR = (IRI, "GatherMap");
    pub const STRATEGY: PAIR = (IRI, "Strategy");
}
