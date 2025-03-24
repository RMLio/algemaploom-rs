use sophia_api::term::Term;
use sophia_inmem::graph::FastGraph;

use crate::new_rml::extractors::store::{get_object, get_objects};
use crate::new_rml::extractors::{Extractor, ExtractorResult, FromVocab};
use crate::new_rml::rml_model::v2::lv::StructuralAnnotation;

impl Extractor<StructuralAnnotation> for StructuralAnnotation {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &FastGraph,
    ) -> ExtractorResult<StructuralAnnotation>
    where
        TTerm: Term,
    {
        let on_fields = get_objects(
            graph_ref,
            subject_ref.borrow_term(),
            vocab::rml_lv::PROPERTY::ON_FIELDS.to_rcterm(),
        );

        let target_fields = get_objects(
            graph_ref,
            subject_ref.borrow_term(),
            vocab::rml_lv::PROPERTY::TARGET_FIELDS.to_rcterm(),
        );
        let target_views = get_objects(
            graph_ref,
            subject_ref.borrow_term(),
            vocab::rml_lv::PROPERTY::TARGET_VIEW.to_rcterm(),
        );

        let kind = get_object(
            graph_ref,
            subject_ref,
            vocab::rdf::PROPERTY::TYPE.to_rcterm(),
        )?;

        Ok(StructuralAnnotation {
            kind,
            on_fields,
            target_fields,
            target_views,
        })
    }
}
