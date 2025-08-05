use sophia_api::term::Term;
use sophia_inmem::graph::FastGraph;

use crate::new_rml::extractors::store::{get_object, get_objects};
use crate::new_rml::extractors::{
    stringify_rcterm, Extractor, ExtractorResult, FromVocab,
};
use crate::new_rml::rml_model::v2::core::expression_map::ExpressionMapEnum;
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
            vocab::rml_lv::PROPERTY::FIELD_NAME.to_rcterm(),
        )?)
        .unwrap();
        log::debug!("RML field name: {}", name);

        let reference_opt = get_object(
            graph_ref,
            subject_ref.borrow_term(),
            vocab::rml_core::PROPERTY::REFERENCE.to_rcterm(),
        )
        .ok();

        let constant_opt = get_object(
            graph_ref,
            subject_ref.borrow_term(),
            vocab::rml_core::PROPERTY::CONSTANT.to_rcterm(),
        )
        .ok();

        let kind = if let Some(reference) = reference_opt {
            log::debug!("Reference RML field");
            RMLFieldKind::Expression(ExpressionMapEnum::new_reference_term(
                reference,
            ))
        } else if let Some(constant) = constant_opt {
            log::debug!("Constant RML field");
            RMLFieldKind::Expression(ExpressionMapEnum::new_constant_term(
                constant,
            ))
        } else {
            log::debug!("Extracting RMLIterable");
            let iterable = RMLIterable::extract_self(
                subject_ref.borrow_term(),
                graph_ref,
            )?;
            RMLFieldKind::Iterable(iterable)
        };

        log::debug!("RML Field kind is: {:#?}", kind);
        let fields = get_objects(
            graph_ref,
            subject_ref,
            vocab::rml_lv::PROPERTY::FIELD.to_rcterm(),
        )
        .iter()
        .filter_map(|term| Self::extract_self(term, graph_ref).ok())
        .collect();

        Ok(RMLField { name, kind, fields })
    }
}
