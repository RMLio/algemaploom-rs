use std::collections::HashSet;

use sophia_term::RcTerm;

use crate::new_rml::rml_model::v2::core::expression_map::{
    split_template_string, RefAttributeGetter,
};
use crate::new_rml::rml_model::v2::core::TemplateSubString;
use crate::new_rml::rml_model::v2::AttributeAliaser;

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

impl AttributeAliaser for BaseExpressionMapEnum {
    fn alias_attribute(&self, alias: &str) -> Self {
        match self {
            BaseExpressionMapEnum::Template(v) => {
                let template_substring_vec = split_template_string(&v);
                template_substring_vec.alias_attribute(alias).into()
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
                split_template_string(template)
                    .into_iter()
                    .filter_map(|sstring| {
                        match sstring {
                            TemplateSubString::Attribute(str) => Some(str),
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
