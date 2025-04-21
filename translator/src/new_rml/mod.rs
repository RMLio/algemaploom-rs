pub mod error;
pub mod extractors;
pub mod rml_model;
pub mod translator;

#[cfg(test)]
mod tests {
    use chumsky::chain::Chain;

    use super::extractors::io;
    use super::*;
    use crate::error::TranslationError;
    use crate::new_rml::translator::NewRMLDocumentTranslator;
    use crate::{test_case, LanguageTranslator};

    #[test]
    fn simple_rml_test() -> Result<(), TranslationError> {
        let input_file =
            test_case!("rml-core-tests/RMLTC0000-JSON/mapping.ttl");
        let document = io::parse_file(input_file.into())?;
        assert!(
            document.triples_maps.len() == 1,
            "{} triples maps extracted",
            document.triples_maps.len()
        );

        let plan_translated =
            NewRMLDocumentTranslator::translate_to_plan(document)?;

        assert!(
            plan_translated.sources.len() == 1,
            "{} source operators translated",
            plan_translated.sources.len()
        );

        Ok(())
    }
}
