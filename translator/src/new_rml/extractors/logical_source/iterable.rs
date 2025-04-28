use sophia_api::term::Term;

use crate::new_rml::extractors::store::get_object_with_ps;
use crate::new_rml::extractors::{Extractor, FromVocab};
use crate::new_rml::rml_model::v2::core::RMLIterable;
use crate::new_rml::rml_model::v2::io::source::ReferenceFormulation;

impl Extractor<RMLIterable> for RMLIterable {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> crate::new_rml::extractors::ExtractorResult<RMLIterable>
    where
        TTerm: Term + Clone,
    {
        let iter_old_pred = &vocab::rml::PROPERTY::ITERATOR.to_rcterm();
        let iter_new_pred = &vocab::rml_core::PROPERTY::ITERATOR.to_rcterm();
        let iterator = get_object_with_ps(
            graph_ref,
            subject_ref.borrow_term(),
            &[iter_old_pred, iter_new_pred],
        )
        .ok()
        .and_then(|lit| lit.lexical_form().map(|l| l.to_string()));

        let refform_old_pred =
            &vocab::rml::PROPERTY::REFERENCEFORMULATION.to_rcterm();
        let refform_new_pred =
            &vocab::rml_core::PROPERTY::REFERENCE_FORMULATION.to_rcterm();
        let reference_formulation_subj_term_opt = get_object_with_ps(
            graph_ref,
            subject_ref.borrow_term(),
            &[refform_old_pred, refform_new_pred],
        )
        .ok();

        let reference_formulation = reference_formulation_subj_term_opt
            .and_then(|ref_form| {
                ReferenceFormulation::extract_self(&ref_form, graph_ref).ok()
            });

        Ok(RMLIterable {
            iterator,
            reference_formulation,
        })
    }
}
