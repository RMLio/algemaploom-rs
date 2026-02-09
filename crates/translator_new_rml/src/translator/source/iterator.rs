use std::collections::HashMap;

use operator::{formats, Iterator};

use super::fields::translate_rml_field_vec;
use crate::error::NewRMLTranslationResult;
use crate::rml_model::v2::core::AbstractLogicalSourceEnum;
use crate::translator::error::TranslationError;
use crate::translator::OperatorTranslator;

#[derive(Debug, Clone)]
pub struct IteratorTranslator;

impl OperatorTranslator for IteratorTranslator {
    type Input = AbstractLogicalSourceEnum;

    type Output = Iterator;

    fn translate(
        abs_ls: &Self::Input,
    ) -> NewRMLTranslationResult<Self::Output> {
        let logical_view = match &abs_ls{
            AbstractLogicalSourceEnum::LogicalSource(_) => {
                Err(TranslationError::SourceError(
                        "RML 2's IO Logical source is unsupported for translation for iterators in source operator".to_string()
                        )
                    )
            },
            AbstractLogicalSourceEnum::LogicalView(logical_view) => Ok(logical_view),
        }?;

        let rml_iterable = &abs_ls.get_iterable();
        let mut reference_formulation = formats::ReferenceFormulation::CSVRows;

        if let Some(ref_form) = &rml_iterable.reference_formulation {
            reference_formulation = ref_form.try_into()?;
        }

        let mut alias_query_map = HashMap::new();

        Ok(Iterator {
            reference:             rml_iterable.iterator.clone(),
            reference_formulation: reference_formulation.clone(),
            fields:                translate_rml_field_vec(
                &logical_view.fields,
                reference_formulation,
                &mut alias_query_map,
            )?,
            alias:                 None,
        })
    }
}
