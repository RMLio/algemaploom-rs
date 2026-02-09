use std::collections::HashSet;
use std::ops::Deref;

use derive_more::{IsVariant, TryUnwrap, Unwrap};

use crate::rml_model::v2::core::expression_map::term_map::{
    CommonTermMapInfo, GraphMap, ObjectMap, PredicateMap, SubjectMap,
};
use crate::rml_model::v2::fnml::FunctionMap;

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
    FunctionMap(FunctionMap),
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
            TermMapEnum::FunctionMap(function_map) => {
                &function_map.term_map_info
            }
        }
    }
}

impl RefAttributeGetter for TermMapEnum {
    fn get_ref_attributes(&self) -> HashSet<String> {
        match self {
            TermMapEnum::SubjectMap(subject_map) => {
                let mut result = subject_map.term_map_info.get_ref_attributes(); 
                result.extend(subject_map.graph_maps.iter().flat_map(|tm| tm.get_ref_attributes()));
                result
            }
            TermMapEnum::PredicateMap(predicate_map) => {
                predicate_map.term_map_info.get_ref_attributes()
            }
            TermMapEnum::ObjectMap(object_map) => {
                object_map.term_map_info.get_ref_attributes()
            }
            TermMapEnum::GraphMap(graph_map) => {
                graph_map.term_map_info.get_ref_attributes()
            }
            TermMapEnum::FunctionMap(function_map) => {
                function_map.term_map_info.get_ref_attributes()
            }
        }
    }
}

pub trait AttributeAliaser {
    fn alias_attribute(&self, alias: &str) -> Self;
}
pub trait RefAttributeGetter {
    fn get_ref_attributes(&self) -> HashSet<String>;
}
