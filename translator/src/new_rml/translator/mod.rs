pub mod error;
mod extend;
mod join;
mod serializer;
mod source;
mod store;

use std::cell::RefMut;
use std::path::Path;

use extend::ExtendOperatorTranslator;
use join::JoinTranslator;
use log::debug;
use operator::Target;
use plangenerator::states::Processed;
use plangenerator::Plan;
use serializer::SerializerOperatorTranslator;
use store::SearchStore;

use super::error::NewRMLTranslationResult;
use super::rml_model::v2::core::TriplesMap;
use super::rml_model::Document;
use crate::new_rml::error::NewRMLTranslationError;
use crate::new_rml::extractors::io::parse_file;
use crate::LanguageTranslator;

pub trait OperatorTranslator {
    type Input;
    type Output;

    fn translate(
        _input: &Self::Input,
    ) -> NewRMLTranslationResult<Self::Output> {
        unimplemented!("Operator translator does not support translation without the usage of a search store")
    }
    fn translate_with_store(
        _store: &SearchStore,
        _input: &Self::Input,
    ) -> NewRMLTranslationResult<Self::Output> {
        unimplemented!("Operator translator does not support translation with the usage of a search store")
    }
}

#[derive(Debug, Clone)]
pub struct NewRMLDocumentTranslator {}

impl LanguageTranslator<&Path> for NewRMLDocumentTranslator {
    fn translate_to_plan(path: &Path) -> crate::LanguageTranslateResult {
        let document = parse_file(path.to_path_buf())?;
        NewRMLDocumentTranslator::translate_to_plan(document)
    }
}

impl LanguageTranslator<Document> for NewRMLDocumentTranslator {
    fn translate_to_plan(
        mut model: Document,
    ) -> crate::LanguageTranslateResult {
        //preprocessing to change all logical sources to logical views
        for tm in model.triples_maps.iter_mut() {
            tm.transform_to_logical_view()?;
        }

        let search_store = SearchStore::from_document(&model)?;

       // for tm in model.triples_maps.iter() {
       //     JoinTranslator::translate_with_store(&search_store, tm)?;
       // }

        for (abs_ls_id, tm_vec) in search_store.partition_lsid_tmid() {
            let tm_vec: Vec<_> = tm_vec
                .iter()
                .flat_map(|tm_id| {
                    search_store.tm_search_map.get(tm_id).copied()
                })
                .collect();

            let mut plan = search_store
                .ls_id_sourced_plan_map
                .get(&abs_ls_id)
                .unwrap()
                .borrow_mut();

            for tm in &tm_vec {
                let extended_plan =
                    plan_with_extend_operator(&search_store, &mut plan, tm)?;
                *plan = extended_plan;
            }

            let serializer_operator =
                SerializerOperatorTranslator::translate_with_store(
                    &search_store,
                    &tm_vec,
                )?;

            plan.serialize(serializer_operator)
                .map_err(Into::<NewRMLTranslationError>::into)?
                .sink(&Target::default())
                .map_err(Into::<NewRMLTranslationError>::into)?;
        }

        Ok(search_store.root_plan.unwrap())
    }
}

fn plan_with_extend_operator(
    search_store: &SearchStore<'_>,
    plan: &mut RefMut<Plan<Processed>>,
    tm: &TriplesMap,
) -> Result<Plan<Processed>, NewRMLTranslationError> {
    let extend_op =
        ExtendOperatorTranslator::translate_with_store(search_store, tm)?;
    let extended_plan =
        plan.apply(&extend_op.into(), "ExtendOperator")
            .map_err::<NewRMLTranslationError, _>(|err| err.into())?;
    Ok(extended_plan)
}
