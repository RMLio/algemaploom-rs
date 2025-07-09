use std::collections::HashMap;

use vocab::ToString;

use crate::rml::parser::extractors::rcterm_to_string;
use crate::rml::types::Quad;

pub fn get_triples_strings(
    quad: &Quad<'_>,
    variable_map: &HashMap<String, String>,
) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let triple = &quad.triple;

    let sm = triple.sm;
    let sm_var = variable_map.get(&sm.tm_info.identifier).unwrap();

    let cls_templates = sm.classes.iter().map(|cls| {
        format!(
            "{} <{}> <{}>",
            sm_var,
            vocab::rdf::PROPERTY::TYPE.to_string(),
            rcterm_to_string(cls)
        )
    });
    result.extend(cls_templates);

    let pm_var = variable_map.get(&triple.pm.tm_info.identifier).unwrap();
    let om_var = variable_map.get(&triple.om.tm_info.identifier).unwrap();
    let pm_om_string = format!("{} {}", pm_var, om_var);

    let p_o_string = if let Some(lang) = &triple.om.language {
        format!("{}@{}", pm_om_string, lang)
    } else if let Some(dtype) = &triple.om.data_type {
        format!("{}^^{}", pm_om_string, rcterm_to_string(dtype))
    } else {
        pm_om_string
    };
    let s_p_o = format!("{} {}", sm_var, p_o_string);

    result.push(s_p_o);


    result
}
