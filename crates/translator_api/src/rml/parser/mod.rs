//!
//! Contains the functionalities to parse an input RML document into 
//! the [RML data model](crate::rml::parser::rml_model)


use sophia_api::prelude::Iri;

pub mod extractors;
pub mod rml_model;



/// Type alias for [sophia's IRI](Iri) 
pub type IriString = Iri<String>; 
