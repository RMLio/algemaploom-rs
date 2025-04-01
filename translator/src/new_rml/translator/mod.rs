mod store;

use std::path::{Path, PathBuf};

use store::SearchStore;

use super::rml_model::Document;
use crate::new_rml::extractors::io::parse_file;
use crate::LanguageTranslator;

#[derive(Debug, Clone)]
pub struct NewRMLDocumentTranslator {}

impl LanguageTranslator<&Path> for NewRMLDocumentTranslator {
    fn translate_to_plan(path: &Path) -> crate::LanguageTranslateResult {
        let document = parse_file(path.to_path_buf())?;
        NewRMLDocumentTranslator::translate_to_plan(document)
    }
}

impl LanguageTranslator<Document> for NewRMLDocumentTranslator {
    fn translate_to_plan(model: Document) -> crate::LanguageTranslateResult {
        let search_store = SearchStore::from_document(&model);


        todo!()
    }
}
