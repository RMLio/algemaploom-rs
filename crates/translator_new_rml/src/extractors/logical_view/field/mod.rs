pub mod error;
use std::collections::{HashMap, HashSet};

use error::FieldErrorEnum;
use sophia_api::term::Term;
use sophia_inmem::graph::FastGraph;

use crate::extractors::error::ParseError;
use crate::extractors::logical_view::error::LogicalViewErrorEnum;
use crate::extractors::store::{get_object, get_objects};
use crate::extractors::{
    stringify_term, Extractor, ExtractorResult, FromVocab,
};
use crate::rml_model::v2::core::expression_map::ExpressionMapEnum;
use crate::rml_model::v2::core::RMLIterable;
use crate::rml_model::v2::lv::{RMLField, RMLFieldKind};

pub fn extract_field<TTerm>(
    subject_ref: TTerm,
    graph_ref: &FastGraph,
    parent_path_opt: Option<&str>,
) -> Result<RMLField, FieldErrorEnum>
where
    TTerm: Term + Clone,
{
    let name = stringify_term(get_object(
        graph_ref,
        subject_ref.borrow_term(),
        vocab::rml_lv::property::FIELD_NAME.to_rcterm(),
    )?)
    .unwrap();
    log::debug!("RML field name: {}", name);

    let reference_opt = get_object(
        graph_ref,
        subject_ref.borrow_term(),
        vocab::rml_core::property::REFERENCE.to_rcterm(),
    )
    .ok();

    let constant_opt = get_object(
        graph_ref,
        subject_ref.borrow_term(),
        vocab::rml_core::property::CONSTANT.to_rcterm(),
    )
    .ok();

    let kind = if let Some(reference) = reference_opt {
        log::debug!("Reference RML field");
        RMLFieldKind::Expression(ExpressionMapEnum::new_reference_term(
            reference,
        ))
    } else if let Some(constant) = constant_opt {
        log::debug!("Constant RML field");
        RMLFieldKind::Expression(ExpressionMapEnum::new_constant_term(constant))
    } else {
        log::debug!("Extracting RMLIterable");
        let iterable =
            RMLIterable::extract_self(subject_ref.borrow_term(), graph_ref)
                .map_err(|err| {
                    FieldErrorEnum::IterableError(err.to_string())
                })?;
        RMLFieldKind::Iterable(iterable)
    };

    log::debug!("RML Field kind is: {:#?}", kind);
    let mut absolute_name = name.clone();
    if let Some(parent_path) = parent_path_opt {
        absolute_name = format!("{}.{}", parent_path, name);
    }
    let fields = get_objects(
        graph_ref,
        subject_ref,
        vocab::rml_lv::property::FIELD.to_rcterm(),
    )
    .iter()
    .map(|term| extract_field(term, graph_ref, Some(&absolute_name)))
    .collect::<Result<Vec<_>, FieldErrorEnum>>()?;

    name_conflict_check(&fields)?;

    Ok(RMLField {
        name,
        kind,
        fields,
        absolute_name,
    })
}

pub fn name_conflict_check(fields: &[RMLField]) -> Result<(), FieldErrorEnum> {
    let name_iter = fields.iter().map(|f| &f.name);
    let unique_names_count = name_iter.clone().collect::<HashSet<_>>().len();

    if fields.len() != unique_names_count {
        let conflicting_field_names = name_iter
            .fold(HashMap::new(), |mut map, f| {
                map.entry(f).and_modify(|c| *c += 1).or_insert(1);
                map
            })
            .iter()
            .filter(|pair| *pair.1 > 1)
            .map(|pair| pair.0.to_string())
            .collect::<Vec<_>>();

        return Err(FieldErrorEnum::ConflictingNamesError(
            conflicting_field_names,
        ));
    }

    for field in fields {
        name_conflict_check(&field.fields)?;
    }

    Ok(())
}
