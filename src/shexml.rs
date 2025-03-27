use plangenerator::error::PlanError;
use plangenerator::states::Init;
use plangenerator::Plan;
use translator::error::TranslationError;
use translator::shexml::error::ShExMLTranslationError;
use translator::shexml::{parcombi, ShExMLTranslator};
use translator::LanguageTranslator;

use crate::handler::{FileTranslatorHandler, StringTranslatorHandler};

#[derive(Debug, Clone)]
pub struct ShExMLFileHandler;

#[derive(Debug, Clone)]
pub struct ShExMLStringHandler;

impl FileTranslatorHandler for ShExMLFileHandler {
    fn translate(
        &self,
        file_path: &dyn AsRef<str>,
    ) -> Result<Plan<Init>, TranslationError> {
        let shexml_document =
            parcombi::parse_file(file_path.as_ref())
                .map_err::<ShExMLTranslationError, _>(|err| err.into())?;

        ShExMLTranslator::translate_to_plan(shexml_document)
    }

    fn supported_extension(&self) -> String {
        "shexml".to_string()
    }
}

impl StringTranslatorHandler for ShExMLStringHandler {
    fn translate(&self, mapping: &str) -> Result<Plan<Init>, TranslationError> {
        let shexml_document =
            parcombi::parse_string(mapping.to_string())
                .map_err::<ShExMLTranslationError, _>(|err| err.into())?;

        ShExMLTranslator::translate_to_plan(shexml_document)
    }
}
