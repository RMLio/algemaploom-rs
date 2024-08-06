use std::collections::HashMap;

use lazy_static::lazy_static;
use operator::{Extend, Function, Operator, RcExtendFunction};
use regex::Regex;
use rml_interpreter::rml_model::term_map::{
    SubjectMap, TermMapInfo, TermMapType,
};
use rml_interpreter::rml_model::PredicateObjectMap;
use sophia_api::term::{TTerm, TermKind};

use crate::rmlalgebra::util::{
    extract_gm_tm_infos, extract_tm_infos_from_poms,
};
use crate::OperatorTranslator;

#[derive(Debug, Clone)]
pub struct ExtendTranslator<'a> {
    pub tms:          Vec<&'a TermMapInfo>,
    pub variable_map: &'a HashMap<String, String>,
    pub base_iri:     Option<String>,
}

impl<'a> OperatorTranslator<Operator> for ExtendTranslator<'a> {
    fn translate(&self) -> Operator {
        let mut extend_pairs = HashMap::new();
        for tm_info in &self.tms {
            let (variable, function) =
                extract_extend_function_from_term_map_info(
                    self.variable_map,
                    tm_info,
                    &self.base_iri,
                );
            extend_pairs.insert(variable, function);
        }

        Operator::ExtendOp {
            config: Extend { extend_pairs },
        }
    }
}

pub fn extract_extend_function_from_term_map_info(
    variable_map: &HashMap<String, String>,
    tm_info: &TermMapInfo,
    base_iri: &Option<String>,
) -> (String, Function) {
    let func = extract_function(tm_info, base_iri);

    (
        variable_map.get(&tm_info.identifier).unwrap().to_string(),
        func,
    )
}

lazy_static! {
    static ref TEMPLATE_REGEX: Regex = Regex::new(r"\{([^\\{\\}]*)\}").unwrap();
}

fn extract_template_function(
    term_value: String,
    term_type: &Option<TermKind>,
) -> Function {
    let found_variables = TEMPLATE_REGEX
        .captures_iter(&term_value)
        .map(|c| c.extract())
        .map(|(_, [var])| var);

    let variable_function_pairs = if *term_type == Some(TermKind::Iri) {
        found_variables
            .map(|var| {
                (
                    var.to_string(),
                    Function::UriEncode {
                        inner_function: Function::Reference {
                            value: var.to_string(),
                        }
                        .into(),
                    }
                    .into(),
                )
            })
            .collect()
    } else {
        found_variables
            .map(|var| {
                (
                    var.to_string(),
                    Function::Reference {
                        value: var.to_string(),
                    }
                    .into(),
                )
            })
            .collect()
    };

    Function::TemplateFunctionValue {
        template: term_value,
        variable_function_pairs,
    }
}

fn extract_function(
    tm_info: &TermMapInfo,
    base_iri: &Option<String>,
) -> Function {
    let term_value = tm_info.term_value.value().to_string();
    let value_function: RcExtendFunction = match tm_info.term_map_type {
        TermMapType::Constant => {
            Function::Constant {
                value: term_value.clone(),
            }
        }
        TermMapType::Reference => {
            Function::Reference {
                value: term_value.clone(),
            }
        }
        TermMapType::Template => {
            extract_template_function(term_value, &tm_info.term_type)
        }
        TermMapType::Function => {
            let fn_map = tm_info.fun_map_opt.as_ref().unwrap();
            let fno_identifier = fn_map.function_iri.clone();
            let param_func_pairs = fn_map
                .param_om_pairs
                .iter()
                .map(|(param, om)| {
                    (
                        param.to_string(),
                        extract_function(&om.tm_info, base_iri).into(),
                    )
                })
                .collect();

            Function::FnO {
                fno_identifier,
                param_func_pairs,
            }
        }
    }
    .into();

    match tm_info.term_type.unwrap() {
        sophia_api::term::TermKind::Iri => {
            Function::Iri {
                base_iri:       base_iri.clone(),
                inner_function: value_function,
            }
        }
        sophia_api::term::TermKind::Literal => {
            Function::Literal {
                inner_function:    value_function,
                langtype_function: None,
                dtype_function:    None,
            }
        }
        sophia_api::term::TermKind::BlankNode => {
            Function::BlankNode {
                inner_function: value_function,
            }
        }
        typ => panic!("Unrecognized term kind {:?}", typ),
    }
}

pub fn translate_extend_pairs(
    variable_map: &HashMap<String, String>,
    sm: &SubjectMap,
    poms: &[PredicateObjectMap],
    base_iri: &Option<String>,
) -> HashMap<String, Function> {
    let mut tm_infos = extract_tm_infos_from_poms(poms);
    tm_infos.push(&sm.tm_info);
    tm_infos.extend(extract_gm_tm_infos(sm, poms));

    tm_infos
        .into_iter()
        .map(|tm_info| {
            extract_extend_function_from_term_map_info(
                variable_map,
                tm_info,
                base_iri,
            )
        })
        .collect()
}
