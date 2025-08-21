use operator::formats::ReferenceFormulation;
use operator::Field as OperatorField;
use sophia_turtle::serializer::nt;

use crate::new_rml::error::{NewRMLTranslationError, NewRMLTranslationResult};
use crate::new_rml::extractors::turtle_stringify_term;
use crate::new_rml::rml_model::v2::core::expression_map::BaseExpressionMapEnum;
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
                reference: None,
                reference_formulation: ref_form,
                inner_fields,
            })
        }
        RMLFieldKind::Expression(expression_map) => {
            if let Ok(base_expr_enum) =
                expression_map.try_unwrap_base_expression_map_ref()
            {
                match base_expr_enum {
                    BaseExpressionMapEnum::Reference(reference) => {
                        Ok(OperatorField {
                            alias:                 field.name.clone(),
                            reference:             Some(reference.clone()),
                            constant:              None,
                            iterator:              None,
                            reference_formulation: ref_form,
                            inner_fields:          vec![],
                        })
                    }
                    BaseExpressionMapEnum::Constant(constant) => {
                        Ok(OperatorField {
                            alias:                 field.name.clone(),
                            reference:             None,
                            constant:              turtle_stringify_term(constant), 
                            iterator:              None,
                            reference_formulation: ref_form,
                            inner_fields:          vec![],
                        })
                    }
                    BaseExpressionMapEnum::Template(template) => {
                        Err(TranslationError::SourceError(
                                format!("Translating template expression maps as part of RML field straight away is not supported!
                                    Expression field: {:?} with value {:?}", field.name, template)).into())
                    },
                    BaseExpressionMapEnum::Unknown { type_iri, term_val } => {
                        Err(TranslationError::SourceError(
                                format!("RML expression field has an unknown type {:?} with value {:?}
                                    Expression field: {:?}", type_iri, term_val, field.name)).into())
                    }
                }
            } else {
                //FIXME: Also consider function expression maps in logical views!!!
                Err(TranslationError::SourceError("Using FNML function expression maps in RML fields is not supported yet!".to_string()).into())
            }
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
