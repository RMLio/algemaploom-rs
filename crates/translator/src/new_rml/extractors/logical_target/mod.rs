use sophia_api::term::Term;

use super::Extractor;
use crate::new_rml::extractors::store::{
    get_object, get_object_with_ps,
};
use crate::new_rml::extractors::FromVocab;
use crate::new_rml::rml_model::v2::io::target::{LogicalTarget, Target};

impl Extractor<LogicalTarget> for LogicalTarget {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<LogicalTarget>
    where
        TTerm: Term,
    {
        let target_old_pred = &vocab::rmlt::PROPERTY::TARGET.to_rcterm();
        let target_new_pred = &vocab::rml_io::PROPERTY::TARGET.to_rcterm();
        let target_subj_term = get_object_with_ps(
            graph_ref,
            subject_ref.borrow_term(),
            &[target_old_pred, target_new_pred],
        )?;

        let target = Target::extract_self(&target_subj_term, graph_ref)?;

        let ser_format = get_object(
            graph_ref,
            subject_ref,
            &vocab::rml_io::PROPERTY::SERIALIZATION.to_rcterm(),
        )
        .ok();

        Ok(LogicalTarget { target, ser_format })
    }
}
