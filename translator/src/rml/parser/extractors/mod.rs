//!
//! Provides various methods to parse RML v1.1.2 documents into the
//! RML data model described in the [data model module](crate::rml::parser::rml_model)
//!
//!
use std::fmt::Debug;

use sophia_api::term::IriRef;
use sophia_inmem::graph::FastGraph;
use sophia_term::{ArcTerm, RcTerm};
use vocab::{ToString, PAIR};

use self::error::ParseError;
use super::extractors::store::get_objects;
use super::rml_model::term_map::TermMapInfo;
use crate::rml::error::RMLTranslationError;

mod config_extractor;
pub mod error;
mod functionmap_extractor;
mod graphmap_extractor;
pub mod io;
mod ldes_extractor;
mod logicalsource_extractor;
mod logicaltarget_extractor;
mod objectmap_extractor;
mod pom_extractor;
mod predicatemap_extractor;
mod rdb_logicalsource;
mod source;
mod store;
mod subjectmap_extractor;
mod term_map_info_extractor;
pub mod triplesmap_extractor;
mod util;

pub type ExtractorResult<T> = Result<T, RMLTranslationError>;

/// A trait for extracting RML [term maps](https://rml.io/specs/rml/#term-map).
pub trait TermMapExtractor<T: Debug> {

    /// Returns the [TermMapInfo] of the underlying term map. 
    fn get_term_map_info(&self) -> TermMapInfo;

    /// Given an input [TermMapInfo], create a 
    /// constant-valued term map.
    fn create_constant_map(tm_info: TermMapInfo) -> T;

    /// Tries to create the underlying term map for the given 
    /// subject IRI in the given RDF graph.
    ///
    /// # Errors 
    ///
    /// Returns an error if the given subject IRI does not describe the underlying 
    /// term map type.
    ///
    fn create_term_map(
        subj_ref: &RcTerm,
        graph_ref: &FastGraph,
    ) -> ExtractorResult<T>;



    
    /// Creates the underlying [constant-vaued term map](https://rml.io/specs/rml/#constant-valued-term-map), 
    /// from the given RDF term.
    ///
    /// # Errors
    ///
    /// Returns an error if the given RDF term is a blank node. 
    fn extract_constant_term_map(map_const: &RcTerm) -> ExtractorResult<T> {
        if let RcTerm::BlankNode(_) = map_const {
            return Err(ParseError::GenericError(format!(
                "Constant-valued term map cannot be a BlankNode"
            ))
            .into());
        };

        let tm_info = TermMapInfo::from_constant_value(map_const.clone());

        Ok(Self::create_constant_map(tm_info))
    }

    /// Extracts term maps associated with the given parent/container subject IRI
    /// and return the first one.
    ///
    /// # Errors
    ///
    /// Returns an error if it cannot find any Term Maps.
    fn extract_from_container(
        graph_ref: &FastGraph,
        container_map_subj_ref: &RcTerm,
    ) -> ExtractorResult<T> {
        Self::extract_many_from_container(graph_ref, container_map_subj_ref)
            .and_then(|mut vec| vec.pop().ok_or(ParseError::Infallible.into()))
    }


    /// Extracts term maps associated with the given parent/container subject IRI 
    /// and return all the extracted items in a [Vec].
    ///
    /// # Errors
    ///
    /// Returns an error if no term maps are found.
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
                "0 TermMap of type {:?} found for {:?}",
                map_pred, container_map_subj_ref
            ))
            .into())
        } else {
            Ok(result)
        }
    }

    /// Returns the [constant shortcut properties](https://rml.io/specs/rml/#constant) of 
    /// the underlying term map.
    fn get_const_pred() -> RcTerm;

    /// Returns the full RDF property of the underlying term map (e.g., rml:subjectMap,
    /// rml:predicateMap, ...)
    fn get_map_pred() -> RcTerm;
}

/// A generic and simple trait for extracting an RML data
/// model [RML data model](crate::rml::parser::rml_model).
pub trait Extractor<T> {
    /// Extracts the underlying RML data mode for the given 
    /// subject IRI which reperesents the resource describing the data model.
    ///
    /// # Errors
    ///
    /// Returns error if somethin goes wrong while parsing/extracting.
    fn extract_self(
        subject_ref: &RcTerm,
        graph_ref: &FastGraph,
    ) -> ExtractorResult<T>;
}

/// Returns the lexical string representation of the given RDF term.
///
/// # Panics
///
/// Panics if the given term is of kind "Varialbe" or "Triple".
pub fn rcterm_to_string(rcterm: &RcTerm) -> String {
    let re_opt = match rcterm {
        RcTerm::Iri(iri_ref) => Some(iri_ref.to_string()),
        RcTerm::BlankNode(bnode_id) => Some(bnode_id.to_string()),
        RcTerm::Literal(generic_literal) => {
            Some(generic_literal.get_lexical_form().to_string())
        }
        _ => None,
    };

    re_opt.unwrap()
}

/// A trait to convert all the [vocab pairs](vocab) into 
/// types handled by [sophia terms'](sophia_term) [RcTerm] and [ArcTerm]. 
pub trait FromVocab {
    fn to_rcterm(&self) -> RcTerm;
    fn to_arcterm(&self) -> ArcTerm;
}

impl FromVocab for PAIR<'_> {
    fn to_rcterm(&self) -> RcTerm {
        RcTerm::Iri(IriRef::new_unchecked(self.to_string().into()))
    }
    fn to_arcterm(&self) -> ArcTerm {
        ArcTerm::Iri(IriRef::new_unchecked(self.to_string().into()))
    }
}
