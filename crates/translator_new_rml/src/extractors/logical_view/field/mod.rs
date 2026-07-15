pub mod error;
use std::collections::{HashMap, HashSet};

use error::FieldErrorEnum;
use sophia_api::term::Term;
use sophia_inmem::graph::FastGraph;

use crate::extractors::store::{get_object, get_objects};
use crate::extractors::{stringify_term, Extractor, FromVocab};
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

    // Try to extract an expression field
    let expression_result = ExpressionMapEnum::extract_self(subject_ref.borrow_term(), graph_ref);
    let kind = match expression_result {
        Ok(expression_map) => {
            log::debug!("Found RML field expression map:");
            RMLFieldKind::Expression(expression_map)
        },
        Err(expr_err) => {
            // Try to extract an iterable field
            let iter_result = RMLIterable::extract_self(subject_ref.borrow_term(), graph_ref);
            match iter_result {
                Ok(iterable) => {
                    log::debug!("Found RML iterable");
                    RMLFieldKind::Iterable(iterable)
                },
                Err(iter_err) => {
                    log::error!("RML Field extraction: failed to parse as an expression field and an iterable field.");
                    let err_msg = format!("Expression field parsing error: {}\nIterable field error: {}", expr_err.to_string(), iter_err.to_string());
                    return Err(FieldErrorEnum::ParseError(err_msg));
                }
            }
        },
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
