use std::fmt::Debug;
use std::rc::Rc;

use operator::{formats, IOType};
use sophia_api::serializer::*;
use sophia_api::term::{BnodeId, FromTerm};
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;
use sophia_turtle::serializer::nt::NtSerializer;

use crate::new_rml::error::NewRMLTranslationError;
use crate::new_rml::extractors::error::ParseError;
use crate::new_rml::extractors::{stringify_rcterm, FromVocab};
use crate::new_rml::rml_model::v2::core::RMLIterable;
use crate::new_rml::translator::error::TranslationError;

#[derive(Debug, Clone)]
pub struct ReferenceFormulation {
    pub iri:  RcTerm,
    pub kind: ReferenceFormulationKind,
}

impl TryFrom<ReferenceFormulation> for operator::formats::ReferenceFormulation {
    type Error = TranslationError;

    fn try_from(value: ReferenceFormulation) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}

impl TryFrom<&ReferenceFormulation>
    for operator::formats::ReferenceFormulation
{
    type Error = TranslationError;

    fn try_from(value: &ReferenceFormulation) -> Result<Self, Self::Error> {
        match value.kind {
            ReferenceFormulationKind::Iri => {
                match value.iri.clone() {
                    value
                        if value
                            == vocab::d2rq::CLASS::DATABASE.to_rcterm()
                            || value
                                == vocab::rml_io::CLASS::SQL_QUERY
                                    .to_rcterm()
                            || value
                                == vocab::rml_io::CLASS::SQL_TABLE
                                    .to_rcterm() =>
                    {
                        Ok(formats::ReferenceFormulation::SQLQuery)
                    }
                    value
                        if value == vocab::query::CLASS::CSV.to_rcterm()
                            || value
                                == vocab::rml_io::CLASS::CSV.to_rcterm() =>
                    {
                        Ok(formats::ReferenceFormulation::CSVRows)
                    }
                    value
                        if value
                            == vocab::query::CLASS::JSONPATH.to_rcterm()
                            || value
                                == vocab::rml_io::CLASS::JSONPATH
                                    .to_rcterm() =>
                    {
                        Ok(formats::ReferenceFormulation::JSONPath)
                    }
                    value
                        if value == vocab::query::CLASS::XPATH.to_rcterm()
                            || value
                                == vocab::rml_io::CLASS::XPATH.to_rcterm() =>
                    {
                        Ok(formats::ReferenceFormulation::XMLPath)
                    }
                    value => {
                        Err(TranslationError::SourceError(format!(
                            "Unsupported reference formulation: {}",
                            stringify_rcterm(value).unwrap()
                        )))
                    }
                }
            }
            ReferenceFormulationKind::CustomReferenceFormulation {
                meta_data_graph: _,
            } => {
                Err(TranslationError::SourceError(format!(
                    "Complex reference formulation unsupported: {:?}",
                    value
                )))
            }
        }
    }
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
    type Error = NewRMLTranslationError;

    fn try_from(value: ReferenceFormulation) -> Result<Self, Self::Error> {
        value.iri.try_into()
    }
}

impl TryFrom<RcTerm> for RMLReferenceFormulationTypeKind {
    type Error = NewRMLTranslationError;

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
                ))
                .into())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogicalSource {
    pub iterable:   RMLIterable,
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
    pub subj_iri: RcTerm,
    pub type_iri: RcTerm,
    pub metadata: Rc<FastGraph>,
}
impl TryFrom<SourceKind> for IOType {
    type Error = TranslationError;

    fn try_from(value: SourceKind) -> Result<Self, Self::Error> {
        value.try_into()
    }
}

impl TryFrom<&SourceKind> for IOType {
    type Error = TranslationError;
    fn try_from(value: &SourceKind) -> Result<Self, Self::Error> {
        if value.type_iri == vocab::rml_io::CLASS::FILE_PATH.to_rcterm()
            || value.type_iri == vocab::rml_io::CLASS::RELATIVE_PATH.to_rcterm()
            || value.type_iri
                == vocab::rml_io::CLASS::RELATIVE_PATH_SOURCE.to_rcterm()
        {
            Ok(IOType::File)
        } else if value.type_iri == vocab::d2rq::CLASS::DATABASE.to_rcterm()
            || value.type_iri == vocab::rml_io::CLASS::SQL_TABLE.to_rcterm()
        {
            Ok(IOType::RDB)
        } else {
            Err(TranslationError::SourceError(format!(
                "Input format {} not not supported to convert to IOType",
                stringify_rcterm(value.type_iri.clone()).unwrap()
            )))
        }
    }
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
            subj_iri: RcTerm::from_term(BnodeId::new_unchecked_const(
                "default_bnode",
            )),
            type_iri: vocab::rml_io::CLASS::FILE_PATH.to_rcterm(),
            metadata: Rc::new(FastGraph::new()),
        }
    }
}
