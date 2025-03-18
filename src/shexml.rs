use plangenerator::error::PlanError;
use plangenerator::states::Init;
use plangenerator::Plan;
use translator::{shexml::parcombi, LanguageTranslator};

use crate::handler::{FileTranslatorHandler, StringTranslatorHandler};

#[derive(Debug, Clone)]
pub struct ShExMLFileHandler;

#[derive(Debug, Clone)]
pub struct ShExMLStringHandler;

impl FileTranslatorHandler for ShExMLFileHandler {
    fn translate(
        &self,
        file_path: &dyn AsRef<str>,
    ) -> Result<Plan<Init>, PlanError> {
        let shexml_document = parcombi::parse_file(
            file_path.as_ref(),
        )
        .map_err(|shex_err| {
            PlanError::GenericError(format!(
                "Something went wrong while parsing shexml: \n {:?}",
                shex_err
            ))
        })?;

        translator::shexml::ShExMLTranslator::translate_to_plan(shexml_document)
    }

    fn supported_extension(&self) -> String {
        "shexml".to_string()
    }
}

impl StringTranslatorHandler for ShExMLStringHandler {
    fn translate(&self, mapping: &str) -> Result<Plan<Init>, PlanError> {
        let shexml_document = parcombi::parse_string(
            mapping.to_string(),
        )
        .map_err(|shex_err| {
            PlanError::GenericError(format!(
                "Something went wrong while parsing shexml: \n {:?}",
                shex_err
            ))
        })?;

        translator::shexml::ShExMLTranslator::translate_to_plan(shexml_document)
    }
}
