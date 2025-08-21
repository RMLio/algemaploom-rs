use std::collections::HashMap;
use std::rc::Rc;

use operator::{Extend, Function};
use sophia_api::ns::xsd;
use sophia_api::term::Term;

use super::error::TranslationError;
use super::store::SearchStore;
use super::OperatorTranslator;
use crate::new_rml::error::{NewRMLTranslationError, NewRMLTranslationResult};
use crate::new_rml::extractors::{stringify_term, turtle_stringify_term};
use crate::new_rml::rml_model::v2::core::expression_map::term_map::{
    termkind_to_rml_rcterm, CommonTermMapInfo, ObjectMap, RMLTermTypeKind,
};
use crate::new_rml::rml_model::v2::core::expression_map::{
    BaseExpressionMapEnum, ExpressionMapEnum,
};
use crate::new_rml::rml_model::v2::core::{TemplateSubString, TriplesMap};
use crate::new_rml::rml_model::v2::fnml::{FunctionExpressionMap, InputMap};

pub fn func_is_not_constant(func: &Function) -> bool {
    match func {
        Function::Iri {
            base_iri: _,
            inner_function,
        } => func_is_not_constant(inner_function),
        Function::Literal {
            inner_function,
            dtype_function: _,
            langtype_function: _,
        } => func_is_not_constant(inner_function),
        Function::BlankNode { inner_function } => {
            func_is_not_constant(inner_function)
        }
        Function::TypedConstant {
            value: _,
            term_type: _,
        } => false,
        Function::Constant { value: _ } => false,
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
        let base_iri = &tm.base_iri;
        let mut extend_pairs: HashMap<String, Function> = HashMap::new();

        // Extend function for the subject map
        let (var, func) =
            extend_from_term_map(store, base_iri, tm.subject_map.as_ref())?;
        insert_non_constant_func(&mut extend_pairs, var, func);

        // Extend functions for each graph map of a subject map
        if let Ok(sm) = tm.subject_map.try_unwrap_subject_map_ref() {
            for gm in &sm.graph_maps {
                let (var, func) =
                    extend_from_term_map(store, base_iri, gm.as_ref())?;
                insert_non_constant_func(&mut extend_pairs, var, func);
            }
        }

        // Extend functions for each predicate object map of the given triples map
        for pom in &tm.predicate_object_map_vec {
            for gm in &pom.graph_map_vec {
                let (var, func) =
                    extend_from_term_map(store, base_iri, gm.as_ref())?;
                insert_non_constant_func(&mut extend_pairs, var, func);
            }

            for pm in &pom.predicate_map_vec {
                let (var, func) =
                    extend_from_term_map(store, base_iri, pm.as_ref())?;
                insert_non_constant_func(&mut extend_pairs, var, func);
            }

            for om_enum in &pom.object_map_vec {
                let (var, mut func) =
                    extend_from_term_map(store, base_iri, om_enum.as_ref())?;

                if let Ok(om) = om_enum.try_unwrap_object_map_ref() {
                    func = extend_lang_dtype_function_for_om(
                        store, base_iri, om, func,
                    )?;
                }
                insert_non_constant_func(&mut extend_pairs, var, func);
            }
        }
        Ok(operator::Extend { extend_pairs })
    }
}

