pub const PREFIX: &str = "tree";
pub const IRI: &str = "https://w3id.org/tree#";

pub mod class {
    use super::IRI;
    use crate::PAIR;

    pub const COLLECTION: PAIR = (IRI, "Collection");
}

pub mod property {
    use super::IRI;
    use crate::PAIR;

    pub const SHAPE: PAIR = (IRI, "shape");
} 