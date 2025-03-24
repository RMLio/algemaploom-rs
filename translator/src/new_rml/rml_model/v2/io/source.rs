use std::fmt::Debug;
use std::rc::Rc;

use sophia_api::prelude::Iri;
use sophia_api::serializer::*;
use sophia_api::term::FromTerm;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;
use sophia_turtle::serializer::nt::NtSerializer;

use crate::new_rml::extractors::error::ParseError;
use crate::new_rml::extractors::FromVocab;

#[derive(Debug, Clone)]
pub struct ReferenceFormulation {
    pub iri:  RcTerm,
    pub kind: ReferenceFormulationKind,
}

#[derive(Clone)]
pub enum ReferenceFormulationKind {
    Iri,
    CustomReferenceFormulation { meta_data_graph: Rc<FastGraph> },
}

impl Debug for ReferenceFormulationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut serde = NtSerializer::new_stringifier();
        match self {
            Self::Iri => f.debug_tuple("Iri").finish(),
            Self::CustomReferenceFormulation {
                meta_data_graph: graph,
            } => {
                f.debug_struct("CustomReferenceFormulation")
                    .field(
                        "meta_data_graph",
                        &serde
                            .serialize_graph(graph.as_ref())
                            .unwrap()
                            .as_str(),
                    )
                    .finish()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum RMLReferenceFormulationTypeKind {
    JSONPath,
    CSVRows,
    XPath,
    Parent,
    XPathNamespace { prefix: String, uri: String },
}

impl TryFrom<ReferenceFormulation> for RMLReferenceFormulationTypeKind {
    type Error = ParseError;

    fn try_from(value: ReferenceFormulation) -> Result<Self, Self::Error> {
        value.iri.try_into()
    }
}

impl TryFrom<RcTerm> for RMLReferenceFormulationTypeKind {
    type Error = ParseError;

    fn try_from(value: RcTerm) -> Result<Self, Self::Error> {
        match value {
            value if value == vocab::query::CLASS::CSV.to_rcterm() => {
                Ok(RMLReferenceFormulationTypeKind::CSVRows)
            }
            value if value == vocab::query::CLASS::JSONPATH.to_rcterm() => {
                Ok(RMLReferenceFormulationTypeKind::JSONPath)
            }
            value if value == vocab::query::CLASS::XPATH.to_rcterm() => {
                Ok(RMLReferenceFormulationTypeKind::XPath)
            }
            _ => {
                Err(ParseError::GenericError(format!(
                    "reference formulation type is not supported: {:?}",
                    value
                )))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogicalSource {
    pub identifier: RcTerm,
    pub source:     Source,
}

#[derive(Debug, Clone)]
pub struct Source {
    pub encoding:     Option<RcTerm>,
    pub compression:  Option<RcTerm>,
    pub nullable_vec: Vec<String>,
    pub kind:         SourceKind,
}

#[derive(Clone)]
pub struct SourceKind {
    pub type_iri: RcTerm,
    pub metadata: Rc<FastGraph>,
}

impl Debug for SourceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut serde = NtSerializer::new_stringifier();
        f.debug_struct("SourceKind")
            .field("type_iri", &self.type_iri)
            .field(
                "metadata",
                &serde
                    .serialize_graph(self.metadata.as_ref())
                    .unwrap()
                    .as_str(),
            )
            .finish()
    }
}

impl Default for SourceKind {
    fn default() -> Self {
        Self {
            type_iri: RcTerm::from_term(Iri::new_unchecked("default")),
            metadata: Rc::new(FastGraph::new()),
        }
    }
}