pub fn insert_non_constant_func(
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
    base_iri: &str,
    om: &ObjectMap,
    func: Function,
) -> Result<Function, NewRMLTranslationError> {
    let term_type = &om.term_map_info.get_term_type_enum();
    log::debug!("Term expression {:?} is a literal", om.term_map_info.expression); 
    Ok(match &func {
        Function::Literal {
            inner_function,
            dtype_function: _,
            langtype_function: _,
        } => {
            if let Some(lang_map) = &om.language_map {
                let langtype_function = Some(Rc::new(
                    extension_func_from_exp_map(store, lang_map, term_type)?,
                ));
                Function::Literal {
                    inner_function: inner_function.clone(),
                    dtype_function: None,
                    langtype_function,
                }
            } else if let Some(dtype_map) = &om.datatype_map {
                let dtype_function = Some(Rc::new(Function::Iri {
                    base_iri:       Some(base_iri.to_string()),
                    inner_function: extension_func_from_exp_map(
                        store, dtype_map, term_type,
                    )?
                    .into(),
                }));
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

pub fn extend_from_term_map(
    store: &SearchStore,
    base_iri: &str,
    term_map_info: &CommonTermMapInfo,
) -> NewRMLTranslationResult<(String, Function)> {
    let inner_func = extension_func_from_exp_map(
        store,
        &term_map_info.expression,
        &term_map_info.get_term_type_enum(),
    )?;

    let function = match term_map_info.get_term_type_enum() {
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
            log::debug!("Term expression {:?} is a literal", term_map_info.expression); 
            let mut dtype_function = None; 
            let mut langtype_function = None; 
            if let Ok(BaseExpressionMapEnum::Constant(term)) = term_map_info.expression.try_unwrap_base_expression_map_ref(){
                if let Some( lt) = term.language_tag(){
                    langtype_function = Some(Function::Constant { value: lt.as_str().to_string() }.into()); 
                }
                if let Some(dt) = term.datatype(){
                    let dtype_inner = Function::Constant{ 
                        value: dt.as_str().to_string()
                    }.into(); 
                    dtype_function = Some(Function::Iri { base_iri: None, inner_function: dtype_inner }.into())
;
                }
            }
            
            Ok(Function::Literal {
                inner_function:    inner_func.into(),
                dtype_function,
                langtype_function,
            })
        }

        _ => {
            Err(TranslationError::ExtendError(format!(
                "Given term type is unsupported: {:?}",
                stringify_term(term_map_info.term_type.clone())
            )))
        }
    }?;

    let var = store
        .termm_id_quad_var_map
        .get(&term_map_info.identifier)
        .unwrap()
        .to_string();
    Ok((var, function))
}

pub fn extension_func_from_exp_map(
    store: &SearchStore,
    exp_map: &ExpressionMapEnum,
    term_type: &RMLTermTypeKind,
) -> NewRMLTranslationResult<Function> {
    match exp_map {
        ExpressionMapEnum::BaseExpressionMap(base_expression_map_enum) => {
            extend_func_from_base_expr_map(base_expression_map_enum, term_type)
        }
        ExpressionMapEnum::FunctionExpressionMap(function_expression_map) => {
            extend_func_from_func_expr_map(
                store,
                function_expression_map,
                term_type,
            )
        }
    }
}
fn extend_func_from_base_expr_map(
    base_expr_map: &BaseExpressionMapEnum,
    term_type: &RMLTermTypeKind,
) -> NewRMLTranslationResult<Function> {
    match base_expr_map {
        BaseExpressionMapEnum::Template(_) => {
            Ok(template_extend_function(base_expr_map, term_type))
        }
        BaseExpressionMapEnum::Reference(reference) => {
            Ok(extend_func_from_ref_attr(reference, term_type))
        }
        BaseExpressionMapEnum::Constant(constant) => {
            let value = stringify_term(constant)
                .ok_or(TranslationError::ExtendError(
                        format!("Empty string returned while trying to get the string representation of the term {:?}", constant)
                        )
                    )?;  
            Ok(Function::Constant {
                value ,
            })
        }
        BaseExpressionMapEnum::Unknown { type_iri, term_val } =>  {
            Err(TranslationError::ExtendError(
                format!("Cannot translate extension function for expression map with type: {:?} and value {:?}", type_iri, term_val)
                ).into())
        },
    }
}

fn template_extend_function(
    exp_map: &BaseExpressionMapEnum,
    term_type: &RMLTermTypeKind,
) -> Function {
    let template_splits = exp_map.get_template_string_split();
    let mut template_function = Function::Nop;

    for split in template_splits {
        let right_func_opt = match &split {
            TemplateSubString::Attribute(attr) => {
                Some(extend_func_from_ref_attr(attr, term_type))
            }
            TemplateSubString::NormalString(norm) => {
                if norm.is_empty() {
                    None
                } else {
                    Some(Function::Constant {
                        value: norm.to_string(),
                    })
                }
            }
        };

        if let Some(right_func) = right_func_opt {
            template_function = Function::Concatenate {
                left_value:  template_function.into(),
                separator:   "".to_string(),
                right_value: right_func.into(),
            };
        }
    }
    template_function
}

fn extend_func_from_ref_attr(
    attr: &str,
    term_type: &RMLTermTypeKind,
) -> Function {
    let inner_function = Function::Reference {
        value: attr.to_string(),
    };
    match term_type {
        RMLTermTypeKind::BlankNode
        | RMLTermTypeKind::IRI
        | RMLTermTypeKind::UnsafeIRI
        | RMLTermTypeKind::URI
        | RMLTermTypeKind::UnsafeURI => {
            Function::UriEncode {
                inner_function: inner_function.into(),
            }
        }
        _ => inner_function,
    }
}

fn extend_func_from_func_expr_map(
    store: &SearchStore,
    func_exp_map: &FunctionExpressionMap,
    term_type: &RMLTermTypeKind,
) -> NewRMLTranslationResult<Function> {
    //FIXME: Implement FnO function translation to extend functions from expression maps

    todo!()
    // if let ExpressionMapKind::FunctionExecution { execution, returns } =
    //     &exp_map.kind
    // {
    //     let mut parameters = HashMap::with_capacity(execution.input.capacity());
    //     for input in &execution.input {
    //         let (param, func) = fno_input_extend_function(store, input)?;
    //         parameters.insert(param, func);
    //     }

    //     Ok(Function::FnO {
    //         fno_identifier: stringify_rcterm(execution.function.clone())
    //             .unwrap(),
    //         parameters,
    //     })
    // } else {
    //     Err(TranslationError::Infallible.into())
    // }
}

fn star_extend_function(exp_map: &ExpressionMapEnum) -> Function {
    todo!()
    // TODO: Implement star extend function
    //
}
