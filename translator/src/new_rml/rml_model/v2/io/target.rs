use std::fmt::Debug;
use std::rc::Rc;

use sophia_api::serializer::*;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;
use sophia_turtle::serializer::nt::NtSerializer;

use crate::new_rml::extractors::FromVocab;

#[derive(Debug, Clone, Default)]
pub struct LogicalTarget {
    pub target:     Target,
    pub ser_format: Option<RcTerm>,
}

#[derive(Debug, Clone, Default)]
pub struct Target {
    pub encoding:    Option<RcTerm>,
    pub compression: Option<RcTerm>,
    pub mode:        Option<RcTerm>,
    pub kind:        TargetKind,
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
            type_iri: vocab::rml_io::CLASS::STD_OUT.to_rcterm(),
            metadata: Rc::new(FastGraph::new()),
        }
    }
}
