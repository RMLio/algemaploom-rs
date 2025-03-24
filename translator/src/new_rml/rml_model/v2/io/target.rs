use std::fmt::Debug;
use std::rc::Rc;

use sophia_api::prelude::Iri;
use sophia_api::serializer::*;
use sophia_api::term::{FromTerm, Term};
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;
use sophia_turtle::serializer::nt::NtSerializer;

#[derive(Debug, Clone)]
pub struct LogicalTarget {
    pub target:     Target,
    pub ser_format: Option<RcTerm>,
}

impl Default for LogicalTarget {
    fn default() -> Self {
        Self {
            target:     Default::default(),
            ser_format: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Target {
    pub encoding:    Option<RcTerm>,
    pub compression: Option<RcTerm>,
    pub mode:        Option<RcTerm>,
    pub kind:        TargetKind,
}

impl Default for Target {
    fn default() -> Self {
        Self {
            encoding:    Default::default(),
            compression: Default::default(),
            mode:        Default::default(),
            kind:        Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct TargetKind {
    pub type_iri: RcTerm,
    pub metadata: Rc<FastGraph>,
}

impl Debug for TargetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut serde = NtSerializer::new_stringifier();
        f.debug_struct("CustomTarget")
            .field("type_iri", &self.type_iri)
            .field(
                "meta_data_graph",
                &serde
                    .serialize_graph(self.metadata.as_ref())
                    .unwrap()
                    .as_str(),
            )
            .finish()
    }
}

impl Default for TargetKind {
    fn default() -> Self {
        Self {
            type_iri: RcTerm::from_term(Iri::new_unchecked("default")),
            metadata: Rc::new(FastGraph::new()),
        }
    }
}
