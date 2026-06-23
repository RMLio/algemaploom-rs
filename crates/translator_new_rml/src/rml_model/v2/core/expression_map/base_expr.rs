use sophia_term::RcTerm;
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

use crate::rml_model::v2::core::expression_map::error::ExpressionMapError;
use crate::rml_model::v2::core::expression_map::{
    split_template_string, RefAttributeGetter,
};
use crate::rml_model::v2::core::TemplateSubString;
use crate::rml_model::v2::AttributeAliaser;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Template {
    value: String,
    parts: Vec<TemplateSubString>,
}

impl Template {
    pub fn get_parts(&self) -> &Vec<TemplateSubString> {
        &self.parts
    }
}
impl Hash for Template {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        for part in &self.parts {
            part.hash(state);
        }
    }
}

impl TryFrom<String> for Template {
    type Error = ExpressionMapError;
    fn try_from(value: String) -> Result<Self, ExpressionMapError> {
        // Validate the template string
        let parts = split_template_string(&value)?;
        Ok(Template {
            value,
            parts
        })
    }
}

impl Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum BaseExpressionMapEnum {
    Template(Template),
    Reference(String),
    Constant(RcTerm),
    Unknown { type_iri: RcTerm, term_val: RcTerm },
}
impl BaseExpressionMapEnum {
    pub fn get_template_string_split(&self) -> Vec<TemplateSubString> {
        match self {
            BaseExpressionMapEnum::Template(template) => {
                template.parts.clone()
            }
            _ => Vec::new(),
        }
    }
}

impl AttributeAliaser for BaseExpressionMapEnum {
    fn alias_attribute(&self, alias: &str) -> Self {
        match self {
            BaseExpressionMapEnum::Template(v) => {
                v.parts.alias_attribute(alias).try_into().unwrap()
            }
            BaseExpressionMapEnum::Reference(v) => {
                BaseExpressionMapEnum::Reference(format!("{}.{}", alias, v))
            }
            _ => self.clone(),
        }
    }
}

impl RefAttributeGetter for BaseExpressionMapEnum {
    fn get_ref_attributes(&self) -> HashSet<String> {
        match self {
            BaseExpressionMapEnum::Template(template) => {
                template.parts
                    .iter()
                    .filter_map(|sstring| {
                        match sstring {
                            TemplateSubString::Attribute(str) => Some(str.clone()),
                            TemplateSubString::NormalString(_) => None,
                        }
                    })
                    .collect()
            }
            BaseExpressionMapEnum::Reference(ref_attr) => {
                HashSet::from([ref_attr.to_string()])
            }
            _ => HashSet::new(),
        }
    }
}
