use crate::rml::parser::rml_model::{
    PredicateObjectMap, TriplesMap
};
use crate::rml::parser::rml_model::term_map::{SubjectMap, TermMapType};
use crate::rml::parser::extractors::rcterm_to_string;
use sophia_api::term::TermKind;

// Core extractors

fn get_subject_map(tm: &TriplesMap) -> Result<&SubjectMap, String> {
    Ok(&tm.subject_map)
}

fn get_pom(tm: &TriplesMap, index: usize) -> Result<&PredicateObjectMap, String> {
    tm.po_maps.get(index)
        .ok_or(format!("No predicate-object map at index {}", index))
}

fn get_template_from_term_map_info(tm_info: &crate::rml::parser::rml_model::term_map::TermMapInfo) -> Result<String, String> {
    match &tm_info.term_map_type {
        TermMapType::Template => Ok(rcterm_to_string(&tm_info.term_value)),
        _ => Err("Not a template term map".to_string())
    }
}

fn get_reference_from_term_map_info(tm_info: &crate::rml::parser::rml_model::term_map::TermMapInfo) -> Result<String, String> {
    match &tm_info.term_map_type {
        TermMapType::Reference => Ok(rcterm_to_string(&tm_info.term_value)),
        _ => Err("Not a reference term map".to_string())
    }
}

fn get_constant_from_term_map_info(tm_info: &crate::rml::parser::rml_model::term_map::TermMapInfo) -> Result<String, String> {
    match &tm_info.term_map_type {
        TermMapType::Constant => Ok(rcterm_to_string(&tm_info.term_value)),
        _ => Err("Not a constant term map".to_string())
    }
}

fn get_term_type_from_term_map_info(tm_info: &crate::rml::parser::rml_model::term_map::TermMapInfo) -> Result<String, String> {
    match &tm_info.term_type {
        Some(TermKind::Iri) => Ok("http://www.w3.org/ns/r2rml#IRI".to_string()),
        Some(TermKind::BlankNode) => Ok("http://www.w3.org/ns/r2rml#BlankNode".to_string()),
        Some(TermKind::Literal) => Ok("http://www.w3.org/ns/r2rml#Literal".to_string()),
        Some(TermKind::Triple) => Ok("http://www.w3.org/ns/r2rml#IRI".to_string()), // Fallback
        Some(TermKind::Variable) => Ok("http://www.w3.org/ns/r2rml#IRI".to_string()), // Fallback
        None => Ok("http://www.w3.org/ns/r2rml#IRI".to_string()), // Default
    }
}

fn get_value_from_term_map_info(tm_info: &crate::rml::parser::rml_model::term_map::TermMapInfo) -> Result<String, String> {
    match &tm_info.term_map_type {
        TermMapType::Constant => get_constant_from_term_map_info(tm_info),
        TermMapType::Reference => get_reference_from_term_map_info(tm_info),
        TermMapType::Template => get_template_from_term_map_info(tm_info),
        _ => Err("Unsupported term map type".to_string())
    }
}

// Extractor functions for tests

pub fn extract_triples_map_identifier(tm: &TriplesMap) -> Result<String, String> {
    Ok(tm.identifier.clone())
}

pub fn extract_predicate_object_maps_count(tm: &TriplesMap) -> Result<usize, String> {
    Ok(tm.po_maps.len())
}

pub fn extract_logical_source_iterator(tm: &TriplesMap) -> Result<String, String> {
    tm.logical_source.iterator.clone()
        .ok_or("No iterator found in logical source".to_string())
}

pub fn extract_logical_source_reference_formulation(tm: &TriplesMap) -> Result<String, String> {
    Ok(rcterm_to_string(&tm.logical_source.reference_formulation))
}

pub fn extract_subject_map_template(tm: &TriplesMap) -> Result<String, String> {
    let sm = get_subject_map(tm)?;
    get_value_from_term_map_info(&sm.tm_info)
}

pub fn extract_subject_map_has_classes(tm: &TriplesMap) -> Result<bool, String> {
    let sm = get_subject_map(tm)?;
    Ok(!sm.classes.is_empty())
}

