use crate::new_rml::extractors::store::get_object;
use crate::new_rml::extractors::FromVocab;
use crate::new_rml::rml_model::v2::core::expression_map::term_map::SubjectMap;
use crate::new_rml::rml_model::v2::core::expression_map::{
    BaseExpressionMapEnum, ExpressionMapEnum,
};
use crate::new_rml::rml_model::v2::core::{
    AbstractLogicalSourceEnum, PredicateObjectMap, TriplesMap,
};
use crate::new_rml::rml_model::v2::io::source::{LogicalSource, Source};
use crate::new_rml::rml_model::v2::TermMapEnum;

// Core extractors

fn get_subject_map(tm: &TriplesMap) -> Result<&SubjectMap, String> {
    tm.subject_map
        .try_unwrap_subject_map_ref()
        .map_err(|_| "Subject map is not a standard subject map".to_string())
}

fn get_pom(
    tm: &TriplesMap,
    index: usize,
) -> Result<&PredicateObjectMap, String> {
    tm.predicate_object_map_vec
        .get(index)
        .ok_or(format!("No predicate-object map at index {}", index))
}

fn get_value_from_base_expression_map(
    base_expression_map: &BaseExpressionMapEnum,
) -> Result<String, String> {
    match base_expression_map {
        BaseExpressionMapEnum::Template(v)
        | BaseExpressionMapEnum::Reference(v)
        | BaseExpressionMapEnum::Constant(v) => Ok(v.to_string()),
        BaseExpressionMapEnum::Unknown { type_iri, term_val } => {
            Err(format!(
                "Unknown expression map detected with type {:?} and value {:?}",
                type_iri, term_val
            ))
        }
    }
}

fn get_expression_value(
    expression_map: &ExpressionMapEnum,
) -> Result<String, String> {
    match expression_map {
        ExpressionMapEnum::BaseExpressionMap(base_expression_map_enum) => {
            get_value_from_base_expression_map(
                base_expression_map_enum,
            )
        }
        ExpressionMapEnum::FunctionExpressionMap(function_expression_map) => {
            todo!()
        }
    }
}

fn get_expression_value_from_term_map(
    term_map: &TermMapEnum,
) -> Result<String, String> {
    get_expression_value(&term_map.as_ref().expression)
}

fn get_logical_source(tm: &TriplesMap) -> Result<&LogicalSource, String> {
    match &tm.abs_logical_source {
        AbstractLogicalSourceEnum::LogicalSource(ls) => Ok(ls),
        AbstractLogicalSourceEnum::LogicalView(_) => {
            Err("LogicalView not supported".to_string())
        }
    }
}

fn get_source(tm: &TriplesMap) -> Result<&Source, String> {
    Ok(&get_logical_source(tm)?.source)
}

fn stringify_rcterm<T: sophia_api::prelude::Term + Clone>(
    term: &T,
) -> Result<String, String> {
    crate::new_rml::extractors::stringify_term(term.clone())
        .ok_or("Failed to stringify term".to_string())
}

// Public extractors
pub fn extract_base_iri(triplesmap: &TriplesMap) -> Result<String, String> {
    Ok(triplesmap.base_iri.clone())
}

pub fn extract_triples_map_identifier(
    triplesmap: &TriplesMap,
) -> Result<String, String> {
    stringify_rcterm(&triplesmap.identifier)
}

pub fn extract_predicate_object_maps_count(
    triplesmap: &TriplesMap,
) -> Result<usize, String> {
    Ok(triplesmap.predicate_object_map_vec.len())
}

pub fn extract_logical_source_iterator(
    triplesmap: &TriplesMap,
) -> Result<String, String> {
    let ls = get_logical_source(triplesmap)?;
    ls.iterable
        .iterator
        .clone()
        .ok_or("No iterator defined".to_string())
}

pub fn extract_logical_source_reference_formulation(
    triplesmap: &TriplesMap,
) -> Result<String, String> {
    let ls = get_logical_source(triplesmap)?;
    ls.iterable
        .reference_formulation
        .as_ref()
        .and_then(|rf| crate::new_rml::extractors::stringify_term(&rf.iri))
        .ok_or("No reference formulation defined".to_string())
}

pub fn extract_source_type(triplesmap: &TriplesMap) -> Result<String, String> {
    let source = get_source(triplesmap)?;
    stringify_rcterm(&source.kind.type_iri)
}

// Subject map extractors
pub fn extract_subject_map_template(
    triplesmap: &TriplesMap,
) -> Result<String, String> {
    get_expression_value_from_term_map(&triplesmap.subject_map)
}

pub fn extract_subject_map_has_classes(
    triplesmap: &TriplesMap,
) -> Result<bool, String> {
    let sm = get_subject_map(triplesmap)?;
    Ok(!sm.classes.is_empty())
}

pub fn extract_subject_map_first_class(
    triplesmap: &TriplesMap,
) -> Result<String, String> {
    let sm = get_subject_map(triplesmap)?;
    let class = sm.classes.first().ok_or("No classes defined")?;
    stringify_rcterm(class)
}

pub fn extract_subject_map_term_type(
    triplesmap: &TriplesMap,
) -> Result<String, String> {
    let sm = get_subject_map(triplesmap)?;
    stringify_rcterm(&sm.term_map_info.term_type)
}

