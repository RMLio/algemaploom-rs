use sophia_api::term::{Term, TermKind};
use sophia_term::RcTerm;

use crate::new_rml::extractors::error::ParseError;
use crate::new_rml::extractors::{ExtractorResult, FromVocab};
use crate::new_rml::rml_model::v2::fnml::FunctionExecution;

pub mod term_map;

#[derive(Debug, Clone)]
pub struct ExpressionMap {
    pub map_type_pred_iri: RcTerm,
    pub kind:              ExpressionMapKind,
}

impl ExpressionMap {
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

    pub fn get_value_type_enum(&self) -> ExtractorResult<ExpressionValueEnum> {
        match self.map_type_pred_iri.clone() {
            value
                if value == vocab::r2rml::PROPERTY::TEMPLATE.to_rcterm()
                    || value
                        == vocab::rml_core::PROPERTY::TEMPLATE.to_rcterm() =>
            {
                Ok(ExpressionValueEnum::Template)
            }
            value
                if value == vocab::r2rml::PROPERTY::CONSTANT.to_rcterm()
                    || value
                        == vocab::rml_core::PROPERTY::CONSTANT.to_rcterm() =>
            {
                Ok(ExpressionValueEnum::Constant)
            }
            value
                if value == vocab::rml::PROPERTY::REFERENCE.to_rcterm()
                    || value
                        == vocab::rml_core::PROPERTY::REFERENCE.to_rcterm() =>
            {
                Ok(ExpressionValueEnum::Reference)
            }

            _ => {
                Err(ParseError::GenericError(format!(
                "Invalid predicate IRI detected for term map type inference {:?}",
                self.map_type_pred_iri
            )).into())
            }
        }
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
pub enum ExpressionValueEnum {
    Template,
    Constant,
    Reference,
}