pub fn extract_subject_map_term_type(tm: &TriplesMap) -> Result<String, String> {
    let sm = get_subject_map(tm)?;
    get_term_type_from_term_map_info(&sm.tm_info)
}

pub fn extract_subject_map_first_class(tm: &TriplesMap) -> Result<String, String> {
    let sm = get_subject_map(tm)?;
    let first_class = sm.classes.first()
        .ok_or("No classes found in subject map")?;
    Ok(rcterm_to_string(first_class))
}

pub fn extract_predicate_from_pom(triplesmap: &TriplesMap, index: usize) -> Result<String, String> {
    let pom = get_pom(triplesmap, index)?;
    let pm = pom.predicate_maps.first()
        .ok_or("No predicate map found")?;
    get_value_from_term_map_info(&pm.tm_info)
}

pub fn extract_object_reference_from_pom(triplesmap: &TriplesMap, index: usize) -> Result<String, String> {
    let pom = get_pom(triplesmap, index)?;
    let om = pom.object_maps.first()
        .ok_or("No object map found")?;
    get_value_from_term_map_info(&om.tm_info)
}

pub fn extract_object_template_from_pom(triplesmap: &TriplesMap, index: usize) -> Result<String, String> {
    let pom = get_pom(triplesmap, index)?;
    let om = pom.object_maps.first()
        .ok_or("No object map found")?;
    get_value_from_term_map_info(&om.tm_info)
}

pub fn extract_object_term_type_from_pom(triplesmap: &TriplesMap, index: usize) -> Result<String, String> {
    let pom = get_pom(triplesmap, index)?;
    let om = pom.object_maps.first()
        .ok_or("No object map found")?;
    get_term_type_from_term_map_info(&om.tm_info)
}

pub fn extract_has_ref_object_map_in_pom(triplesmap: &TriplesMap, index: usize) -> Result<bool, String> {
    let pom = get_pom(triplesmap, index)?;
    let om = pom.object_maps.first()
        .ok_or("No object map found")?;
    Ok(om.parent_tm.is_some())
}

pub fn extract_ref_object_map_parent_triples_map(triplesmap: &TriplesMap, index: usize) -> Result<String, String> {
    let pom = get_pom(triplesmap, index)?;
    let om = pom.object_maps.first()
        .ok_or("No object map found")?;
    let parent_tm = om.parent_tm.as_ref()
        .ok_or("No parent triples map found")?;
    Ok(rcterm_to_string(parent_tm))
}

pub fn extract_join_condition_child(triplesmap: &TriplesMap, index: usize) -> Result<String, String> {
    let pom = get_pom(triplesmap, index)?;
    let om = pom.object_maps.first()
        .ok_or("No object map found")?;
    let join = om.join_condition.as_ref()
        .ok_or("No join condition found")?;
    join.child_attributes.first()
        .cloned()
        .ok_or("No child attributes in join condition".to_string())
}

pub fn extract_join_condition_parent(triplesmap: &TriplesMap, index: usize) -> Result<String, String> {
    let pom = get_pom(triplesmap, index)?;
    let om = pom.object_maps.first()
        .ok_or("No object map found")?;
    let join = om.join_condition.as_ref()
        .ok_or("No join condition found")?;
    join.parent_attributes.first()
        .cloned()
        .ok_or("No parent attributes in join condition".to_string())
}

pub fn extract_subject_map_classes(triplesmap: &TriplesMap) -> Result<Vec<String>, String> {
    let sm = get_subject_map(triplesmap)?;
    let mut classes = Vec::new();
    for class_rc_term in &sm.classes {
        classes.push(rcterm_to_string(class_rc_term));
    }
    Ok(classes)
}

// Placeholder source extractors (not used in CSV tests but needed for compilation)
pub fn extract_source_path(_triplesmap: &TriplesMap) -> Result<String, String> {
    Err("Source path extraction not implemented for old RML".to_string())
}

pub fn extract_source_root(_triplesmap: &TriplesMap) -> Result<String, String> {
    Err("Source root extraction not implemented for old RML".to_string())
}

pub fn extract_source_type(_triplesmap: &TriplesMap) -> Result<String, String> {
    Err("Source type extraction not implemented for old RML".to_string())
}
