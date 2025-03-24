pub const PREFIX: &str = "formats";
pub const IRI: &str = "http://www.w3.org/ns/formats/";

pub mod CLASS {
    use super::IRI;
    use crate::PAIR;

    pub const NTRIPLES: PAIR = (IRI, "N-Triples");
    pub const NQUADS: PAIR = (IRI, "N-Quads");
    pub const JSONLD: PAIR = (IRI, "JSON-LD");
    pub const N3: PAIR = (IRI, "N3");
    pub const LD_PATCH: PAIR = (IRI, "LD_Patch");
    pub const MICRODATA: PAIR = (IRI, "microdata");
    pub const OWL_XML: PAIR = (IRI, "OWL_XML");
    pub const OWL_FUNCTIONAL: PAIR = (IRI, "OWL_Functional");
    pub const OWL_MANCHESTER: PAIR = (IRI, "OWL_Manchester");
    pub const POWDER: PAIR = (IRI, "POWDER");
    pub const POWDER_S: PAIR = (IRI, "POWDER-S");
    pub const TURTLE: PAIR = (IRI, "Turtle");
    pub const TRIG: PAIR = (IRI, "TriG");
    pub const RDFA: PAIR = (IRI, "RDFa");
    pub const RDF_JSON: PAIR = (IRI, "RDF_JSON");
    pub const RDF_XML: PAIR = (IRI, "RDF_XML");
    pub const SPARQL_XML: PAIR = (IRI, "SPARQL_Results_XML");
    pub const SPARQL_JSON: PAIR = (IRI, "SPARQL_Results_JSON");
    pub const SPARQL_CSV: PAIR = (IRI, "SPARQL_Results_CSV");
    pub const SPARQL_TSV: PAIR = (IRI, "SPARQL_Results_TSV");
}
