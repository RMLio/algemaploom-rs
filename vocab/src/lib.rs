//! 
//! Vocab module contains API for the IRI values of common ontologies used in 
//! the mapping languages.
//! 
//! Classes of the ontologies are behind the `CLASS` sub-module of each ontology modules.
//! 
//! The following example shows the utilization of the `TriplesMap` class 
//! from the latest RML ontology [http://w3id.org/rml/TriplesMap](http://w3id.org/rml/TriplesMap)
//! # Example 
//! ```
//! use vocab::rml_core;
//! use vocab::ToString; 
//! 
//! let rml_triples_map = rml_core::CLASS::TRIPLES_MAP;
//! 
//! assert_eq!("http://w3id.org/rml/TriplesMap".to_string(), rml_triples_map.to_string()); 
//! 
//! ```
//! 
//! Idem for the properties of the ontologies which are behind the `PROPERTY` sub-module of each 
//! ontology modules.
//! 
use std::fmt::Display;

pub mod comp;
pub mod csvw;
pub mod d2rq;
pub mod fnml;
pub mod fno;
pub mod formats;
pub mod ldes;
pub mod query;
pub mod r2rml;
pub mod rdf;
pub mod rml;
pub mod rml_core;
pub mod rml_io;
pub mod rmls;
pub mod rmlt;
pub mod tree;
pub mod void;
pub mod xsd;
pub mod rml_lv;
pub mod rml_fnml;
pub mod rml_cc; 

pub type PAIR<'a> = (&'a str, &'a str);

pub trait ToString {
    fn to_string(self) -> String;
}

impl<'a> ToString for PAIR<'a> {
    fn to_string(self) -> String {
        format!("{}{}", self.0, self.1)
    }
}
