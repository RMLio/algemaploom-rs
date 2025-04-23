use lazy_static::lazy_static;
use regex::Regex;
use sophia_api::term::{Term, TermKind};
use sophia_term::RcTerm;

use crate::new_rml::extractors::error::ParseError;
use crate::new_rml::extractors::{ExtractorResult, FromVocab};
use crate::new_rml::rml_model::v2::core::TemplateSubString;
use crate::new_rml::rml_model::v2::fnml::FunctionExecution;

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
                println!("{}", c); 
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

    if !current_buf.is_empty(){
        result.push(TemplateSubString::NormalString(current_buf));
    }
    result
}

#[derive(Debug, Clone)]
pub struct ExpressionMap {
    pub map_type_pred_iri: RcTerm,
    pub kind:              ExpressionMapKind,
}

impl ExpressionMap {
    pub fn get_template_string_split(&self) -> Vec<TemplateSubString> {
        if let ExpressionMapTypeEnum::Template =
            self.get_map_type_enum().unwrap()
        {
            if let Some(template) = self.get_value() {
                return split_template_string(template);
            }
        }
        vec![]
    }

    pub fn from_template_str(template: &str) -> ExpressionMap {
        Self {
            map_type_pred_iri: vocab::rml_core::PROPERTY::TEMPLATE.to_rcterm(),
            kind:              ExpressionMapKind::NonFunction(
                template.to_string(),
            ),
        }
    }
    pub fn from_const_str(const_str: &str) -> ExpressionMap {
        Self {
            map_type_pred_iri: vocab::rml_core::PROPERTY::CONSTANT.to_rcterm(),
            kind:              ExpressionMapKind::NonFunction(
                const_str.to_string(),
            ),
        }
    }
    pub fn from_ref_str(ref_str: &str) -> ExpressionMap {
        Self {
            map_type_pred_iri: vocab::rml_core::PROPERTY::REFERENCE.to_rcterm(),
            kind:              ExpressionMapKind::NonFunction(
                ref_str.to_string(),
            ),
        }
    }
    pub fn try_new(
        value_pred: RcTerm,
        value: String,
    ) -> Result<Self, ParseError> {
        if value_pred.kind() != TermKind::Iri {
            return Err(ParseError::GenericError(format!(
                "Expression map contains an invalid predicate term {:?}",
                value_pred
            )));
        }
        Ok(Self {
            map_type_pred_iri: value_pred,
            kind:              ExpressionMapKind::NonFunction(value),
        })
    }
    pub fn get_value(&self) -> Option<&String> {
        match &self.kind {
            ExpressionMapKind::FunctionExecution {
                execution: _,
                returns: _,
            } => None,
            ExpressionMapKind::NonFunction(s) => Some(s),
        }
    }

    pub fn get_map_type_enum(&self) -> ExtractorResult<ExpressionMapTypeEnum> {
        match self.map_type_pred_iri.clone() {
            value if value == vocab::rml_fnml::PROPERTY::FUNCTION_MAP.to_rcterm() => {
                Ok(ExpressionMapTypeEnum::Function)
            }
            value
                if value == vocab::r2rml::PROPERTY::TEMPLATE.to_rcterm()
                    || value
                        == vocab::rml_core::PROPERTY::TEMPLATE.to_rcterm() =>
            {
                Ok(ExpressionMapTypeEnum::Template)
            }
            value
                if value == vocab::r2rml::PROPERTY::CONSTANT.to_rcterm()
                    || value
                        == vocab::rml_core::PROPERTY::CONSTANT.to_rcterm() =>
            {
                Ok(ExpressionMapTypeEnum::Constant)
            }
            value
                if value == vocab::rml::PROPERTY::REFERENCE.to_rcterm()
                    || value
                        == vocab::rml_core::PROPERTY::REFERENCE.to_rcterm() =>
            {
                Ok(ExpressionMapTypeEnum::Reference)
            }

            _ => {
                Err(ParseError::GenericError(format!(
                "Invalid predicate IRI detected for term map type inference {:?}",
                self.map_type_pred_iri
            )).into())
            }
        }
    }

    pub fn try_get_non_function_value(&self) -> Option<&String> {
        self.kind.try_get_non_function_value()
    }
}
#[derive(Debug, Clone)]
pub enum ExpressionMapKind {
    FunctionExecution {
        execution: FunctionExecution,
        returns:   Vec<RcTerm>,
    },
    NonFunction(String),
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
