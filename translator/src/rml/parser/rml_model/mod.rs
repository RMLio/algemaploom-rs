use self::source_target::LogicalSource;
use self::term_map::{GraphMap, ObjectMap, PredicateMap, SubjectMap};

pub mod join;
pub mod source_target;
pub mod term_map;

#[derive(Debug, Clone)]
pub struct Document {
    pub default_base_iri: Option<String>,
    pub triples_maps:     Vec<TriplesMap>,
}

#[derive(Debug, Clone)]
pub struct TriplesMap {
    pub identifier:     String,
    pub logical_source: LogicalSource,
    pub subject_map:    SubjectMap,
    pub po_maps:        Vec<PredicateObjectMap>,
}

impl TriplesMap {
    pub fn contains_ptm(&self) -> bool {
        self.po_maps.iter().any(|pom| pom.contains_ptm())
    }
}

#[derive(Debug, Clone)]
pub struct PredicateObjectMap {
    pub predicate_maps: Vec<PredicateMap>,
    pub object_maps:    Vec<ObjectMap>,
    pub graph_maps:     Vec<GraphMap>,
}

impl PredicateObjectMap {
    pub fn contains_ptm(&self) -> bool {
        self.object_maps.iter().any(|om| om.parent_tm.is_some())
    }
}
