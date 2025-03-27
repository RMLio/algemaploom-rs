use sophia_api::term::TermKind;

use super::{Extractor, ExtractorResult, FromVocab, TermMapExtractor};
use crate::rml::parser::extractors::error::ParseError;
use crate::rml::parser::rml_model::term_map::{GraphMap, TermMapInfo};

impl TermMapExtractor<GraphMap> for GraphMap {
    fn get_term_map_info(&self) -> TermMapInfo {
        self.tm_info.clone()
    }

    fn create_constant_map(tm_info: TermMapInfo) -> GraphMap {
        if tm_info.term_type == Some(TermKind::Literal) {
            panic!("Constant-valued GraphMap has to be either an IRI or a BlankNode");
        }
        Self { tm_info }
    }

    fn create_term_map(
        subj_ref: &sophia_term::RcTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<GraphMap> {
        let mut tm_info = TermMapInfo::extract_self(subj_ref, graph_ref)?;

        tm_info = match tm_info.term_type{
            Some(ttype) if ttype == TermKind::Literal || ttype== TermKind::Variable => {

                return Err(ParseError::GenericError("GraphMap can only have either rr:Iri or rr:BNode as rr:termType!".to_string()).into())
            },
            Some(_) => tm_info,
            None => TermMapInfo{term_type: Some(TermKind::Iri), ..tm_info},
        };

        Ok(GraphMap { tm_info })
    }

    fn get_const_pred() -> sophia_term::RcTerm {
        vocab::r2rml::PROPERTY::GRAPH.to_rcterm()
    }

    fn get_map_pred() -> sophia_term::RcTerm {
        vocab::r2rml::PROPERTY::GRAPHMAP.to_rcterm()
    }

    fn extract_many_from_container(
        graph_ref: &sophia_inmem::graph::FastGraph,
        container_map_subj_ref: &sophia_term::RcTerm,
    ) -> super::ExtractorResult<Vec<GraphMap>> {
        let map_pred = Self::get_map_pred();
        let const_pred = Self::get_const_pred();
        let map_subj_vec = super::store::get_objects(
            graph_ref,
            container_map_subj_ref,
            &map_pred,
        );
        let map_const_obj_vec = super::store::get_objects(
            graph_ref,
            container_map_subj_ref,
            &const_pred,
        );

        let mut result: Vec<_> = map_subj_vec
            .iter()
            .map(|map_subj| Self::create_term_map(map_subj, graph_ref))
            .collect::<super::ExtractorResult<_>>()?;

        let constant_tms: Vec<_> = map_const_obj_vec
            .iter()
            .map(|map_const_obj_vec| {
                Self::extract_constant_term_map(map_const_obj_vec)
            })
            .collect::<ExtractorResult<Vec<_>>>()?;

        result.extend(constant_tms);

        Ok(result)
    }
}
