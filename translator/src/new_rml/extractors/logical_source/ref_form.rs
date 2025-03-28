use sophia_api::graph::Graph;
use sophia_api::term::{FromTerm, Term, TermKind};
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use crate::new_rml::error::NewRMLTranslationError;
use crate::new_rml::extractors::error::ParseError;
use crate::new_rml::extractors::store::get_subgraph_subject;
use crate::new_rml::extractors::{Extractor, ExtractorResult};
use crate::new_rml::rml_model::v2::io::source::{
    ReferenceFormulation, ReferenceFormulationKind,
};

impl Extractor<ReferenceFormulation> for ReferenceFormulation {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> Result<ReferenceFormulation, NewRMLTranslationError>
    where
        TTerm: Term,
    {
        match subject_ref.kind() {
            TermKind::Iri | TermKind::BlankNode => {
                derive_reference_formulation(subject_ref, graph_ref)
            }
            _ => {
                Err(ParseError::GenericError(
                    "RML reference formulation cannot be a Literal".to_string(),
                ).into())
            }
        }
    }
}

fn derive_reference_formulation<TTerm>(
    subject_ref: TTerm,
    graph: &FastGraph,
) -> ExtractorResult<ReferenceFormulation>
where
    TTerm: Term,
{
    let subgraph: FastGraph =
        get_subgraph_subject(graph, subject_ref.borrow_term())?;
    let metadata_triple_opt = subgraph.triples().next();

    if metadata_triple_opt.is_some() {
        Ok(ReferenceFormulation {
            iri:  RcTerm::from_term(subject_ref),
            kind: ReferenceFormulationKind::CustomReferenceFormulation {
                meta_data_graph: subgraph.into(),
            },
        })
    } else {
        Ok(ReferenceFormulation {
            iri:  RcTerm::from_term(subject_ref),
            kind: ReferenceFormulationKind::Iri,
        })
    }
}
