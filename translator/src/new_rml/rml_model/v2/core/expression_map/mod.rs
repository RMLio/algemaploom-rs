use std::collections::HashSet;

use derive_more::{TryUnwrap, Unwrap};
use sophia_api::term::{Term, TermKind};
use sophia_term::RcTerm;

use crate::new_rml::extractors::error::ParseError;
use crate::new_rml::extractors::stringify_rcterm;
use crate::new_rml::rml_model::v2::core::TemplateSubString;
use crate::new_rml::rml_model::v2::fnml::{
    FunctionExecution, FunctionExpressionMap,
};
use crate::new_rml::rml_model::v2::{AttributeAliaser, RefAttributeGetter};

mod base_expr;
pub use base_expr::BaseExpressionMapEnum;
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

impl From<Vec<TemplateSubString>> for BaseExpressionMapEnum {
    fn from(value: Vec<TemplateSubString>) -> Self {
        let template_string: String =
            value.into_iter().map(|val| val.to_string()).collect();
        BaseExpressionMapEnum::Template(template_string)
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

#[derive(Debug, Clone, Hash, Eq, PartialEq, TryUnwrap, Unwrap)]
#[unwrap(ref)]
#[try_unwrap(ref)]
pub enum ExpressionMapEnum {
    BaseExpressionMap(BaseExpressionMapEnum),
    FunctionExpressionMap(FunctionExpressionMap),
}

impl ExpressionMapEnum {
    pub fn get_template_string_split(&self) -> Vec<TemplateSubString> {
        if let Ok(base_expr_enum) = self.try_unwrap_base_expression_map_ref() {
            match base_expr_enum {
                BaseExpressionMapEnum::Template(template) => {
                    split_template_string(template)
                }
                _ => vec![],
            }
        } else {
            vec![]
        }
    }

    pub fn new_template_term<T>(term: T) -> ExpressionMapEnum
    where
        T: Term,
    {
        ExpressionMapEnum::BaseExpressionMap(BaseExpressionMapEnum::Template(
            stringify_rcterm(term).unwrap(),
        ))
    }

    pub fn new_reference_term<T>(term: T) -> ExpressionMapEnum
    where
        T: Term,
    {
        ExpressionMapEnum::BaseExpressionMap(BaseExpressionMapEnum::Reference(
            stringify_rcterm(term).unwrap(),
        ))
    }

    pub fn new_constant_term<T>(term: T) -> ExpressionMapEnum
    where
        T: Term,
    {
        ExpressionMapEnum::BaseExpressionMap(BaseExpressionMapEnum::Constant(
            stringify_rcterm(term).unwrap(),
        ))
    }

    pub fn new_template_str(template: &str) -> ExpressionMapEnum {
        let template_expr =
            BaseExpressionMapEnum::Template(template.to_string());
        Self::BaseExpressionMap(template_expr)
    }

    pub fn new_const_str(const_str: &str) -> ExpressionMapEnum {
        Self::BaseExpressionMap(BaseExpressionMapEnum::Constant(
            const_str.to_string(),
        ))
    }

    pub fn new_ref_str(ref_str: &str) -> ExpressionMapEnum {
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

impl RefAttributeGetter for ExpressionMapEnum {
    fn get_ref_attributes(&self) -> HashSet<String> {
        match self {
            ExpressionMapEnum::BaseExpressionMap(base_expression_map_enum) => {
                base_expression_map_enum.get_ref_attributes()
            }
            ExpressionMapEnum::FunctionExpressionMap(
                function_expression_map,
            ) => function_expression_map.get_ref_attributes(),
        }
    }
}

impl AttributeAliaser for ExpressionMapEnum {
    fn alias_attribute(&self, alias: &str) -> Self {
        match self {
            ExpressionMapEnum::BaseExpressionMap(base_expression_map_enum) => {
                Self::BaseExpressionMap(
                    base_expression_map_enum.alias_attribute(alias),
                )
            }
            ExpressionMapEnum::FunctionExpressionMap(
                function_expression_map,
            ) => {
                Self::FunctionExpressionMap(
                    function_expression_map.alias_attribute(alias),
                )
            }
        }
    }
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
