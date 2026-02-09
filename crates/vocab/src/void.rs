pub const PREFIX: &str = "void";
pub const IRI: &str = "http://rdfs.org/ns/void#";

pub mod PROPERTY {
    use super::IRI;
    use crate::PAIR;

    pub const DATA_DUMP: PAIR = (IRI, "dataDump");
    pub const URI_LOOKUP_ENDPOINT: PAIR = (IRI, "uriLookupEndpoint");
    pub const SPARQL_ENDPOINT: PAIR = (IRI, "sparqlEndpoint");
}

pub mod CLASS {
    use super::IRI;
    use crate::PAIR;

    pub const DATASET: PAIR = (IRI, "Dataset");
    pub const LINKSET: PAIR = (IRI, "Linkset");
}
