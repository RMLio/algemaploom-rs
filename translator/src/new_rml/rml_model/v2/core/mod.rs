use expression_map::term_map::{GraphMap, ObjectMap, PredicateMap, SubjectMap};
use expression_map::ExpressionMap;
use sophia_term::RcTerm;

use super::io::source::{
    LogicalSource, RMLReferenceFormulationTypeKind, ReferenceFormulation,
};
use super::lv::LogicalView;
use crate::new_rml::extractors::error::ParseError;
use crate::new_rml::extractors::ExtractorResult;

pub mod expression_map;

#[derive(Debug, Clone)]
pub struct TriplesMap {
    pub identifier:               RcTerm,
    pub base_iri:                 String,
    pub subject_map:              SubjectMap,
    pub predicate_object_map_vec: Vec<PredicateObjectMap>,
    pub logical_source:           AbstractLogicalSource,
}

#[derive(Debug, Clone)]
pub struct PredicateObjectMap {
    pub predicate_map_vec: Vec<PredicateMap>,
    pub object_map_vec:    Vec<ObjectMap>,
    pub ref_object_map:    Vec<RefObjectMap>,
    pub graph_map_vec:     Vec<GraphMap>,
}

#[derive(Debug, Clone)]
pub struct RefObjectMap {
    pub ptm_iri:        RcTerm,
    pub join_condition: Vec<JoinCondition>,
}

#[derive(Debug, Clone)]
pub struct JoinCondition {
    pub parent: ExpressionMap,
    pub child:  ExpressionMap,
}

#[derive(Debug, Clone)]
pub struct RMLIterable {
    pub iterator:              Option<String>,
    pub reference_formulation: Option<ReferenceFormulation>,
}

impl RMLIterable {
    pub fn try_get_ref_formulation_enum(
        &self,
    ) -> ExtractorResult<RMLReferenceFormulationTypeKind> {
        let ref_form = self.reference_formulation.clone().ok_or(
            ParseError::GenericError(format!(
                "iterator does not have a reference formulation: {:?}",
                self
            )),
        )?;
        ref_form.try_into()
    }
}

#[derive(Debug, Clone)]
pub struct AbstractLogicalSource {
    pub iterable:        RMLIterable,
    pub abs_source_enum: AbstractSourceEnum,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum AbstractSourceEnum {
    IOLogicalSource(LogicalSource),
    LogicalView(LogicalView),
}
