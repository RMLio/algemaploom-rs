use std::{collections::HashSet, ops::Deref};

use derive_more::{IsVariant, TryUnwrap, Unwrap};

use crate::new_rml::rml_model::v2::core::expression_map::term_map::{
    CommonTermMapInfo, GraphMap, ObjectMap, PredicateMap, SubjectMap,
};

pub mod core;
pub mod fnml;
pub mod io;
pub mod lv;

#[derive(Debug, Clone, TryUnwrap, IsVariant, Unwrap, Hash, PartialEq, Eq)]
#[unwrap(ref)]
#[try_unwrap(ref)]
pub enum TermMapEnum {
    SubjectMap(SubjectMap),
    PredicateMap(PredicateMap),
    ObjectMap(ObjectMap),
    GraphMap(GraphMap),
}

impl AsRef<CommonTermMapInfo> for TermMapEnum {
    fn as_ref(&self) -> &CommonTermMapInfo {
        match self {
            TermMapEnum::SubjectMap(subject_map) => &subject_map.term_map_info,
            TermMapEnum::PredicateMap(predicate_map) => {
                &predicate_map.term_map_info
            }
            TermMapEnum::ObjectMap(object_map) => &object_map.term_map_info,
            TermMapEnum::GraphMap(graph_map) => &graph_map.term_map_info,
        }
    }
}

pub trait AttributeAliaser {
    fn alias_attribute(&self, alias: &str) -> Self;
}
pub trait RefAttributeGetter {
    fn get_ref_attributes(&self) -> HashSet<String>;
}
