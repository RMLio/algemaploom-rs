use std::collections::HashSet;
use std::hash::Hash;

use log::debug;
use sophia_api::prelude::Iri;
use sophia_api::term::{BnodeId, FromTerm, LanguageTag, SimpleTerm, TermKind};
use sophia_term::RcTerm;
use vocab::ToString;

use super::{ExpressionMap, ExpressionMapTypeEnum};
use crate::new_rml::extractors::error::ParseError;
use crate::new_rml::extractors::{ExtractorResult, FromVocab};
use crate::new_rml::rml_model::v2::core::TemplateSubString;
use crate::new_rml::rml_model::v2::io::target::LogicalTarget;
use crate::new_rml::rml_model::v2::AttributeAliaser;

#[derive(Debug, Clone)]
pub struct TermMap {
    pub identifier:      RcTerm,
    pub term_type:       RcTerm,
    pub expression:      ExpressionMap,
    pub logical_targets: Vec<LogicalTarget>,
}

impl AttributeAliaser for TermMap{
    fn alias_attribute(&self, alias: &str) -> Self {
        Self{
            identifier: self.identifier.clone(),
            term_type: self.term_type.clone(),
            expression: self.expression.alias_attribute(alias),
            logical_targets: self.logical_targets.clone(),
        }
    }
}

impl TermMap {
    pub fn get_constant_value(&self) -> Option<String> {
        match self.expression.get_map_type_enum().unwrap() {
            ExpressionMapTypeEnum::Constant => {
                match self.try_get_term_type_enum().unwrap() {
                    RMLTermTypeKind::IRI => {
                        Some(format!(
                            "<{}>",
                            self.expression.get_value().unwrap()
                        ))
                    }
                    RMLTermTypeKind::BlankNode => {
                        self.expression.get_value().cloned()
                    }
                    RMLTermTypeKind::Literal => {
                        self.expression
                            .get_value()
                            .cloned()
                            .map(|e| format!("\"{}\"", e))
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn get_template_string_split(&self) -> Vec<TemplateSubString> {
        self.expression.get_template_string_split()
    }

    pub fn get_ref_attributes(&self) -> HashSet<String> {
        self.expression.get_ref_attributes()
    }

    pub fn try_get_node(&self) -> Option<RcTerm> {
        if let super::ExpressionMapKind::NonFunction(val) =
            &self.expression.kind
        {
            if self.is_iri_term_type() {
                Some(RcTerm::from_term(Iri::new_unchecked(val.as_str())))
            } else if self.is_bnode_term_type() {
                Some(RcTerm::from_term(BnodeId::new_unchecked(val.as_str())))
            } else {
                Some(RcTerm::from_term(SimpleTerm::LiteralLanguage(
                    val.as_str().into(),
                    LanguageTag::new_unchecked("en".into()),
                )))
            }
        } else {
            None
        }
    }
    pub fn is_iri_term_type(&self) -> bool {
        self.term_type == vocab::rml_core::CLASS::IRI.to_rcterm()
            || self.term_type == vocab::r2rml::CLASS::IRI.to_rcterm()
    }

    pub fn is_bnode_term_type(&self) -> bool {
        self.term_type == vocab::rml_core::CLASS::BLANKNODE.to_rcterm()
            || self.term_type == vocab::r2rml::CLASS::BLANKNODE.to_rcterm()
    }

    pub fn is_literal_term_type(&self) -> bool {
        self.term_type == vocab::rml_core::CLASS::LITERAL.to_rcterm()
            || self.term_type == vocab::r2rml::CLASS::LITERAL.to_rcterm()
    }

    pub fn try_get_term_type_enum(&self) -> ExtractorResult<RMLTermTypeKind> {
        if self.is_literal_term_type() {
            Ok(RMLTermTypeKind::Literal)
        } else if self.is_bnode_term_type() {
            Ok(RMLTermTypeKind::BlankNode)
        } else if self.is_iri_term_type() {
            Ok(RMLTermTypeKind::IRI)
        } else {
            Err(ParseError::GenericError(format!(
                "Term type is not supported yet: {:?}",
                self.term_type
            ))
            .into())
        }
    }
}

impl Eq for TermMap {}

impl PartialEq for TermMap {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.term_type == other.term_type
    }
}

impl Hash for TermMap {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.term_type.hash(state);
        self.expression.hash(state);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RMLTermTypeKind {
    BlankNode,
    IRI,
    Literal,
    UnsafeIRI,
    URI,
    UnsafeURI,
    UnknownT(RcTerm),
    Unknown,
    Variable,
}

pub fn termkind_to_rml_rcterm(kind: TermKind) -> Result<RcTerm, ParseError> {
    match kind {
        TermKind::Iri => Ok(vocab::rml_core::CLASS::IRI.to_rcterm()),
        TermKind::Literal => Ok(vocab::rml_core::CLASS::LITERAL.to_rcterm()),
        TermKind::BlankNode => {
            Ok(vocab::rml_core::CLASS::BLANKNODE.to_rcterm())
        }
        TermKind::Variable => Err(ParseError::GenericError(
            "Sophia's variable term kind do not have a corresponding RML iri"
                .to_string(),
        )),
        TermKind::Triple => {
            Err(ParseError::GenericError(
                "Sophia's triple term kind do not have a corresponding RML iri"
                    .to_string(),
            ))
        }
    }
}

impl From<TermKind> for RMLTermTypeKind {
    fn from(val: TermKind) -> Self {
        match val {
            TermKind::Iri => RMLTermTypeKind::IRI,
            TermKind::Literal => RMLTermTypeKind::Literal,
            TermKind::BlankNode => RMLTermTypeKind::BlankNode,
            TermKind::Variable => RMLTermTypeKind::Variable,
            TermKind::Triple => RMLTermTypeKind::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SubjectMap {
    pub term_map:   TermMap,
    pub classes:    Vec<RcTerm>,
    pub graph_maps: Vec<GraphMap>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PredicateMap {
    pub term_map: TermMap,
}

#[derive(Debug, Clone)]
pub struct ObjectMap {
    pub term_map:     TermMap,
    pub language_map: Option<ExpressionMap>,
    pub datatype_map: Option<ExpressionMap>,
}

impl ObjectMap {
    pub fn get_ref_attributes(&self) -> HashSet<String> {
        let mut term_map_attributes = self.term_map.get_ref_attributes();
        if let Some(dtype_map) = &self.datatype_map {
            term_map_attributes.extend(dtype_map.get_ref_attributes());
        }

        if let Some(langtype_map) = &self.language_map {
            term_map_attributes.extend(langtype_map.get_ref_attributes());
        }

        term_map_attributes
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GraphMap {
    pub term_map: TermMap,
}

impl GraphMap {
    pub fn is_default_graph(&self) -> bool {
        if let Some(value) = self.term_map.get_constant_value() {
            debug!("Graph map's constant iri is {:?}", value);
            value
                == format!(
                    "<{}>",
                    vocab::rml_core::CLASS::DEFAULT_GRAPH.to_string()
                )
        } else {
            false
        }
    }
}

impl Default for GraphMap {
    fn default() -> Self {
        Self {
            term_map: TermMap {
                identifier:      RcTerm::from_term(BnodeId::new_unchecked(
                    uuid::Uuid::new_v4().to_string(),
                )),
                term_type:       vocab::rml_core::CLASS::IRI.to_rcterm(),
                expression:      ExpressionMap::try_new(
                    vocab::rml_core::PROPERTY::CONSTANT.to_rcterm(),
                    "<defaultGraph>".to_string(),
                )
                .unwrap(),
                logical_targets: Vec::new(),
            },
        }
    }
}
