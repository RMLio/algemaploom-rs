mod source;
mod store;
pub mod error;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use operator::Source;
use plangenerator::data_type::RcRefCellPlan;
use plangenerator::states::Processed;
use plangenerator::Plan;
use source::AbstractLogicalSourceTranslator;
use store::SearchStore;

use super::error::NewRMLTranslationResult;
use super::rml_model::Document;
use crate::new_rml::extractors::io::parse_file;
use crate::LanguageTranslator;

pub trait OperatorTranslator {
    type Input;
    type Output;

    fn translate(store: &SearchStore, input: &Self::Input) -> NewRMLTranslationResult<Self::Output>;
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

        let search_store = SearchStore::from_document(&model);
        let mut ls_id_sourced_plan_map = HashMap::new();

        for abs_ls in search_store.abs_ls_search_map.values().copied() {
            let mut plan = Plan::new();

            let source = AbstractLogicalSourceTranslator::translate(
                &search_store,
                abs_ls,
            )?;
            let sourced_plan: RcRefCellPlan<Processed> =
                plan.source(source).into();

            ls_id_sourced_plan_map
                .insert(abs_ls.get_identifier(), sourced_plan);
        }

        for (abs_ls_id, tm_vec) in search_store.partition_lsid_tmid() {}

        todo!()
    }
}
