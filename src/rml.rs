use log::{error, info, warn};
use plan::states::Init;
use plan::Plan;
use translator::error::TranslationError;
use translator::new_rml::translator::NewRMLDocumentTranslator;
use translator::rml::parser::extractors::io::{
    parse_file as old_parse_file, parse_str as old_parse_str,
};
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
        info!("Trying to translate file {} with old RML spec translator https://rml.io/specs/rml/", file_path.as_ref());
        if let Ok(document) = old_parse_file(file_path.as_ref().into()) {
            OptimizedRMLDocumentTranslator::translate_to_plan(document)
        } else {
            warn!("Failed extracting with old RML spec translator");
            info!("Trying again with the new RML spec translator https://kg-construct.github.io/rml-resources/portal/");
            let document = translator::new_rml::extractors::io::parse_file(
                file_path.as_ref().into(),
            )?;

            NewRMLDocumentTranslator::translate_to_plan(document)
        }
    }

    fn supported_extension(&self) -> String {
        "ttl".to_string()
    }
}

impl StringTranslatorHandler for RMLStringHandler {
    fn translate(&self, mapping: &str) -> Result<Plan<Init>, TranslationError> {
        // TODO: Needs a better way to detect old vs new RML mapping document <16-04-25, Min Oo> //

        if mapping.contains("<http://www.w3.org/ns/r2rml#>") {
            // Old RML mapping document

            info!("Using translator for the Old RML spec https://rml.io/specs/rml/");
            let document = old_parse_str(mapping)?;
            OptimizedRMLDocumentTranslator::translate_to_plan(document)
        } else {
            // New RML mapping document shouldn't contain R2RML's prefix (BIIIGG ASSUMPTION)
            info!("Using translator for the latest RML spec https://kg-construct.github.io/rml-resources/portal/");
            let document =
                translator::new_rml::extractors::io::parse_str(mapping)?;

            NewRMLDocumentTranslator::translate_to_plan(document)
        }
    }
}
