
use plangenerator::error::PlanError;

use super::*;
use crate::test_case;

#[ignore]
#[test]
fn translate_to_plan_test() -> Result<(), PlanError> {
    let input_shexml = test_case!("shexml/sample.shexml");
    let shexml_document = shexml_interpreter::parse_file(input_shexml).unwrap();

    ShExMLTranslator::translate_to_plan(shexml_document)?;
    Ok(())
}
