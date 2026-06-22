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
// Data error, so mapping should succeed.
fn rmltc0002e_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0002e-JSON", true)
}

#[test]
// Data error, so mapping should succeed.
fn rmltc0002g_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0002g-JSON", true)
}

#[test]
fn rmltc0003c_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0003c-JSON", true)
}

#[test]
fn rmltc0004a_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0004a-JSON", true)
}

#[test]
fn rmltc0004b_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0004b-JSON", false)
}

#[test]
fn rmltc0005a_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0005a-JSON", true)
}

#[test]
fn rmltc0006a_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0006a-JSON", true)
}

#[test]
fn rmltc0007a_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0007a-JSON", true)
}

#[test]
fn rmltc0007b_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0007b-JSON", true)
}

#[test]
fn rmltc0007c_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0007c-JSON", true)
}

#[test]
fn rmltc0007d_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0007d-JSON", true)
}

#[test]
fn rmltc0007e_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0007e-JSON", true)
}

#[test]
fn rmltc0007f_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0007f-JSON", true)
}

#[test]
fn rmltc0007g_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0007g-JSON", true)
}

#[test]
fn rmltc0007h_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0007h-JSON", false)
}

#[test]
fn rmltc0008a_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0008a-JSON", true)
}

#[test]
fn rmltc0008b_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0008b-JSON", true)
}

#[test]
fn rmltc0008c_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0008c-JSON", true)
}

#[test]
fn rmltc0009a_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0009a-JSON", true)
}

#[test]
fn rmltc0009b_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0009b-JSON", true)
}

#[test]
fn rmltc0010a_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0010a-JSON", true)
}

#[test]
fn rmltc0010b_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0010b-JSON", true)
}

#[test]
fn rmltc0010c_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0010c-JSON", true)
}

#[test]
fn rmltc0011b_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0011b-JSON", true)
}

#[test]
fn rmltc0012a_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0012a-JSON", true)
}

#[test]
fn rmltc0012b_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0012b-JSON", true)
}

#[test]
fn rmltc0012c_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0012c-JSON", false)
}

#[test]
fn rmltc0012d_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0012d-JSON", false)
}

#[test]
#[ignore = "Blank node term type in Subject Map not supported yet"]
fn rmltc0012e_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0012e-JSON", true)
}

#[test]
fn rmltc0013a_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0013a-JSON", true)
}

#[test]
fn rmltc0015a_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0015a-JSON", true)
}

#[test]
// Data error, so mapping should succeed.
fn rmltc0015b_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0015b-JSON", true)
}

#[test]
fn rmltc0019a_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0019a-JSON", true)
}

#[test]
// Data error, so mapping should succeed.
fn rmltc0019b_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0019b-JSON", true)
}

#[test]
fn rmltc0020a_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0020a-JSON", true)
}

#[test]
fn rmltc0021a_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0021a-JSON", true)
}

#[test]
fn rmltc0022a_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0022a-JSON", true)
}

#[test]
fn rmltc0022b_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0022b-JSON", true)
}

#[test]
fn rmltc0022c_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0022c-JSON", true)
}

#[test]
fn rmltc0022d_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0022d-JSON", true)
}

#[test]
fn rmltc0022e_json() -> Result<(), Box<dyn Error>> {
    execute("RMLTC0022e-JSON", true)
}

// #[test]
//  fn rmltc0023a_json() -> Result<(), Box<dyn Error>> {
//      execute("RMLTC0023a-JSON", false)
//  }


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