use sophia_api::term::Term;

use super::error::ParseError;
use super::store::{get_object, get_object_with_ps};
use super::term_map_extractor::term_map_from_constant_term;
use super::{Extractor, FromVocab};
use crate::new_rml::rml_model::v2::fnml::InputMap;

impl Extractor<InputMap> for InputMap {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<InputMap>
    where
        TTerm: Term,
    {
        let parameter = get_object_with_ps(
            graph_ref,
            subject_ref.borrow_term(),
            &[
                &vocab::rml_fnml::PROPERTY::PARAMETER_MAP.to_rcterm(),
                &vocab::rml_fnml::PROPERTY::PARAMETER.to_rcterm(),
            ],
        )
        .iter()
        .filter_map(|iri| term_map_from_constant_term(iri).ok())
        .filter_map(|tm| tm.try_get_node())
        .next()
        .ok_or(ParseError::GenericError(format!(
            "No parameters detected for FNML input map {:?}",
            subject_ref.borrow_term()
        )))?;

        let value_map = get_object_with_ps(
            graph_ref,
            subject_ref.borrow_term(),
            &[
                &vocab::rml_fnml::PROPERTY::INPUT_VALUE_MAP.to_rcterm(),
                &vocab::rml_fnml::PROPERTY::INPUT_VALUE.to_rcterm(),
            ],
        )
        .iter()
        .filter_map(|iri| term_map_from_constant_term(iri).ok())
        .next()
        .ok_or(ParseError::GenericError(format!(
            "No parameters detected for FNML input map {:?}",
            subject_ref
        )))?;

        Ok(InputMap {
            parameter,
            value_map,
        })
    }
}
