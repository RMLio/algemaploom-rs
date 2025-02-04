use plangenerator::error::PlanError;
use plangenerator::states::Init;
use plangenerator::Plan;
use rml_interpreter::extractors::io::{parse_file, parse_str};
use translator::rmlalgebra::OptimizedRMLDocumentTranslator;
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
    ) -> Result<Plan<Init>, PlanError> {
        let document = parse_file(file_path.as_ref().into())
            .map_err(|err| PlanError::GenericError(format!("{:?}", err)))?;

        OptimizedRMLDocumentTranslator::translate_to_plan(document)
    }

    fn supported_extension(&self) -> String {
        "ttl".to_string()
    }
}

impl StringTranslatorHandler for RMLStringHandler {
    fn translate(&self, mapping: &str) -> Result<Plan<Init>, PlanError> {
        let document = parse_str(mapping)
            .map_err(|err| PlanError::GenericError(format!("{:?}", err)))?;

        OptimizedRMLDocumentTranslator::translate_to_plan(document)
    }
}
