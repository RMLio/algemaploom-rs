use crate::error::NewRMLTranslationResult;
use crate::rml_model::v2::core::expression_map::term_map::RMLTermTypeKind;
use crate::rml_model::v2::lv::{RMLField, RMLFieldKind};
use crate::translator::extend::extension_func_from_exp_map;
use operator::formats::ReferenceFormulation;
use operator::io::source::field::Field as OperatorField;
use std::collections::HashMap;
use std::rc::Rc;

fn translate_rml_field_mut(
    field: &RMLField,
    ref_form: ReferenceFormulation,
    alias_query_map:  &mut HashMap<String, String>, 
) -> NewRMLTranslationResult<OperatorField> {
    log::debug!("Translating field: {:?}", field);

    let alias = field.name.clone();
    let absolute_path  = Some(field.absolute_name.clone()); 

    match &field.kind {
        RMLFieldKind::Iterable(rmliterable) => {
            let value = &rmliterable.iterator;
            let ref_form: ReferenceFormulation = rmliterable
                .reference_formulation
                .clone()
                .and_then(|rmliter_ref_form| rmliter_ref_form.try_into().ok())
                .unwrap_or_else(|| ref_form.clone());

            let inner_fields =
                translate_rml_field_vec(&field.fields, ref_form.clone(), alias_query_map)?;
            Ok(OperatorField {
                alias,
                absolute_path,
                iterator: value.clone(),
                expression: None,
                reference_formulation: ref_form,
                inner_fields,
            })
        }
        RMLFieldKind::Expression(expression_map) => {
            let extension_function = extension_func_from_exp_map(&Some(expression_map.clone()), &RMLTermTypeKind::Unknown)?; // TODO: or term type Literal?
            Ok(OperatorField {
                alias,
                absolute_path,
                expression: Some(Rc::new(extension_function)),
                iterator: None,
                reference_formulation: ref_form,
                inner_fields: vec![],
            })
        }
    }
}

pub fn translate_rml_field_vec(
    fields: &[RMLField],
    ref_form: ReferenceFormulation,
    alias_query_map:  &mut HashMap<String, String>, 
) -> NewRMLTranslationResult<Vec<OperatorField>> {
    fields.iter().try_fold(
        vec![],
        |mut acc: Vec<OperatorField>,
         f|
         -> NewRMLTranslationResult<Vec<OperatorField>> {
            acc.push(translate_rml_field_mut(f, ref_form.clone(), alias_query_map)?);
            Ok(acc)
        },
    )
}
