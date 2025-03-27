use plangenerator::states::Init;
use plangenerator::Plan;
use translator::error::TranslationError;
use translator::rml::parser::extractors::io::{parse_file, parse_str};
use translator::rml::OptimizedRMLDocumentTranslator;
use translator::LanguageTranslator;

use crate::handler::{FileTranslatorHandler, StringTranslatorHandler};

#[derive(Debug)]
pub struct RMLFileHandler;

#[derive(Debug)]
pub struct RMLStringHandler;

impl FileTranslatorHandler for RMLFileHandler {
    fn translate(
        &self,
        file_path: &dyn AsRef<str>,
    ) -> Result<Plan<Init>, TranslationError> {
        let document = parse_file(file_path.as_ref().into())?;
        OptimizedRMLDocumentTranslator::translate_to_plan(document)
    }

    fn supported_extension(&self) -> String {
        "ttl".to_string()
    }
}

impl StringTranslatorHandler for RMLStringHandler {
    fn translate(&self, mapping: &str) -> Result<Plan<Init>, TranslationError> {
        let document = parse_str(mapping)?;

        OptimizedRMLDocumentTranslator::translate_to_plan(document)
    }
}
