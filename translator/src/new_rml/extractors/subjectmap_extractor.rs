use sophia_api::term::Term;
use sophia_term::RcTerm;

use super::error::ParseError;
use super::store::get_objects;
use super::TermMapExtractor;
use crate::new_rml::error::NewRMLTranslationError;
use crate::new_rml::extractors::{Extractor, FromVocab};
use crate::new_rml::rml_model::v2::core::expression_map::term_map::{
    GraphMap, SubjectMap, CommonTermMapInfo,
};

impl TermMapExtractor<SubjectMap> for SubjectMap {
    fn create_shortcut_map(tm: CommonTermMapInfo) -> SubjectMap {
        match tm.term_type {
            ref term if *term == vocab::rml_core::CLASS::IRI.to_rcterm() => {
                SubjectMap {
                    term_map_info:   tm,
                    classes:    vec![],
                    graph_maps: vec![],
                }
            }
            _ => {
                panic!(
                    "Constant-valued SubjectMap has to have an IRI as value"
                );
            }
        }
    }

    fn create_term_map<TS>(
        subj_ref: TS,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<SubjectMap>
    where
        TS: Term + Clone,
    {
        let term_map_info =
            CommonTermMapInfo::extract_self(subj_ref.borrow_term(), graph_ref)?;

        if term_map_info.term_type == vocab::rml_core::CLASS::LITERAL.to_rcterm() {
            return Err(ParseError::GenericError(
                    "SubjectMap can only have rml:IRI rml:UnsafeIRI, rml:URI, rml:UnsafeURI or rml:BlankNode as rml:termType!"
                        .to_string(),
                ).into());
        }

        let class_pred = vocab::rml_core::PROPERTY::CLASS.to_rcterm();

        let classes: Vec<RcTerm> =
            get_objects(graph_ref, subj_ref.borrow_term(), &class_pred);

        let graph_maps = GraphMap::extract_many_from_container(
            graph_ref,
            subj_ref.borrow_term(),
        )?
        .into_iter()
        .filter(|gm| !gm.is_default_graph())
        .collect();

        Ok(SubjectMap {
            term_map_info,
            classes,
            graph_maps,
        })
    }

    fn get_shortcut_preds() -> Vec<RcTerm> {
        vec![
            vocab::r2rml::PROPERTY::SUBJECT.to_rcterm(),
            vocab::rml_core::PROPERTY::SUBJECT.to_rcterm(),
        ]
    }

    fn get_map_preds() -> Vec<RcTerm> {
        vec![
            vocab::r2rml::PROPERTY::SUBJECTMAP.to_rcterm(),
            vocab::rml_core::PROPERTY::SUBJECT_MAP.to_rcterm(),
        ]
    }

    fn extract_from_container<TTerm>(
        graph_ref: &sophia_inmem::graph::FastGraph,
        container_map_subj_ref: TTerm,
    ) -> super::ExtractorResult<SubjectMap>
    where
        TTerm: Term + Clone,
    {
        Self::extract_many_from_container(
            graph_ref,
            container_map_subj_ref.borrow_term(),
        )
        .and_then(|mut sms| {
            if sms.len() > 1 {
                Err(ParseError::GenericError(format!(
                    "There can only be ONE subject map for {:?}",
                    container_map_subj_ref
                ))
                .into())
            } else {
                sms.pop().ok_or(ParseError::NoTermMapFoundError(format!(
                    "No subject map found for the triples map {:?}",
                    container_map_subj_ref
                )).into())
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    use sophia_api::graph::Graph;
    use sophia_api::prelude::Any;
    use sophia_api::triple::Triple;

    use crate::new_rml::extractors::io::load_graph_bread;
    use crate::new_rml::extractors::{
        ExtractorResult, FromVocab, TermMapExtractor,
    };
    use crate::new_rml::rml_model::v2::core::expression_map::term_map::SubjectMap;
    use crate::new_rml::rml_model::v2::core::expression_map::ExpressionMapTypeEnum;
    use crate::{load_graph, test_case};

    #[test]
    fn create_subjectmap_test() -> ExtractorResult<()> {
        let graph = load_graph!("rml/sample_mapping.ttl")?;
        let sub_pred = vocab::r2rml::PROPERTY::SUBJECTMAP.to_rcterm();
        let triple = graph
            .triples_matching(Any, [sub_pred], Any)
            .next()
            .unwrap()
            .unwrap();
        let sub_ref = triple.o();
        let subj_map = SubjectMap::create_term_map(sub_ref, &graph)?;

        assert_eq!(
            subj_map.term_map_info.expression.get_map_type_enum()?,
            ExpressionMapTypeEnum::Template
        );
        assert!(subj_map.classes.is_empty());

        Ok(())
    }
}
