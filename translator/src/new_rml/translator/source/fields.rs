use operator::formats::ReferenceFormulation;
use operator::Field as OperatorField;

use crate::new_rml::error::NewRMLTranslationResult;
use crate::new_rml::rml_model::v2::core::RMLIterable;
use crate::new_rml::rml_model::v2::lv::{RMLField, RMLFieldKind};

pub fn translate_rml_field(
    field: &RMLField,
    ref_form: ReferenceFormulation,
) -> NewRMLTranslationResult<OperatorField> {
    let (reference, reference_formulation) = match &field.kind {
        RMLFieldKind::Iterable(rmliterable) => {
            (
                rmliterable.iterator.clone().unwrap(),
                rmliterable
                    .reference_formulation
                    .clone()
                    .unwrap()
                    .try_into()?,
            )
        }
        RMLFieldKind::Expression(expression_map) => {
            (expression_map.get_value().unwrap().to_string(), ref_form)
        }
    };

    let inner_fields =
        translate_rml_field_vec(&field.fields, reference_formulation.clone())?;

    Ok(OperatorField {
        alias: field.name.clone(),
        reference,
        reference_formulation,
        inner_fields,
    })
}

pub fn translate_rml_field_vec(
    fields: &[RMLField],
    ref_form: ReferenceFormulation,
) -> NewRMLTranslationResult<Vec<OperatorField>> {
    fields.iter().try_fold(
        vec![],
        |mut acc: Vec<OperatorField>,
         f|
         -> NewRMLTranslationResult<Vec<OperatorField>> {
            acc.push(translate_rml_field(f, ref_form.clone())?);
            Ok(acc)
        },
    )
}
