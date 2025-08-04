use std::fmt::Debug;

use sophia_api::prelude::Iri;
use sophia_api::term::{FromTerm, Term, TermKind};
use sophia_inmem::graph::FastGraph;
use sophia_term::{ArcTerm, RcTerm};
use vocab::{ToString, PAIR};

use self::error::ParseError;
use super::error::NewRMLTranslationError;
use crate::new_rml::extractors::store::get_objects;
use crate::new_rml::rml_model::v2::core::expression_map::term_map::CommonTermMapInfo;

mod abstract_logical_source_extractor;
pub mod error;
mod expression_map;
mod graphmap_extractor;
mod input_map;
pub mod io;
mod logical_source;
mod logical_target;
mod logical_view;
mod objectmap_extractor;
mod pom_extractor;
mod predicatemap_extractor;
//mod rdb_logicalsource;
mod fnml;
mod refobject_extractor;
mod source;
pub mod store;
mod subjectmap_extractor;
mod target;
mod term_map_extractor;
pub mod tests;
pub mod triplesmap_extractor;
mod util;

pub type ExtractorResult<T> = Result<T, NewRMLTranslationError>;

pub trait TermMapExtractor<T: Debug> {
    fn create_shortcut_map(tm: CommonTermMapInfo) -> T;

    fn create_term_map<TTerm>(
        subj_ref: TTerm,
        graph_ref: &FastGraph,
    ) -> ExtractorResult<T>
    where
        TTerm: Term + Clone;

    fn extract_constant_term_map<TTerm>(map_const: TTerm) -> ExtractorResult<T>
    where
        TTerm: Term + Clone,
    {
        if let TermKind::BlankNode = map_const.kind() {
            return Err(ParseError::GenericError(
                "Constant-valued term map cannot be a BlankNode".to_string(),
            )
            .into());
        };

        let tm_info = CommonTermMapInfo::from_constant_value(map_const)?;

        Ok(Self::create_shortcut_map(tm_info))
    }

    fn extract_from_container<TTerm>(
        graph_ref: &FastGraph,
        container_map_subj_ref: TTerm,
    ) -> ExtractorResult<T>
    where
        TTerm: Term + Clone,
    {
        Self::extract_many_from_container(graph_ref, container_map_subj_ref)
            .and_then(|mut vec| vec.pop().ok_or(ParseError::Infallible.into()))
    }

    fn extract_many_from_container<TTerm>(
        graph_ref: &FastGraph,
        container_map_subj_ref: TTerm,
    ) -> ExtractorResult<Vec<T>>
    where
        TTerm: Term + Clone,
    {
        let map_preds = Self::get_map_preds();
        let const_preds = Self::get_shortcut_preds();
        let map_subj_vec = map_preds.iter().flat_map(|f| {
            get_objects(graph_ref, container_map_subj_ref.borrow_term(), f)
        });
        let map_const_obj_vec = const_preds.iter().flat_map(|f| {
            get_objects(graph_ref, container_map_subj_ref.borrow_term(), f)
        });

        let mut result: Vec<_> = map_subj_vec
            .map(|map_subj| Self::create_term_map(&map_subj, graph_ref))
            .collect::<ExtractorResult<_>>()?;

        let constant_tms = map_const_obj_vec
            .map(|map_const_obj_vec| {
                Self::extract_constant_term_map(&map_const_obj_vec)
            })
            .collect::<ExtractorResult<Vec<_>>>()?;

        result.extend(constant_tms);

        Ok(result)
    }

    fn get_shortcut_preds() -> Vec<RcTerm>;
    fn get_map_preds() -> Vec<RcTerm>;
}

pub trait Extractor<T> {
    fn extract_identifier<TTerm>(subj_ref: TTerm) -> Result<TTerm, ParseError>
    where
        TTerm: Term,
    {
        Ok(subj_ref)
    }

    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &FastGraph,
    ) -> ExtractorResult<T>
    where
        TTerm: Term + Clone;
}

pub trait FromVocab {
    fn to_rcterm(&self) -> RcTerm;
    fn to_arcterm(&self) -> ArcTerm;
}

impl<'a> FromVocab for PAIR<'a> {
    fn to_rcterm(&self) -> RcTerm {
        RcTerm::from_term(Iri::new_unchecked(format!("{}{}", self.0, self.1)))
    }
    fn to_arcterm(&self) -> ArcTerm {
        ArcTerm::from_term(Iri::new_unchecked(format!("{}{}", self.0, self.1)))
    }
}

pub fn stringify_rcterm<T>(term: T) -> Option<String>
where
    T: Term,
{
    match term.kind() {
        TermKind::Iri => Some(term.iri().unwrap().as_str().to_string()),
        TermKind::Literal => Some(term.lexical_form().unwrap().to_string()),
        TermKind::BlankNode => {
            Some(term.bnode_id().unwrap().as_str().to_string())
        }
        TermKind::Triple => None,
        TermKind::Variable => None,
    }
}
