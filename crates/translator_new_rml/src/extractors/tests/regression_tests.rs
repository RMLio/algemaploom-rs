use crate::translator::NewRMLDocumentTranslator;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use translator_api::LanguageTranslator;

#[test]
fn rmltc0000_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0000-JSON", true)
}

#[test]
fn rmltc0001a_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0001a-JSON", true)
}

#[test]
fn rmltc0001b_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0001b-JSON", true)
}

fn execute(test_name: &str, must_succeed: bool) -> Result<(), Box<dyn Error>> {
    let test_path = test_path(test_name);
    let mapping_path = test_path.join("mapping.ttl");
    let plan_result = NewRMLDocumentTranslator::translate_to_plan(mapping_path.as_path());
    if let Ok(plan) = plan_result {
        if !must_succeed {
            panic!("A plan was generated but the test was expected to fail");
        }
        let actual_json_plan = plan.to_json_string()?;
        let actual_plan: serde_json::Value = serde_json::from_str(&actual_json_plan)?;

        let expected_json_path = test_path.join("mapping.json");
        let expected_json_plan = fs::read_to_string(expected_json_path)?;
        let expected_plan: serde_json::Value = serde_json::from_str(&expected_json_plan)?;

        assert_eq!(actual_plan, expected_plan);
    } else {
        if must_succeed {
            panic!("No plan was generated but the test was expected to pass");
        }
    }

    Ok(())
}

#[inline]
fn test_path(test_dir_name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources");
    path.push("test");
    path.push("rml-core-tests");
    path.push(test_dir_name);
    path
}