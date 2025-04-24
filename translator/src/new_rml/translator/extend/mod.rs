use std::collections::HashMap;
use std::rc::Rc;

use operator::{Extend, Function};

use super::error::TranslationError;
use super::store::SearchStore;
use super::OperatorTranslator;
use crate::new_rml::error::{NewRMLTranslationError, NewRMLTranslationResult};
use crate::new_rml::extractors::stringify_rcterm;
use crate::new_rml::rml_model::v2::core::expression_map::term_map::{
    ObjectMap, RMLTermTypeKind, TermMap,
};
use crate::new_rml::rml_model::v2::core::expression_map::{
    ExpressionMap, ExpressionMapKind, ExpressionMapTypeEnum,
};
use crate::new_rml::rml_model::v2::core::{TemplateSubString, TriplesMap};
use crate::new_rml::rml_model::v2::fnml::InputMap;

pub fn func_is_not_constant(func: &Function) -> bool {
    match func {
        Function::Iri {
            base_iri,
            inner_function,
        } => func_is_not_constant(inner_function),
        Function::Literal {
            inner_function,
            dtype_function,
            langtype_function,
        } => func_is_not_constant(inner_function),
        Function::BlankNode { inner_function } => {
            func_is_not_constant(inner_function)
        }
        Function::TypedConstant { value, term_type } => false,
        Function::Constant { value } => false,
        Function::Nop => false,
        _ => true,
    }
}

#[derive(Debug, Clone)]
pub struct ExtendOperatorTranslator {}

impl OperatorTranslator for ExtendOperatorTranslator {
    type Input = TriplesMap;

    type Output = Extend;

    fn translate_with_store(
        store: &SearchStore,
        tm: &Self::Input,
    ) -> crate::new_rml::error::NewRMLTranslationResult<Self::Output> {
        let base_iri = &store.base_iri;
        let mut extend_pairs: HashMap<String, Function> = HashMap::new();

        // Extend function for the subject map
        let (var, func) =
            extend_from_term_map(store, base_iri, &tm.subject_map.term_map)?;
        insert_non_constant_func(&mut extend_pairs, var, func);

        // Extend functions for each graph map of a subject map
        for gm in &tm.subject_map.graph_maps {
            let (var, func) =
                extend_from_term_map(store, base_iri, &gm.term_map)?;
            insert_non_constant_func(&mut extend_pairs, var, func);
        }

        // Extend functions for each predicate object map of the given triples map
        for pom in &tm.predicate_object_map_vec {
            for gm in &pom.graph_map_vec {
                let (var, func) =
                    extend_from_term_map(store, base_iri, &gm.term_map)?;
                insert_non_constant_func(&mut extend_pairs, var, func);
            }

            for pm in &pom.predicate_map_vec {
                let (var, func) =
                    extend_from_term_map(store, base_iri, &pm.term_map)?;
                insert_non_constant_func(&mut extend_pairs, var, func);
            }

            for om in &pom.object_map_vec {
                let (var, func) =
                    extend_from_term_map(store, base_iri, &om.term_map)?;

                let func = extend_lang_dtype_function_for_om(store, om, func)?;
                insert_non_constant_func(&mut extend_pairs, var, func);
            }
        }
        Ok(operator::Extend { extend_pairs })
    }
}

fn insert_non_constant_func(
    extend_pairs: &mut HashMap<String, Function>,
    var: String,
    func: Function,
) {
    if func_is_not_constant(&func) {
        extend_pairs.insert(var, func);
    }
}

fn extend_lang_dtype_function_for_om(
    store: &SearchStore<'_>,
    om: &ObjectMap,
    func: Function,
) -> Result<Function, NewRMLTranslationError> {
    Ok(match &func {
        Function::Literal {
            inner_function,
            dtype_function: _,
            langtype_function: _,
        } => {
            if let Some(lang_map) = &om.language_map {
                let langtype_function =
                    Some(Rc::new(extend_from_exp_map(store, lang_map)?));
                Function::Literal {
                    inner_function: inner_function.clone(),
                    dtype_function: None,
                    langtype_function,
                }
            } else if let Some(dtype_map) = &om.datatype_map {
                let dtype_function =
                    Some(Rc::new(extend_from_exp_map(store, dtype_map)?));
                Function::Literal {
                    inner_function: inner_function.clone(),
                    dtype_function,
                    langtype_function: None,
                }
            } else {
                func
            }
        }
        _ => func,
    })
}

