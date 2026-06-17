use crate::extractors::{stringify_term, FromVocab};
use crate::rml_model::v2::core::RMLIterable;
use crate::translator::error::TranslationError;
use operator::io::io_type::IOType;
use sophia_api::prelude::Graph;
use sophia_api::serializer::*;
use sophia_api::term::{BnodeId, FromTerm};
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;
use sophia_turtle::serializer::nt::NtSerializer;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ReferenceFormulation {
    pub iri:  RcTerm,
    pub kind: ReferenceFormulationKind,
}


#[derive(Clone)]
pub enum ReferenceFormulationKind {
    Iri,
    CustomReferenceFormulation { meta_data_graph: Rc<FastGraph> },
}

impl Eq for ReferenceFormulationKind {}

impl PartialEq for ReferenceFormulationKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::CustomReferenceFormulation {
                    meta_data_graph: l_meta_data_graph,
                },
                Self::CustomReferenceFormulation {
                    meta_data_graph: r_meta_data_graph,
                },
            ) => {
                sophia_isomorphism::isomorphic_graphs(
                    l_meta_data_graph.as_ref(),
                    r_meta_data_graph.as_ref(),
                )
                .unwrap()
            }
            _ => {
                core::mem::discriminant(self) == core::mem::discriminant(other)
            }
        }
    }
}

impl Hash for ReferenceFormulationKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Iri => {
                state.write_u8(0);
            }
            Self::CustomReferenceFormulation { meta_data_graph } => {
                state.write_u8(1);
                let mut serializer = NtSerializer::new_stringifier();
                let serialized_graph = serializer.serialize_graph(meta_data_graph.as_ref()).unwrap();
                state.write(serialized_graph.as_str().as_bytes());
            }
        }
    }
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
pub struct LogicalSource {
    pub iterable:   RMLIterable,
    pub identifier: RcTerm,
    pub source:     Source,
}

impl LogicalSource {
    /// Calculates the "effective equality" hash of a LogicalSource.
    /// This takes the iterable and the source into account, but not the identifier,
    /// because two LogicalSources can be effectively equal even if they have different identifiers.
    pub fn effective_equality_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.iterable.hash(&mut hasher);
        hasher.write_u64(self.source.effective_equality_hash());
        hasher.finish()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Source {
    pub encoding:     Option<RcTerm>,
    pub compression:  Option<RcTerm>,
    pub nullable_vec: Vec<String>,
    pub kind:         SourceKind,
}

impl Source {
    /// Calculates the "effective equality" hash of a Source.
    /// This only takes the kind into account, as the other fields are not relevant for determining
    /// if two Sources are effectively equal.
    pub fn effective_equality_hash(&self) -> u64 {
        self.kind.effective_equality_hash()
    }
}

#[derive(Clone)]
pub struct SourceKind {
    pub subj_iri: RcTerm,
    pub type_iri: RcTerm,
    pub metadata: Rc<FastGraph>,
}

impl SourceKind {

    /// Calculates the "effective equality" hash of a SourceKind
    /// It takes the type_iri and the metadata into account.
    /// For the metadata only the predicate and objects count, because the subject can be different (e.g., if it's a blank node)
    pub fn effective_equality_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.type_iri.hash(&mut hasher);
        self.metadata.as_ref().predicates().for_each(|predicate| {
            predicate.unwrap().hash(&mut hasher);
        });
        self.metadata.as_ref().objects().for_each(|object| {
            object.unwrap().hash(&mut hasher);
        });
        hasher.finish()
    }
}

impl Eq for SourceKind {}
impl PartialEq for SourceKind {
    fn eq(&self, other: &Self) -> bool {
        self.subj_iri == other.subj_iri
            && self.type_iri == other.type_iri
            && sophia_isomorphism::isomorphic_graphs(
                self.metadata.as_ref(),
                other.metadata.as_ref(),
            )
            .unwrap()
    }
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
        if value.type_iri == vocab::rml_io::class::FILE_PATH.to_rcterm()
            || value.type_iri == vocab::rml_io::class::RELATIVE_PATH.to_rcterm()
            || value.type_iri
                == vocab::rml_io::class::RELATIVE_PATH_SOURCE.to_rcterm()
            || value.type_iri == vocab::rml_io::class::MAPPING_DIR.to_rcterm()
        {
            Ok(IOType::File)
        } else if value.type_iri == vocab::d2rq::class::DATABASE.to_rcterm()
            || value.type_iri == vocab::rml_io::class::SQL_TABLE.to_rcterm()
        {
            Ok(IOType::RDB)
        } else if value.type_iri == vocab::td::class::THING.to_rcterm() {
            Ok(IOType::Websocket)
        } else if value.type_iri
            == vocab::rmls::class::TCPSOCKETSTREAM.to_rcterm()
        {
            Ok(IOType::Websocket)
        } else if value.type_iri == vocab::rmls::class::KAFKASTREAM.to_rcterm()
        {
            Ok(IOType::Kafka)
        } else {
            Err(TranslationError::SourceError(format!(
                "Input format {} not not supported to convert to IOType",
                stringify_term(value.type_iri.clone()).unwrap()
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
            type_iri: vocab::rml_io::class::FILE_PATH.to_rcterm(),
            metadata: Rc::new(FastGraph::new()),
        }
    }
}
