use crate::error::TranslationError;
use plan::states::Init;
use plan::Plan;
use translator_api::LanguageTranslator;
use translator_shexml::error::ShExMLTranslationError;
use translator_shexml::{parcombi, ShExMLTranslator};

use crate::handler::TranslatorHandler;

#[derive(Debug, Clone)]
pub struct ShExMLHandler;

impl TranslatorHandler for ShExMLHandler {
    fn translate(&self, mapping: &str) -> Result<Plan<Init>, TranslationError> {
        let shexml_document =
            parcombi::parse_string(mapping.to_string())
                .map_err::<ShExMLTranslationError, _>(|err| err.into())?;

        Ok(ShExMLTranslator::translate_to_plan(shexml_document)?)
    }
}
