use std::rc::Rc;

use lazy_static::lazy_static;
use regex::Regex;
use sophia_api::prelude::Iri;
use sophia_api::term::{BnodeId, FromTerm, LanguageTag, SimpleTerm, TermKind};
use sophia_term::RcTerm;

use super::{ExpressionMap, ExpressionMapKind, ExpressionValueEnum};
use crate::new_rml::extractors::error::ParseError;
use crate::new_rml::extractors::{ExtractorResult, FromVocab};
use crate::new_rml::rml_model::v2::core::TemplateSubString;
use crate::new_rml::rml_model::v2::io::target::LogicalTarget;
lazy_static! {
    static ref TEMPLATE_REGEX: Regex = Regex::new(r"\{([^\{\}]+)\}").unwrap();
}
fn prefix_attributes_from_template(template: &str, prefix: &str) -> String {
    let sanitized = template.replace("\\{", "\\(").replace("\\}", "\\)");
    TEMPLATE_REGEX
        .replace_all(&sanitized, format!("{{{}_$1}}", prefix))
        .replace("\\(", "\\{")
        .replace("\\)", "\\}")
}

fn split_template_string(template: &str) -> Vec<TemplateSubString> {
    let mut chars = template.chars();

    let mut is_escape;
    let mut current_buf = String::new();
    let mut result = Vec::new();
    while let Some(c) = chars.next() {
        is_escape = c == '\\';
        if is_escape {
            if let Some(c) = chars.next() {
                current_buf.push(c);
            }
        } else if c == '{' {
            result.push(TemplateSubString::NormalString(current_buf.clone()));
            current_buf.clear();
            current_buf.push(c);
        } else if c == '}' {
            current_buf.push(c);
            result.push(TemplateSubString::Attribute(current_buf.clone()));
            current_buf.clear();
        } else {
            current_buf.push(c);
        }
    }
    result
}

#[derive(Debug, Clone)]
pub struct TermMap {
    pub identifier:      RcTerm,
    pub term_type:       RcTerm,
    pub expression:      ExpressionMap,
    pub logical_targets: Vec<LogicalTarget>,
}

impl TermMap {
    pub fn get_template_string_split(&self) -> Vec<TemplateSubString> {
        if let ExpressionValueEnum::Template =
            self.expression.get_value_type_enum().unwrap()
        {
            if let Some(template) = self.expression.get_value() {
                return split_template_string(template);
            }
        }
        vec![]
    }

    pub fn get_ref_attributes(&self) -> Vec<String> {
        let template_attr_vec: Vec<_> = self
            .get_template_string_split()
            .into_iter()
            .filter_map(|sstring| {
                match sstring {
                    TemplateSubString::Attribute(str) => Some(str),
                    TemplateSubString::NormalString(_) => None,
                }
            })
            .collect();

        if !template_attr_vec.is_empty() {
            return template_attr_vec;
        }

        if let ExpressionValueEnum::Reference =
            self.expression.get_value_type_enum().unwrap()
        {
            let val = self.expression.get_value().unwrap();
            vec![val.to_string()]
        } else {
            vec![]
        }
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

impl Into<RMLTermTypeKind> for TermKind {
    fn into(self) -> RMLTermTypeKind {
        match self {
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

#[derive(Debug, Clone)]
pub struct PredicateMap {
    pub term_map: TermMap,
}

#[derive(Debug, Clone)]
pub struct ObjectMap {
    pub term_map:     TermMap,
    pub language_map: Option<ExpressionMap>,
    pub datatype_map: Option<ExpressionMap>,
}

#[derive(Debug, Clone)]
pub struct GraphMap {
    pub term_map: TermMap,
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
