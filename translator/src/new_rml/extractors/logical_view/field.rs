use sophia_api::term::Term;
use sophia_inmem::graph::FastGraph;

use crate::new_rml::extractors::store::{get_object, get_objects};
use crate::new_rml::extractors::{
    stringify_rcterm, Extractor, ExtractorResult, FromVocab,
};
use crate::new_rml::rml_model::v2::core::expression_map::{
    ExpressionMap, ExpressionMapKind,
};
use crate::new_rml::rml_model::v2::core::RMLIterable;
use crate::new_rml::rml_model::v2::lv::{RMLField, RMLFieldKind};

impl Extractor<RMLField> for RMLField {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &FastGraph,
    ) -> ExtractorResult<RMLField>
    where
        TTerm: Term + Clone,
    {
        let name = stringify_rcterm(get_object(
            graph_ref,
            subject_ref.borrow_term(),
            &vocab::rml_lv::PROPERTY::FIELD_NAME.to_rcterm(),
        )?)
        .unwrap();

        let reference_opt = get_object(
            graph_ref,
            subject_ref.borrow_term(),
            &vocab::rml_core::PROPERTY::REFERENCE.to_rcterm(),
        )
        .ok();

        let kind = if let Some(reference) = reference_opt {
            RMLFieldKind::Expression(ExpressionMap {
                map_type_pred_iri: vocab::rml_core::PROPERTY::REFERENCE
                    .to_rcterm(),
                kind:              ExpressionMapKind::NonFunction(
                    stringify_rcterm(reference).unwrap(),
                ),
            })
        } else {
            let iterable = RMLIterable::extract_self(
                subject_ref.borrow_term(),
                graph_ref,
            )?;
            RMLFieldKind::Iterable(iterable)
        };

        let fields = get_objects(
            graph_ref,
            subject_ref,
            &vocab::rml_lv::PROPERTY::FIELD.to_rcterm(),
        )
        .iter()
        .filter_map(|term| Self::extract_self(term, graph_ref).ok())
        .collect();

        Ok(RMLField { name, kind, fields })
    }
}