fn extend_from_term_map(
    store: &SearchStore,
    base_iri: &str,
    term_map: &TermMap,
) -> NewRMLTranslationResult<(String, Function)> {
    let inner_func = extend_from_exp_map(store, &term_map.expression)?;

    let function = match term_map.try_get_term_type_enum()? {
        RMLTermTypeKind::BlankNode => {
            Ok(Function::BlankNode {
                inner_function: inner_func.into(),
            })
        }
        RMLTermTypeKind::IRI => {
            Ok(Function::Iri {
                base_iri:       Some(base_iri.to_string()),
                inner_function: inner_func.into(),
            })
        }
        RMLTermTypeKind::Literal => {
            Ok(Function::Literal {
                inner_function:    inner_func.into(),
                dtype_function:    None,
                langtype_function: None,
            })
        }

        _ => {
            Err(TranslationError::ExtendError(format!(
                "Given term type is unsupported: {:?}",
                stringify_rcterm(term_map.term_type.clone())
            )))
        }
    }?;

    let var = store
        .termm_id_quad_var_map
        .get(&term_map.identifier)
        .unwrap()
        .to_string();
    Ok((var, function))
}

fn extend_from_exp_map(
    store: &SearchStore,
    exp_map: &ExpressionMap,
) -> NewRMLTranslationResult<Function> {
    let function: Function = match exp_map.get_map_type_enum()? {
        ExpressionMapTypeEnum::Function => fno_extend_function(store, exp_map)?,
        ExpressionMapTypeEnum::Template => template_extend_function(exp_map),
        ExpressionMapTypeEnum::Constant => {
            Function::Constant {
                value: exp_map
                    .try_get_non_function_value()
                    .unwrap()
                    .to_string(),
            }
        }
        ExpressionMapTypeEnum::Reference => {
            Function::Reference {
                value: exp_map
                    .try_get_non_function_value()
                    .unwrap()
                    .to_string(),
            }
        }
    };

    Ok(function)
}

fn fno_input_extend_function(
    store: &SearchStore,
    input_map: &InputMap,
) -> NewRMLTranslationResult<(String, Rc<Function>)> {
    let param = stringify_rcterm(input_map.parameter.clone()).unwrap();

    let function = extend_from_exp_map(store, &input_map.value_map.expression)?;

    Ok((param, function.into()))
}

fn fno_extend_function(
    store: &SearchStore,
    exp_map: &ExpressionMap,
) -> NewRMLTranslationResult<Function> {
    if let ExpressionMapKind::FunctionExecution { execution, returns } =
        &exp_map.kind
    {
        let mut parameters = HashMap::with_capacity(execution.input.capacity());
        for input in &execution.input {
            let (param, func) = fno_input_extend_function(store, input)?;
            parameters.insert(param, func);
        }

        Ok(Function::FnO {
            fno_identifier: stringify_rcterm(execution.function.clone())
                .unwrap(),
            parameters,
        })
    } else {
        Err(TranslationError::Infallible.into())
    }
}

fn template_extend_function(exp_map: &ExpressionMap) -> Function {
    let template_splits = exp_map.get_template_string_split();
    let mut template_function = Function::Nop;

    for split in template_splits {
        let right_func = match &split {
            TemplateSubString::Attribute(attr) => {
                Function::Reference {
                    value: attr.to_string(),
                }
            }
            TemplateSubString::NormalString(norm) => {
                Function::Constant {
                    value: norm.to_string(),
                }
            }
        };

        template_function = Function::Concatenate {
            left_value:  template_function.into(),
            separator:   "".to_string(),
            right_value: right_func.into(),
        };
    }
    template_function
}
