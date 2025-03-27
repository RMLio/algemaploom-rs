use super::*;
use crate::error::TranslationError;
use crate::test_case;

#[ignore]
#[test]
fn translate_to_plan_test() -> Result<(), TranslationError> {
    let input_shexml = test_case!("shexml/sample.shexml");
    let shexml_document = parcombi::parse_file(input_shexml).unwrap();

    ShExMLTranslator::translate_to_plan(shexml_document)?;
    Ok(())
}
