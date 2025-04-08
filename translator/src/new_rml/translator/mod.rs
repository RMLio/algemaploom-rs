pub mod error;
mod source;
mod store;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use operator::Source;
use plangenerator::data_type::RcRefCellPlan;
use plangenerator::states::Processed;
use plangenerator::Plan;
use sophia_term::RcTerm;
use source::AbstractLogicalSourceTranslator;
use store::SearchStore;

use super::error::NewRMLTranslationResult;
use super::rml_model::Document;
use crate::error::TranslationError;
use crate::new_rml::extractors::io::parse_file;
use crate::LanguageTranslator;

pub trait OperatorTranslator {
    type Input;
    type Output;

    fn translate(input: &Self::Input) -> NewRMLTranslationResult<Self::Output> {
        unimplemented!("Operator translator does not support translation without the usage of a search store")
    }
    fn translate_with_store(
        store: &SearchStore,
        input: &Self::Input,
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

        for (abs_ls_id, tm_vec) in search_store.partition_lsid_tmid() {
            let plan =
                search_store.ls_id_sourced_plan_map.get(&abs_ls_id).unwrap();
        }

        todo!()
    }
}
