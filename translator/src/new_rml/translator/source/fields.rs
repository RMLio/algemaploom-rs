use operator::formats::ReferenceFormulation;
use operator::Field as OperatorField;

use crate::new_rml::error::NewRMLTranslationResult;
use crate::new_rml::rml_model::v2::core::expression_map::ExpressionMapTypeEnum;
use crate::new_rml::rml_model::v2::lv::{RMLField, RMLFieldKind};
use crate::new_rml::translator::error::TranslationError;

pub fn translate_rml_field(
    field: &RMLField,
    ref_form: ReferenceFormulation,
) -> NewRMLTranslationResult<OperatorField> {
    log::debug!("Translating field: {:?}", field);
    match &field.kind {
        RMLFieldKind::Iterable(rmliterable) => {
            let value = &rmliterable.iterator;
            let ref_form: ReferenceFormulation = rmliterable
                .reference_formulation
                .clone()
                .and_then(|rmliter_ref_form| rmliter_ref_form.try_into().ok())
                .unwrap_or_else(|| ref_form.clone());

            let inner_fields =
                translate_rml_field_vec(&field.fields, ref_form.clone())?;
            Ok(OperatorField {
                alias: field.name.clone(),
                constant: None,
                iterator: value.clone(),
                //FIXME: Remove this reference value assignment and update the engine to consider
                //this change too!
                reference: value.clone(),
                reference_formulation: ref_form,
                inner_fields,
            })
        }
        RMLFieldKind::Expression(expression_map) => {
            todo!()
            
           // let value = expression_map.get_value().cloned();
           // let inner_fields =
           //     translate_rml_field_vec(&field.fields, ref_form.clone())?;

           // match expression_map.get_map_type_enum()? {
           //     ExpressionMapTypeEnum::Constant => {
           //         Ok(OperatorField {
           //             alias: field.name.clone(),
           //             constant: value,
           //             iterator: None,
           //             reference: None,
           //             reference_formulation: ref_form,
           //             inner_fields,
           //         })
           //     }
           //     ExpressionMapTypeEnum::Reference => {
           //         Ok(OperatorField {
           //             alias: field.name.clone(),
           //             constant: None,
           //             iterator: None,
           //             reference: value,
           //             reference_formulation: ref_form,
           //             inner_fields,
           //         })
           //     }
           //     field_type => {
           //         Err(TranslationError::SourceError(format!(
           //             "Expression Field {:?} cannot be of type {:?}",
           //             field.name, field_type
           //         ))
           //         .into())
           //     }
           // }
        }
    }
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
