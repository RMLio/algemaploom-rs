pub const PREFIX: &str = "ql";
pub const IRI: &str = "http://semweb.mmlab.be/ns/ql#";

pub mod CLASS {
    use super::IRI;
    use crate::PAIR;
    pub const JSONPATH: PAIR = (IRI, "JSONPath");
    pub const CSV: PAIR = (IRI, "CSV");
    pub const XPATH: PAIR = (IRI, "XPath");
    pub const HTML: PAIR = (IRI, "CSS3");
}
