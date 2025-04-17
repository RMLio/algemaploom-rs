use operator::{formats, Iterator};

use super::fields::translate_rml_field_vec;
use crate::new_rml::error::NewRMLTranslationResult;
use crate::new_rml::rml_model::v2::core::{
    AbstractLogicalSource, AbstractSourceEnum,
};
use crate::new_rml::translator::error::TranslationError;
use crate::new_rml::translator::OperatorTranslator;

#[derive(Debug, Clone)]
pub struct IteratorTranslator {}

impl OperatorTranslator for IteratorTranslator {
    type Input = AbstractLogicalSource;

    type Output = Iterator;

    fn translate(
        abs_ls: &Self::Input,
    ) -> NewRMLTranslationResult<Self::Output> {
        let logical_view = match &abs_ls.abs_source_enum {
            AbstractSourceEnum::IOLogicalSource(_) => {
                Err(TranslationError::SourceError(
                        "RML 2's IO Logical source is unsupported for translation for iterators in source operator".to_string()
                        )
                    )
            },
            AbstractSourceEnum::LogicalView(logical_view) => Ok(logical_view),
        }?;

        let rml_iterable = &abs_ls.iterable;
        let mut reference_formulation = formats::ReferenceFormulation::CSVRows;

        if let Some(ref_form) = &rml_iterable.reference_formulation {
            reference_formulation = ref_form.try_into()?;
        }

        Ok(Iterator {
            reference:             rml_iterable.iterator.clone(),
            reference_formulation: reference_formulation.clone(),
            fields:                translate_rml_field_vec(
                &logical_view.fields,
                reference_formulation,
            )?,
            alias:                 None,
        })
    }
}