pub fn extract_predicate_from_pom(
    triplesmap: &TriplesMap,
    index: usize,
) -> Result<String, String> {
    let pom = get_pom(triplesmap, index)?;
    let pm = pom.predicate_map_vec.first().ok_or("No predicate maps")?;
    get_expression_value_from_term_map(pm)
}

pub fn extract_object_reference_from_pom(
    triplesmap: &TriplesMap,
    index: usize,
) -> Result<String, String> {
    let pom = get_pom(triplesmap, index)?;
    let om = pom.object_map_vec.first().ok_or("No object maps")?;
    get_expression_value_from_term_map(om)
}

pub fn extract_object_template_from_pom(
    triplesmap: &TriplesMap,
    index: usize,
) -> Result<String, String> {
    extract_object_reference_from_pom(triplesmap, index)
}

pub fn extract_object_term_type_from_pom(
    triplesmap: &TriplesMap,
    index: usize,
) -> Result<String, String> {
    let pom = get_pom(triplesmap, index)?;
    let om = pom.object_map_vec.first().ok_or("No object maps")?;
    stringify_rcterm(&om.as_ref().term_type)
}

pub fn extract_object_constant_from_pom(triplesmap: &TriplesMap, index: usize) -> Result<String, String> {
    extract_object_reference_from_pom(triplesmap, index)
}

pub fn extract_subject_map_constant(triplesmap: &TriplesMap) -> Result<String, String> {
    let subject_map = get_subject_map(triplesmap)?;
    get_expression_value(&subject_map.term_map_info.expression)
}

pub fn extract_source_path(triplesmap: &TriplesMap) -> Result<String, String> {
    let source = get_source(triplesmap)?;
    let path_predicate = vocab::rml_io::PROPERTY::PATH.to_rcterm();

    match get_object(
        &source.kind.metadata,
        &source.kind.subj_iri,
        &path_predicate,
    ) {
        Ok(path_term) => {
            crate::new_rml::extractors::stringify_term(path_term)
                .ok_or("Failed to stringify path".to_string())
        }
        Err(e) => {
            Err(format!("No source path found in source metadata: {}", e))
        }
    }
}

pub fn extract_source_root(triplesmap: &TriplesMap) -> Result<String, String> {
    let source = get_source(triplesmap)?;
    let root_predicate = vocab::rml_io::PROPERTY::ROOT.to_rcterm();

    match get_object(
        &source.kind.metadata,
        &source.kind.subj_iri,
        &root_predicate,
    ) {
        Ok(root_term) => {
            crate::new_rml::extractors::stringify_term(root_term)
                .ok_or("Failed to stringify root".to_string())
        }
        Err(e) => {
            Err(format!("No source root found in source metadata: {}", e))
        }
    }
}

// Join condition extractors
pub fn extract_has_ref_object_map_in_pom(
    triplesmap: &TriplesMap,
    pom_index: usize,
) -> Result<bool, String> {
    let pom = get_pom(triplesmap, pom_index)?;
    Ok(!pom.ref_object_map.is_empty())
}

pub fn extract_ref_object_map_parent_triples_map(
    triplesmap: &TriplesMap,
    pom_index: usize,
) -> Result<String, String> {
    let pom = get_pom(triplesmap, pom_index)?;
    if pom.ref_object_map.is_empty() {
        return Err("No RefObjectMap found in this POM".to_string());
    }

    let ref_obj_map = &pom.ref_object_map[0]; // Get first RefObjectMap
    stringify_rcterm(&ref_obj_map.ptm_iri)
}

pub fn extract_join_condition_child(
    triplesmap: &TriplesMap,
    pom_index: usize,
) -> Result<String, String> {
    let pom = get_pom(triplesmap, pom_index)?;
    if pom.ref_object_map.is_empty() {
        return Err("No RefObjectMap found in this POM".to_string());
    }

    let ref_obj_map = &pom.ref_object_map[0];
    if ref_obj_map.join_condition.is_empty() {
        return Err("No join condition found in RefObjectMap".to_string());
    }

    let join_condition = &ref_obj_map.join_condition[0];
    get_expression_value(&join_condition.child)
        .map_err(|_| "No child value in join condition".to_string())
}

pub fn extract_join_condition_parent(
    triplesmap: &TriplesMap,
    pom_index: usize,
) -> Result<String, String> {
    let pom = get_pom(triplesmap, pom_index)?;
    if pom.ref_object_map.is_empty() {
        return Err("No RefObjectMap found in this POM".to_string());
    }

    let ref_obj_map = &pom.ref_object_map[0];
    if ref_obj_map.join_condition.is_empty() {
        return Err("No join condition found in RefObjectMap".to_string());
    }

    let join_condition = &ref_obj_map.join_condition[0];
    get_expression_value(&join_condition.parent)
        .map_err(|_| "No parent value in join condition".to_string())
}

pub fn extract_subject_map_classes(
    triplesmap: &TriplesMap,
) -> Result<Vec<String>, String> {
    let subject_map = get_subject_map(triplesmap)?;

    let mut classes = Vec::new();
    for class_term in &subject_map.classes {
        if let Ok(class_str) = stringify_rcterm(class_term) {
            classes.push(class_str);
        }
    }

    Ok(classes)
}
