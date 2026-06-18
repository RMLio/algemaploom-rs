use crate::translator::NewRMLDocumentTranslator;
use std::error::Error;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
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

#[test]
fn rmltc0002a_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0002a-JSON", true)
}

#[test]
fn rmltc0002b_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0002b-JSON", true)
}

#[test]
fn rmltc0002e_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0002e-JSON", true)
}

#[test]
fn rmltc0002g_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0002g-JSON", true)
}

#[test]
fn rmltc0003c_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0003c-JSON", true)
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
        let actual_plan_hash = json_hash(&actual_json_plan);

        let expected_json_path = test_path.join("mapping.json");
        let expected_json_str = fs::read_to_string(expected_json_path)?;
        let expected_plan_hash = json_hash(&expected_json_str);
        assert_eq!(actual_plan_hash, expected_plan_hash, "Generated plan did not match expected plan.\nActual:\n{}\nExpected:\n{}", actual_json_plan, expected_json_str);
    } else {
        if must_succeed {
            panic!("No plan was generated but the test was expected to pass");
        }
    }

    Ok(())
}

/// Deserializes a JSON string to a JSON object and then serializes it back to a string.
/// Then the hash of this string is returned.
/// This workaround catches the case where comparing JSON objects is not enough,
/// e.g. if strings within a template are switched between different runs.
fn json_hash(json_str: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    let value = serde_json::from_str::<serde_json::Value>(json_str).unwrap();
    let unsorted_str = serde_json::to_string(&value).unwrap();
    let mut char_str = unsorted_str.chars().collect::<Vec<char>>();
    char_str.sort();
    char_str.hash(&mut hasher);
    hasher.finish()
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