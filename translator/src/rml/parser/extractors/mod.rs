use std::fmt::Debug;
use std::rc::Rc;

use sophia_inmem::graph::FastGraph;
use sophia_term::{RcTerm, Term};
use vocab::{ToString, PAIR};

use self::error::ParseError;
use super::extractors::store::get_objects;
use super::rml_model::term_map::TermMapInfo;
use super::TermString;

pub mod error;
mod functionmap_extractor;
mod graphmap_extractor;
pub mod io;
mod logicalsource_extractor;
mod logicaltarget_extractor;
mod objectmap_extractor;
mod pom_extractor;
mod predicatemap_extractor;
mod source;
mod store;
mod subjectmap_extractor;
mod term_map_info_extractor;
pub mod triplesmap_extractor;
mod util;
mod config_extractor;
mod rdb_logicalsource;

pub type ExtractorResult<T> = Result<T, ParseError>;

pub trait TermMapExtractor<T: Debug> {
    fn get_term_map_info(&self) -> TermMapInfo;

    fn create_constant_map(tm_info: TermMapInfo) -> T;

    fn create_term_map(
        subj_ref: &RcTerm,
        graph_ref: &FastGraph,
    ) -> ExtractorResult<T>;

    fn extract_constant_term_map(
        map_const: &Term<Rc<str>>,
    ) -> ExtractorResult<T> {
        if let Term::BNode(_) = map_const {
            return Err(ParseError::GenericError(format!(
                "Constant-valued term map cannot be a BlankNode"
            )))
        };

        let tm_info = TermMapInfo::from_constant_value(map_const.clone());

        Ok(Self::create_constant_map(tm_info))
    }

    fn extract_from_container(
        graph_ref: &FastGraph,
        container_map_subj_ref: &RcTerm,
    ) -> ExtractorResult<T> {
        Self::extract_many_from_container(graph_ref, container_map_subj_ref)
            .and_then(|mut vec| vec.pop().ok_or(ParseError::Infallible))
    }

    fn extract_many_from_container(
        graph_ref: &FastGraph,
        container_map_subj_ref: &RcTerm,
    ) -> ExtractorResult<Vec<T>> {
        let map_pred = Self::get_map_pred();
        let const_pred = Self::get_const_pred();
        let map_subj_vec =
            get_objects(graph_ref, container_map_subj_ref, &map_pred);
        let map_const_obj_vec =
            get_objects(graph_ref, container_map_subj_ref, &const_pred);

        let mut result: Vec<_> = map_subj_vec
            .iter()
            .map(|map_subj| Self::create_term_map(map_subj, graph_ref))
            .collect::<ExtractorResult<_>>()?;

        let constant_tms = map_const_obj_vec
            .iter()
            .map(|map_const_obj_vec| {
                Self::extract_constant_term_map(map_const_obj_vec)
            })
            .collect::<ExtractorResult<Vec<_>>>()?;

        result.extend(constant_tms);

        if result.is_empty() {
            Err(ParseError::NoTermMapFoundError(format!(
                "0 TermMap of type {} found for {}",
                map_pred, container_map_subj_ref
            )))
        } else {
            Ok(result)
        }
    }

    fn get_const_pred() -> RcTerm;
    fn get_map_pred() -> RcTerm;
}

pub trait Extractor<T> {
    fn extract_identifier(subj_ref: RcTerm) -> Result<TermString, ParseError> {
        let identifier =
            subj_ref.to_owned().map(|i| i.to_string());
        Ok(identifier)
    }

    fn extract_self(
        subject_ref: &RcTerm,
        graph_ref: &FastGraph,
    ) -> ExtractorResult<T>;
}

pub trait FromVocab {
    fn to_rcterm(&self) -> RcTerm;

    fn to_term(&self) -> Term<String>;
}

impl<'a> FromVocab for PAIR<'a> {
    fn to_term(&self) -> Term<String> {
        Term::new_iri(self.to_string()).unwrap()
    }
    fn to_rcterm(&self) -> RcTerm {
        Term::new_iri(self.to_string().as_ref()).unwrap()
    }
}
