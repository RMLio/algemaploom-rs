// Hypermedia Controls Ontology (HCTL) vocabulary
// Used in conjunction with WoT Thing Descriptions
// https://www.w3.org/2019/wot/hypermedia

pub const PREFIX: &str = "hctl";
pub const IRI: &str = "https://www.w3.org/2019/wot/hypermedia#";

pub mod CLASS {
    use super::IRI;
    use crate::PAIR;
    
    pub const FORM: PAIR = (IRI, "Form");
    pub const LINK: PAIR = (IRI, "Link");
}

pub mod PROPERTY {
    use super::IRI;
    use crate::PAIR;
    
    pub const HAS_TARGET: PAIR = (IRI, "hasTarget");
    pub const FOR_CONTENT_TYPE: PAIR = (IRI, "forContentType");
    pub const FOR_SUB_PROTOCOL: PAIR = (IRI, "forSubProtocol");
    pub const HAS_OPERATION_TYPE: PAIR = (IRI, "hasOperationType");
}
