pub const PREFIX: &str = "xsd";
pub const IRI: &str = "http://www.w3.org/2001/XMLSchema#";

pub mod TYPE {
    use super::IRI;
    use crate::PAIR;

    pub const XSD_STRING: PAIR = (IRI, "string");
    pub const XSD_INT: PAIR = (IRI, "int"); // signed 32-bit integer
    pub const XSD_INTEGER: PAIR = (IRI, "integer"); // integer value
    pub const XSD_DOUBLE: PAIR = (IRI, "double");
    pub const XSD_LONG: PAIR = (IRI, "long");
    pub const XSD_POSITIVE_INTEGER: PAIR = (IRI, "positiveInteger");
    pub const XSD_BOOLEAN: PAIR = (IRI, "boolean");
    pub const XSD_DATETIME: PAIR = (IRI, "dateTime");

    pub const XSD_ANY: PAIR = (IRI, "any");
}
