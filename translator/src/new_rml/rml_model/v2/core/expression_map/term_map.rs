use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

use log::debug;
use sophia_api::prelude::Iri;
use sophia_api::term::{
    BnodeId, FromTerm, IriRef, LanguageTag, SimpleTerm, Term, TermKind,
};
use sophia_term::RcTerm;
use vocab::ToString;

use super::{BaseExpressionMapEnum, ExpressionMapEnum};
use crate::new_rml::extractors::error::ParseError;
use crate::new_rml::extractors::{ExtractorResult, FromVocab};
use crate::new_rml::rml_model::v2::core::TemplateSubString;
use crate::new_rml::rml_model::v2::io::target::LogicalTarget;
use crate::new_rml::rml_model::v2::{
    AttributeAliaser, RefAttributeGetter, TermMapEnum,
};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CommonTermMapInfo {
    pub identifier:      RcTerm,
    pub term_type:       RcTerm,
    pub expression:      ExpressionMapEnum,
    pub logical_targets: Vec<LogicalTarget>,
}

impl AttributeAliaser for CommonTermMapInfo {
    fn alias_attribute(&self, alias: &str) -> Self {
        Self {
            identifier:      self.identifier.clone(),
            term_type:       self.term_type.clone(),
            expression:      self.expression.alias_attribute(alias),
            logical_targets: self.logical_targets.clone(),
        }
    }
}

impl CommonTermMapInfo {
    pub fn from_constant_value<TTerm>(term: TTerm) -> ExtractorResult<Self>
    where
        TTerm: Term + Debug,
    {
        let identifier: RcTerm = match term.kind() {
            TermKind::Literal => {
                RcTerm::from_term(BnodeId::new_unchecked(format!(
                    "{}-{}",
                    term.lexical_form().unwrap(),
                    uuid::Uuid::new_v4()
                )))
            }
            TermKind::Triple => {
                return Err(ParseError::GenericError(
                    "Triple term map type not supported!".to_string(),
                )
                .into())
            }
            TermKind::Variable => {
                return Err(ParseError::GenericError(
                    "Variable term map type not supported!".to_string(),
                )
                .into())
            }
            _ => RcTerm::from_term(term.borrow_term()),
        };

        Ok(Self {
            identifier,
            term_type: termkind_to_rml_rcterm(term.kind())?,
            expression: ExpressionMapEnum::new_constant_term(term),
            logical_targets: Vec::new(),
        })
    }
    pub fn get_constant_value(&self) -> Option<String> {
        if let Ok(base_expr_enum) = self.expression.try_unwrap_base_expression_map_ref()
        {
            match base_expr_enum {
                BaseExpressionMapEnum::Constant(val) => {
                    match self.try_get_term_type_enum().unwrap() {
                        RMLTermTypeKind::IRI => Some(format!("<{}>", val)),
                        RMLTermTypeKind::BlankNode => Some(val.to_string()),
                        RMLTermTypeKind::Literal => {
                            Some(format!("\"{}\"", val))
                        }
                        _ => None,
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_template_string_split(&self) -> Vec<TemplateSubString> {
        self.expression.get_template_string_split()
    }

    pub fn get_ref_attributes(&self) -> HashSet<String> {
        self.expression.get_ref_attributes()
    }

    pub fn is_iri_term_type(&self) -> bool {
        self.term_type == vocab::rml_core::CLASS::IRI.to_rcterm()
            || self.term_type == vocab::r2rml::CLASS::IRI.to_rcterm()
            || self.term_type == vocab::rml_core::CLASS::UNSAFE_IRI.to_rcterm()
            || self.term_type == vocab::rml_core::CLASS::UNSAFE_URI.to_rcterm()
            || self.term_type == vocab::rml_core::CLASS::URI.to_rcterm()
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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct SubjectMap {
    pub term_map_info: CommonTermMapInfo,
    pub classes:       Vec<RcTerm>,
    pub graph_maps:    Vec<TermMapEnum>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PredicateMap {
    pub term_map_info: CommonTermMapInfo,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ObjectMap {
    pub term_map_info: CommonTermMapInfo,
    pub language_map:  Option<ExpressionMapEnum>,
    pub datatype_map:  Option<ExpressionMapEnum>,
}

impl ObjectMap {
    pub fn get_ref_attributes(&self) -> HashSet<String> {
        let mut term_map_attributes = self.term_map_info.get_ref_attributes();
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
    pub term_map_info: CommonTermMapInfo,
}

impl GraphMap {
    pub fn is_default_graph(&self) -> bool {
        if let Some(value) = self.term_map_info.get_constant_value() {
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
            term_map_info: CommonTermMapInfo {
                identifier:      RcTerm::from_term(BnodeId::new_unchecked(
                    uuid::Uuid::new_v4().to_string(),
                )),
                term_type:       vocab::rml_core::CLASS::IRI.to_rcterm(),
                expression:      ExpressionMapEnum::try_new_unknown(
                    vocab::rml_core::PROPERTY::CONSTANT.to_rcterm(),
                    RcTerm::Iri(IriRef::new_unchecked("<defaultGraph>".into())),
                )
                .unwrap(),
                logical_targets: Vec::new(),
            },
        }
    }
}
