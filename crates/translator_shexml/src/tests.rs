use super::*;
use crate::test_case;

#[ignore]
#[test]
fn translate_to_plan_test() -> Result<(), ShExMLTranslationError> {
    let input_shexml = test_case!("shexml/simple/input.shexml");
    let shexml_document = parcombi::parse_file(input_shexml)?;

    ShExMLTranslator::translate_to_plan(shexml_document)?;
    Ok(())
}
