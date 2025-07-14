use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

use expression_map::term_map::{GraphMap, ObjectMap, PredicateMap, SubjectMap};
use expression_map::ExpressionMap;
use sophia_term::RcTerm;

use super::io::source::{
    LogicalSource, RMLReferenceFormulationTypeKind, ReferenceFormulation,
    Source,
};
use super::lv::{LogicalView, RMLField};
use crate::new_rml::extractors::error::ParseError;
use crate::new_rml::extractors::ExtractorResult;
use crate::new_rml::rml_model::v2::TermMapEnum;

pub mod expression_map;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TemplateSubString {
    Attribute(String),
    NormalString(String),
}

impl Display for TemplateSubString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TemplateSubString::Attribute(inner) => write!(f, "{}", inner),
            TemplateSubString::NormalString(inner) => write!(f, "{}", inner),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TriplesMap {
    pub identifier:               RcTerm,
    pub base_iri:                 String,
    pub subject_map:              TermMapEnum,
    pub ref_obj_attributes:       Vec<String>,
    pub predicate_object_map_vec: Vec<PredicateObjectMap>,
    pub abs_logical_source:       AbstractLogicalSourceEnum,
}

impl TriplesMap {
    pub fn get_parent_tms_pred_refom_pairs(
        &self,
    ) -> HashSet<(RcTerm, (Vec<PredicateMap>, RefObjectMap, Vec<GraphMap>))>
    {
        let pred_refom_graph_tuples = self
            .predicate_object_map_vec
            .iter()
            .filter(|pom| !pom.ref_object_map.is_empty())
            .map(|pom| {
                (
                    &pom.predicate_map_vec,
                    pom.ref_object_map.iter(),
                    &pom.graph_map_vec,
                )
            });

        let mut result = HashSet::new();
        let iter = pred_refom_graph_tuples.flat_map(
            |(pred_vec, ref_om_iter, graph_vec)| {
                ref_om_iter.map(move |ref_om| {
                    let mut graph_vec = graph_vec.clone();
                    graph_vec.extend(self.subject_map.graph_maps.clone());
                    (
                        ref_om.ptm_iri.clone(),
                        (pred_vec.clone(), ref_om.clone(), graph_vec),
                    )
                })
            },
        );

        result.extend(iter);
        result
    }

    pub fn transform_to_logical_view(&mut self) -> ExtractorResult<()> {
        let abs_ls = &self.abs_logical_source;
        if let AbstractLogicalSourceEnum::LogicalSource(ls) = &abs_ls {
            let mut references =
                self.subject_map.term_map_info.get_ref_attributes();
            let sm_gm_references = self
                .subject_map
                .graph_maps
                .iter()
                .flat_map(|gm| gm.term_map_info.get_ref_attributes());

            let pm_references = self
                .predicate_object_map_vec
                .iter()
                .flat_map(|pom| pom.predicate_map_vec.iter())
                .flat_map(|pm| pm.term_map_info.get_ref_attributes());
            let om_references = self
                .predicate_object_map_vec
                .iter()
                .flat_map(|pom| pom.object_map_vec.iter())
                .flat_map(|om| om.get_ref_attributes());
            let pom_gm_references = self
                .predicate_object_map_vec
                .iter()
                .flat_map(|pom| pom.graph_map_vec.iter())
                .flat_map(|gm| gm.term_map_info.get_ref_attributes());

            references.extend(sm_gm_references);
            references.extend(pom_gm_references);
            references.extend(pm_references);
            references.extend(om_references);
            references.extend(self.ref_obj_attributes.iter().cloned());

            let fields = references
                .iter()
                .map(|ref_val| RMLField::from_ref_str(ref_val))
                .collect();

            let lv = LogicalView {
                identifier: ls.identifier.clone(),
                view_on: Box::new(AbstractLogicalSourceEnum::LogicalSource(
                    ls.clone(),
                )),
                fields,
                struct_annotations: vec![],
                join_kind_view_pairs: vec![],
            };

            //modify the old IOLogicalSource to logical views
            self.abs_logical_source =
                AbstractLogicalSourceEnum::LogicalView(lv);
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PredicateObjectMap {
    pub predicate_map_vec: Vec<TermMapEnum>,
    pub object_map_vec:    Vec<TermMapEnum>,
    pub ref_object_map:    Vec<RefObjectMap>,
    pub graph_map_vec:     Vec<TermMapEnum>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RefObjectMap {
    pub ptm_iri:        RcTerm,
    pub join_condition: Vec<JoinCondition>,
}

#[derive(Debug, Clone)]
pub struct JoinCondition {
    pub parent: ExpressionMap,
    pub child:  ExpressionMap,
}

impl Eq for JoinCondition {}

impl Hash for JoinCondition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.parent.get_value().hash(state);
        self.child.get_value().hash(state);
    }
}

impl PartialEq for JoinCondition {
    fn eq(&self, other: &Self) -> bool {
        self.parent.get_value() == other.parent.get_value()
            && self.child.get_value() == other.child.get_value()
    }
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
    pub abs_source_enum: AbstractLogicalSourceEnum,
}

impl AbstractLogicalSource {
    pub fn get_source(&self) -> Source {
        match &self.abs_source_enum {
            AbstractLogicalSourceEnum::LogicalSource(logical_source) => {
                logical_source.source.clone()
            }
            AbstractLogicalSourceEnum::LogicalView(logical_view) => {
                logical_view.get_source()
            }
        }
    }
    pub fn get_identifier(&self) -> RcTerm {
        let term_ref = match &self.abs_source_enum {
            AbstractLogicalSourceEnum::LogicalSource(logical_source) => {
                &logical_source.identifier
            }
            AbstractLogicalSourceEnum::LogicalView(logical_view) => {
                &logical_view.identifier
            }
        };

        term_ref.clone()
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum AbstractLogicalSourceEnum {
    LogicalSource(LogicalSource),
    LogicalView(LogicalView),
}

impl AbstractLogicalSourceEnum {
    pub fn get_iterable(&self) -> RMLIterable {
        match self {
            AbstractLogicalSourceEnum::LogicalSource(logical_source) => {
                logical_source.iterable.clone()
            }
            AbstractLogicalSourceEnum::LogicalView(logical_view) => {
                logical_view.get_iterable()
            }
        }
    }

    pub fn get_source(&self) -> Source {
        match self {
            AbstractLogicalSourceEnum::LogicalSource(logical_source) => {
                logical_source.source.clone()
            }
            AbstractLogicalSourceEnum::LogicalView(logical_view) => {
                logical_view.get_source()
            }
        }
    }

    pub fn get_identifier(&self) -> RcTerm {
        match self {
            AbstractLogicalSourceEnum::LogicalSource(logical_source) => {
                logical_source.identifier.clone()
            }
            AbstractLogicalSourceEnum::LogicalView(logical_view) => {
                logical_view.identifier.clone()
            }
        }
    }
}
