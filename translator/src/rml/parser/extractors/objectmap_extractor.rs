use sophia_api::term::{TTerm, TermKind};
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use super::error::ParseError;
use super::{ExtractorResult, FromVocab, TermMapExtractor};
use crate::rml::parser::extractors::store::{get_object, get_objects};
use crate::rml::parser::extractors::Extractor;
use crate::rml::parser::rml_model::join::JoinCondition;
use crate::rml::parser::rml_model::term_map::{
    GraphMap, ObjectMap, TermMapInfo, TermMapType,
};
use crate::rml::parser::IriString;

fn extract_join_condition(
    subject_ref: &RcTerm,
    graph_ref: &FastGraph,
) -> ExtractorResult<JoinCondition> {
    let jc_pred = vocab::r2rml::PROPERTY::JOINCONDITION.to_rcterm();
    let jc_iri = get_object(graph_ref, subject_ref, &jc_pred)?;

    let child_pred = vocab::r2rml::PROPERTY::CHILD.to_rcterm();
    let child_attributes = get_objects(graph_ref, &jc_iri, &child_pred)
        .iter()
        .map(|term| term.value().to_string())
        .collect();

    let parent_pred = vocab::r2rml::PROPERTY::PARENT.to_rcterm();
    let parent_attributes = get_objects(graph_ref, &jc_iri, &parent_pred)
        .iter()
        .map(|term| term.value().to_string())
        .collect();

    Ok(JoinCondition {
        parent_attributes,
        child_attributes,
    })
}

fn extract_parent_tm(
    subject_ref: &RcTerm,
    graph_ref: &FastGraph,
) -> ExtractorResult<IriString> {
    let parent_tm_pred = vocab::r2rml::PROPERTY::PARENTTRIPLESMAP.to_rcterm();
    get_object(graph_ref, subject_ref, &parent_tm_pred)
        .map(|rcterm| IriString::new(rcterm.value()).unwrap())
}

impl TermMapExtractor<ObjectMap> for ObjectMap {
    fn create_constant_map(tm_info: TermMapInfo) -> ObjectMap {
        if tm_info.term_type == Some(TermKind::BlankNode) {
            panic!("Constant-valued ObjectMap has to have an IRI or a Literal as value");
        }

        ObjectMap {
            tm_info,
            parent_tm: None,
            join_condition: None,
            data_type: None,
            language: None,
            graph_maps: vec![],
        }
    }
    fn create_term_map(
        subj_ref: &RcTerm,
        graph_ref: &FastGraph,
    ) -> super::ExtractorResult<ObjectMap> {
        let dtype_pred = vocab::r2rml::PROPERTY::DATATYPE.to_rcterm();
        let data_type: Option<IriString> =
            get_object(graph_ref, subj_ref, &dtype_pred)
                .ok()
                .map(|tshared| tshared.map(|i| i.to_string()))
                .and_then(|tstring| tstring.try_into().ok());

        let lang_pred = vocab::r2rml::PROPERTY::LANGUAGE.to_rcterm();
        let language = get_object(graph_ref, subj_ref, &lang_pred)
            .ok()
            .map(|tshared| tshared.value_raw().0.to_string());
        let parent_tm = extract_parent_tm(subj_ref, graph_ref).ok();
        let join_condition = extract_join_condition(subj_ref, graph_ref).ok();

        let mut tm_info_res = TermMapInfo::extract_self(subj_ref, graph_ref);
        if tm_info_res.is_err() && parent_tm.is_none() {
            return Err(ParseError::GenericError("Object Map doesn't have either parent triplesmap nor term map type".to_string()));
        }

        if tm_info_res.is_err() && parent_tm.is_some() {
            let identifier = subj_ref.to_string();
            tm_info_res = Ok(TermMapInfo {
                identifier,
                term_type: Some(TermKind::Iri),
                ..Default::default()
            });
        }

        let mut tm_info = tm_info_res?;
        if tm_info.term_type.is_none() {
            let mut inferred_term_type = match tm_info.term_map_type {
                TermMapType::Reference => Some(TermKind::Literal),
                TermMapType::Template => Some(TermKind::Iri),
                TermMapType::Function => Some(TermKind::Literal),
                _ => None,
            };

            if inferred_term_type.is_none() {
                if language.is_some() || data_type.is_some() {
                    inferred_term_type = Some(TermKind::Literal);
                } else {
                    inferred_term_type = Some(TermKind::Iri);
                }
            }

            tm_info.term_type = inferred_term_type;
        }

        let graph_maps =
            GraphMap::extract_many_from_container(graph_ref, subj_ref)?;

        Ok(ObjectMap {
            tm_info,
            parent_tm,
            join_condition,
            data_type,
            language,
            graph_maps,
        })
    }

    fn get_const_pred() -> RcTerm {
        vocab::r2rml::PROPERTY::OBJECT.to_rcterm()
    }

    fn get_map_pred() -> RcTerm {
        vocab::r2rml::PROPERTY::OBJECTMAP.to_rcterm()
    }

    fn get_term_map_info(&self) -> TermMapInfo {
        self.tm_info.clone()
    }
}

#[cfg(test)]
mod tests {

    use sophia_api::triple::Triple;
    use sophia_api::graph::Graph;
    use super::*;
    use crate::import_test_mods;
    use crate::rml::parser::rml_model::term_map::TermMapType;

    import_test_mods!();

    #[test]
    fn map_object_test() -> ExtractorResult<()> {
        let graph: FastGraph = load_graph!("rml/sample_mapping.ttl")?;
        let map_pred = vocab::r2rml::PROPERTY::OBJECTMAP.to_rcterm();
        let container_vec = graph
            .triples_with_p(&map_pred)
            .flatten()
            .map(|trip| trip.s().to_owned());

        let obj_maps: Vec<_> = container_vec
            .flat_map(|objmap_container| {
                ObjectMap::extract_many_from_container(
                    &graph,
                    &objmap_container,
                )
            })
            .flatten()
            .collect();

        assert_eq!(obj_maps.len(), 2);
        obj_maps.iter().for_each(|om| {
            assert_eq!(om.tm_info.term_type, Some(TermKind::Literal));
            assert_eq!(om.tm_info.term_map_type, TermMapType::Reference);
        });

        Ok(())
    }
}
