use crate::new_rml::rml_model::v2::core::expression_map::term_map::{
    GraphMap, ObjectMap, PredicateMap, SubjectMap
};

pub mod core;
pub mod fnml;
pub mod io;
pub mod lv;

#[derive(Debug, Clone)]
pub enum TermMapEnum {
    SubjectMap(SubjectMap),
    PredicateMap(PredicateMap),
    ObjectMap(ObjectMap),
    GraphMap(GraphMap), 
}

pub trait AttributeAliaser {
    fn alias_attribute(&self, alias: &str) -> Self;
}
