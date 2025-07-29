use std::collections::HashSet;

use derive_more::{TryUnwrap, Unwrap};
use sophia_api::term::{Term, TermKind};
use sophia_term::RcTerm;

use crate::new_rml::extractors::error::ParseError;
use crate::new_rml::extractors::{ExtractorResult, FromVocab};
use crate::new_rml::rml_model::v2::core::TemplateSubString;
use crate::new_rml::rml_model::v2::fnml::FunctionExecution;
use crate::new_rml::rml_model::v2::AttributeAliaser;

pub mod term_map;

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
        } else if c == '}' {
            result.push(TemplateSubString::Attribute(current_buf.clone()));
            current_buf.clear();
        } else {
            current_buf.push(c);
        }
    }

    if !current_buf.is_empty() {
        result.push(TemplateSubString::NormalString(current_buf));
    }
    result
}

pub trait RefAttributeGetter {
    fn get_ref_attributes(&self) -> HashSet<String>;
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum BaseExpressionMapEnum {
    Template(String),
    Reference(String),
    Constant(String),
    Unknown { type_iri: RcTerm, term_val: RcTerm },
}
impl BaseExpressionMapEnum {
    pub fn get_template_string_split(&self) -> Vec<TemplateSubString> {
        match self {
            BaseExpressionMapEnum::Template(template) => {
                split_template_string(&template)
            }
            _ => Vec::new(),
        }
    }
}

impl RefAttributeGetter for BaseExpressionMapEnum {
    fn get_ref_attributes(&self) -> HashSet<String> {
        let template_attr_vec: HashSet<_> = self
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

        match self {
            BaseExpressionMapEnum::Reference(ref_val) => {
                HashSet::from([ref_val.to_string()])
            }
            _ => HashSet::new(),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, TryUnwrap, Unwrap)]
#[unwrap(ref)]
#[try_unwrap(ref)]
pub enum ExpressionMapEnum {
    BaseExpressionMap(BaseExpressionMapEnum),
    //FunctionExpressionMap(FunctionExpressionMap),
    // pub map_type_pred_iri: RcTerm,
    // pub kind:              ExpressionMapKind,
}

impl AsRef<BaseExpressionMapEnum> for ExpressionMapEnum {
    fn as_ref(&self) -> &BaseExpressionMapEnum {
        todo!()
    }
}

impl From<Vec<TemplateSubString>> for ExpressionMapEnum {
    fn from(value: Vec<TemplateSubString>) -> Self {
        let template_string: String =
            value.into_iter().map(|val| val.to_string()).collect();
        ExpressionMapEnum::BaseExpressionMap(BaseExpressionMapEnum::Template(
            template_string,
        ))
    }
}

impl AttributeAliaser for Vec<TemplateSubString> {
    fn alias_attribute(&self, alias: &str) -> Self {
        self.iter()
            .map(|val| {
                match val {
                    TemplateSubString::Attribute(attr) => {
                        TemplateSubString::Attribute(format!(
                            "{}.{}",
                            alias, attr
                        ))
                    }
                    _ => val.clone(),
                }
            })
            .collect()
    }
}

impl ExpressionMapEnum {
    pub fn from_template_str(template: &str) -> ExpressionMapEnum {
        let template_expr =
            BaseExpressionMapEnum::Template(template.to_string());
        Self::BaseExpressionMap(template_expr)
    }

    pub fn from_const_str(const_str: &str) -> ExpressionMapEnum {
        Self::BaseExpressionMap(BaseExpressionMapEnum::Constant(
            const_str.to_string(),
        ))
    }

    pub fn from_ref_str(ref_str: &str) -> ExpressionMapEnum {
        Self::BaseExpressionMap(BaseExpressionMapEnum::Reference(
            ref_str.to_string(),
        ))
    }

    pub fn try_new_unknown(
        value_pred: RcTerm,
        value: RcTerm,
    ) -> Result<Self, ParseError> {
        if value_pred.kind() != TermKind::Iri {
            return Err(ParseError::GenericError(format!(
                "Expression map contains an invalid predicate term {:?}",
                value_pred
            )));
        }
        Ok(Self::BaseExpressionMap(BaseExpressionMapEnum::Unknown {
            type_iri: value_pred,
            term_val: value,
        }))
    }
}

impl AttributeAliaser for ExpressionMapEnum {
    fn alias_attribute(&self, alias: &str) -> Self {
        let aliased_kind = match self.get_map_type_enum().unwrap() {
            ExpressionMapTypeEnum::Template => {
                let template_split =
                    self.get_template_string_split().alias_attribute(alias);
                template_split.into()
            }
            ExpressionMapTypeEnum::Constant => self.kind.clone(),
            _ => self.kind.alias_attribute(alias),
        };

        Self {
            map_type_pred_iri: self.map_type_pred_iri.clone(),
            kind:              aliased_kind,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ExpressionMapKind {
    FunctionExecution {
        execution: FunctionExecution,
        returns:   Vec<RcTerm>,
    },
    NonFunction(String),
}

impl AttributeAliaser for ExpressionMapKind {
    fn alias_attribute(&self, alias: &str) -> Self {
        match self {
            ExpressionMapKind::FunctionExecution { execution, returns } => {
                ExpressionMapKind::FunctionExecution {
                    execution: execution.alias_attribute(alias),
                    returns:   returns.clone(),
                }
            }
            ExpressionMapKind::NonFunction(inner) => {
                ExpressionMapKind::NonFunction(format!("{}.{}", alias, inner))
            }
        }
    }
}

impl ExpressionMapKind {
    pub fn try_get_non_function_value(&self) -> Option<&String> {
        if let ExpressionMapKind::NonFunction(val) = self {
            Some(val)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExpressionMapTypeEnum {
    Function,
    Template,
    Constant,
    Reference,
    Star,
    FunctionExecution,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_backslash_test() {
        let test_str = "\\{\\{\\{ {$.['ISO 3166']} \\}\\}\\}";

        let expected = vec![
            TemplateSubString::NormalString("{{{ ".to_string()),
            TemplateSubString::Attribute("$.['ISO 3166']".to_string()),
            TemplateSubString::NormalString(" }}}".to_string()),
        ];

        let actual = split_template_string(test_str);

        assert_eq!(expected, actual);
    }
}
