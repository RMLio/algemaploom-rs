pub const PREFIX: &str = "rr";
pub const IRI: &str = "http://www.w3.org/ns/r2rml#";

pub mod PROPERTY {
    use super::IRI;
    use crate::PAIR;

    pub const PREDICATEOBJECTMAP: PAIR = (IRI, "predicateObjectMap");
    pub const PREDICATE: PAIR = (IRI, "predicate");
    pub const PREDICATEMAP: PAIR = (IRI, "predicateMap");
    pub const OBJECT: PAIR = (IRI, "object");
    pub const OBJECTMAP: PAIR = (IRI, "objectMap");
    pub const TRIPLESMAP: PAIR = (IRI, "triplesMap");
    pub const SUBJECTMAP: PAIR = (IRI, "subjectMap");
    pub const SUBJECT: PAIR = (IRI, "subject");
    pub const CONSTANT: PAIR = (IRI, "constant");
    pub const TEMPLATE: PAIR = (IRI, "template");
    pub const TERMTYPE: PAIR = (IRI, "termType");
    pub const COLUMN: PAIR = (IRI, "column");
    pub const CLASS: PAIR = (IRI, "class");
    pub const PARENTTRIPLESMAP: PAIR = (IRI, "parentTriplesMap");
    pub const JOINCONDITION: PAIR = (IRI, "joinCondition");
    pub const PARENT: PAIR = (IRI, "parent");
    pub const CHILD: PAIR = (IRI, "child");
    pub const GRAPH: PAIR = (IRI, "graph");
    pub const GRAPHMAP: PAIR = (IRI, "graphMap");
    pub const DATATYPE: PAIR = (IRI, "datatype");
    pub const LANGUAGE: PAIR = (IRI, "language");
    pub const DEFAULTGRAPH: PAIR = (IRI, "defaultGraph");
    pub const TABLE: PAIR = (IRI, "table");

    pub const SQLVERSION: PAIR = (IRI, "sqlVersion");
}

pub mod CLASS {
    use super::IRI as SUPER_IRI;
    use crate::PAIR;
    pub const PREDICATEOBJECTMAP: PAIR = (SUPER_IRI, "PredicateObjectMap");
    pub const OBJECTMAP: PAIR = (SUPER_IRI, "ObjectMap");
    pub const TRIPLESMAP: PAIR = (SUPER_IRI, "TriplesMap");
    pub const IRI: PAIR = (SUPER_IRI, "IRI");
    pub const BLANKNODE: PAIR = (SUPER_IRI, "BlankNode");
    pub const LITERAL: PAIR = (SUPER_IRI, "Literal");
}
