use sophia_api::term::Term;
use sophia_term::RcTerm;

use super::error::ParseError;
use super::store::get_objects;
use super::TermMapExtractor;
use crate::new_rml::extractors::{Extractor, FromVocab};
use crate::new_rml::rml_model::v2::core::expression_map::term_map::{
    GraphMap, SubjectMap, TermMap,
};

impl TermMapExtractor<SubjectMap> for SubjectMap {
    fn create_constant_map(tm: TermMap) -> SubjectMap {
        match tm.term_type {
            ref term if *term == vocab::rml_core::CLASS::IRI.to_rcterm() => {
                SubjectMap {
                    term_map:   tm,
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
        let term_map =
            TermMap::extract_self(subj_ref.borrow_term(), graph_ref)?;

        if term_map.term_type == vocab::rml_core::CLASS::LITERAL.to_rcterm() {
            return Err(ParseError::GenericError(
                    "SubjectMap can only have rr:Iri or rr:BlankNode as rr:termType!"
                        .to_string(),
                ));
        }

        let class_pred = vocab::r2rml::PROPERTY::CLASS.to_rcterm();

        let classes: Vec<RcTerm> =
            get_objects(graph_ref, subj_ref.borrow_term(), &class_pred)
                .iter()
                .map(|item| item.clone())
                .collect();

        let graph_maps = GraphMap::extract_many_from_container(
            graph_ref,
            subj_ref.borrow_term(),
        )
        .ok()
        .into_iter()
        .flatten()
        .collect();

        Ok(SubjectMap {
            term_map,
            classes,
            graph_maps,
        })
    }

    fn get_const_preds() -> Vec<RcTerm> {
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
                )))
            } else {
                sms.pop().ok_or(ParseError::Infallible)
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
    use crate::new_rml::rml_model::v2::core::expression_map::ExpressionValueEnum;
    use crate::{load_graph, test_case};

    #[test]
    fn create_subjectmap_test() -> ExtractorResult<()> {
        let graph = load_graph!("sample_mapping.ttl")?;
        let sub_pred = vocab::r2rml::PROPERTY::SUBJECTMAP.to_rcterm();
        let triple = graph
            .triples_matching(Any, [sub_pred], Any)
            .next()
            .unwrap()
            .unwrap();
        let sub_ref = triple.o();
        let subj_map = SubjectMap::create_term_map(sub_ref, &graph)?;

        assert_eq!(
            subj_map.term_map.expression.get_value_type_enum()?,
            ExpressionValueEnum::Template
        );
        assert!(subj_map.classes.len() == 0);

        Ok(())
    }
}
