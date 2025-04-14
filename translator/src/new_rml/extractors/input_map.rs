use sophia_api::term::Term;

use super::error::ParseError;
use super::store::{get_object, get_object_with_ps};
use super::term_map_extractor::term_map_from_constant_term;
use super::{Extractor, FromVocab};
use crate::new_rml::rml_model::v2::core::expression_map::term_map::TermMap;
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
            &[&vocab::rml_fnml::PROPERTY::PARAMETER.to_rcterm()],
        )
        .into_iter()
        .filter_map(|iri| term_map_from_constant_term(iri).ok())
        .filter_map(|tm| tm.try_get_node());

        let parameter_maps = get_object_with_ps(
            graph_ref,
            subject_ref.borrow_term(),
            &[&vocab::rml_fnml::PROPERTY::PARAMETER_MAP.to_rcterm()],
        )
        .into_iter()
        .filter_map(|iri| TermMap::extract_self(iri, graph_ref).ok())
        .filter_map(|tm| tm.try_get_node());

        let parameter = parameter.chain(parameter_maps).next().ok_or(
            ParseError::GenericError(format!(
                "No parameters detected for FNML input map {:?}",
                subject_ref.borrow_term()
            )),
        )?;

        let value = get_object_with_ps(
            graph_ref,
            subject_ref.borrow_term(),
            &[&vocab::rml_fnml::PROPERTY::INPUT_VALUE.to_rcterm()],
        )
        .into_iter()
        .filter_map(|iri| term_map_from_constant_term(iri).ok());

        let value_map = get_object_with_ps(
            graph_ref,
            subject_ref.borrow_term(),
            &[&vocab::rml_fnml::PROPERTY::INPUT_VALUE_MAP.to_rcterm()],
        )
        .into_iter()
        .filter_map(|iri| TermMap::extract_self(iri, graph_ref).ok());

        let value_map =
            value
                .chain(value_map)
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
