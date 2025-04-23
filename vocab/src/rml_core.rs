pub const PREFIX: &str = "rml";

pub const IRI: &str = "http://w3id.org/rml/";

pub mod PROPERTY {
    use super::IRI;
    use crate::PAIR;

    // From Expression Map
    pub const REFERENCE: PAIR = (IRI, "reference");
    pub const CONSTANT: PAIR = (IRI, "constant");
    pub const TEMPLATE: PAIR = (IRI, "template");

    // From Term Map
    pub const LOGICAL_TARGET: PAIR = (IRI, "logicalTarget");
    pub const TERMTYPE: PAIR = (IRI, "termType");

    // From Object Map
    pub const DATATYPE_MAP: PAIR = (IRI, "datatypeMap");
    pub const LANGUAGE_MAP: PAIR = (IRI, "languageMap");
    pub const DATATYPE: PAIR = (IRI, "datatype");
    pub const LANGUAGE: PAIR = (IRI, "language");

    // From Reference Object Map
    pub const JOIN_CONDITION: PAIR = (IRI, "joinCondition");
    pub const PARENT_TRIPLES_MAP: PAIR = (IRI, "parentTriplesMap");

    // From Join Condition
    pub const PARENT_MAP: PAIR = (IRI, "parentMap");
    pub const CHILD_MAP: PAIR = (IRI, "childMap");
    pub const PARENT: PAIR = (IRI, "parent");
    pub const CHILD: PAIR = (IRI, "child");

    // From Predicate Object Map
    pub const PREDICATE: PAIR = (IRI, "predicate");
    pub const OBJECT: PAIR = (IRI, "object");
    pub const PREDICATE_MAP: PAIR = (IRI, "predicateMap");
    pub const OBJECT_MAP: PAIR = (IRI, "objectMap");

    // From Subject Map
    pub const CLASS: PAIR = (IRI, "class");

    // From Triples Map
    pub const SUBJECT: PAIR = (IRI, "subject");
    pub const SUBJECT_MAP: PAIR = (IRI, "subjectMap");
    pub const PREDICATE_OBJECT_MAP: PAIR = (IRI, "predicateObjectMap");
    pub const LOGICAL_SOURCE: PAIR = (IRI, "logicalSource");

    // From Abstract Logical Source
    pub const ITERATOR: PAIR = (IRI, "iterator");
    pub const REFERENCE_FORMULATION: PAIR = (IRI, "referenceFormulation");

    pub const GRAPH: PAIR = (IRI, "graph");
    pub const GRAPH_MAP: PAIR = (IRI, "graphMap");
}

pub mod CLASS {
    use super::IRI as SUPER_IRI;
    use crate::PAIR;

    pub const PREDICATEOBJECT_MAP: PAIR = (SUPER_IRI, "PredicateObjectMap");
    pub const OBJECT_MAP: PAIR = (SUPER_IRI, "ObjectMap");
    pub const SUBJECT_MAP: PAIR = (SUPER_IRI, "SubjectMap");
    pub const TRIPLES_MAP: PAIR = (SUPER_IRI, "TriplesMap");
    pub const IRI: PAIR = (SUPER_IRI, "IRI");
    pub const BLANKNODE: PAIR = (SUPER_IRI, "BlankNode");
    pub const LITERAL: PAIR = (SUPER_IRI, "Literal");
    pub const DEFAULT_GRAPH: PAIR = (SUPER_IRI, "defaultGraph");
}
