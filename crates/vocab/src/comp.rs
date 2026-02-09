pub const PREFIX: &str = "comp";
pub const IRI: &str = "http://semweb.mmlab.be/ns/rml-compression#";

pub mod CLASS {
    use super::IRI;
    use crate::PAIR;

    pub const ZIP: PAIR = (IRI, "zip");
    pub const GZIP: PAIR = (IRI, "gzip");
}
