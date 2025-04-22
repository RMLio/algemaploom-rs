use std::rc::Rc;

use sophia_api::graph::MutableGraph;
use sophia_api::term::{BnodeId, FromTerm, Term};
use sophia_inmem::graph::FastGraph;

use super::store::get_object_with_ps;
use super::{Extractor, RcTerm};
use crate::new_rml::extractors::FromVocab;
use crate::new_rml::rml_model::v2::io::source::{LogicalSource, Source, SourceKind};

mod iterable;
pub mod ref_form;

impl Extractor<LogicalSource> for LogicalSource {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &FastGraph,
    ) -> super::ExtractorResult<LogicalSource>
    where
        TTerm: Term + Clone,
    {
        let source_new_pred = &vocab::rml_io::PROPERTY::SOURCE.to_rcterm();
        let source_old_pred = &vocab::rml::PROPERTY::SOURCE.to_rcterm();
        let source_obj_term = get_object_with_ps(
            graph_ref,
            subject_ref.borrow_term(),
            &[source_new_pred, source_old_pred],
        )?;

        let source = match source_obj_term {
            RcTerm::Literal(literal) => {
                let mut metadata = FastGraph::new();
                let subj_bnode = BnodeId::<&str>::new("default").unwrap();
                metadata.insert(
                    subj_bnode,
                    vocab::rml_io::PROPERTY::ROOT.to_rcterm(),
                    vocab::rml_io::CLASS::MAPPING_DIR.to_rcterm(),
                ).unwrap();

                metadata.insert(
                    subj_bnode,
                    vocab::rml_io::PROPERTY::PATH.to_rcterm(),
                    literal,
                ).unwrap();

                Source {
                    kind:         SourceKind {
                        subj_iri: RcTerm::from_term(subj_bnode), 
                        type_iri: vocab::rml_io::CLASS::MAPPING_DIR.to_rcterm(),
                        metadata: Rc::new(metadata),
                    },
                    encoding:     None,
                    compression:  None,
                    nullable_vec: Vec::new(),
                }
            }
            term => Source::extract_self(&term, graph_ref)?,
        };

        Ok(LogicalSource {
            identifier: RcTerm::from_term(subject_ref),
            source,
        })
    }
}
