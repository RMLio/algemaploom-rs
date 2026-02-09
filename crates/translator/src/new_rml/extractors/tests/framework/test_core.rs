use std::path::PathBuf;
use std::collections::HashMap;

use crate::new_rml::extractors::io::parse_file;
use crate::new_rml::extractors::ExtractorResult;
use crate::new_rml::rml_model::v2::core::TriplesMap;

pub fn extract_triplesmaps_from_test(filename: &str) -> ExtractorResult<Vec<TriplesMap>> {
    let test_path = format!("resources/test/{}", filename);
    let path = PathBuf::from(test_path);
    let document = parse_file(path)?;
    Ok(document.triples_maps)
}

pub fn get_triplemap_identifier(tm: &TriplesMap) -> String {
    crate::new_rml::extractors::stringify_term(&tm.identifier)
        .unwrap_or_else(|| "<unnamed>".to_string())
}

pub trait Expectation {
    fn check(&self, data: &TriplesMap) -> bool;
    fn describe_failure(&self, data: &TriplesMap) -> String;
    fn name(&self) -> &str;
}

pub struct Expect<T, F> {
    pub extractor: F,
    pub expected: T,
    pub name: String,
}

impl<T: PartialEq + Clone + std::fmt::Debug, F: Fn(&TriplesMap) -> Result<T, String>> Expectation for Expect<T, F> {
    fn check(&self, data: &TriplesMap) -> bool {
        match (self.extractor)(data) {
            Ok(actual) => actual == self.expected,
            Err(_) => false, // Extraction error = test failure
        }
    }

    fn describe_failure(&self, data: &TriplesMap) -> String {
        match (self.extractor)(data) {
            Ok(actual) => format!(
                    "{}: Expected {:?}, but got {:?}", 
                    self.name, self.expected, actual
                ),
            Err(error) => format!(
                "{}: Expected {:?}, but extraction failed with error: {}", 
                self.name, self.expected, error
                ),
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

pub fn test_triplesmaps(filename: &str, triplemap_tests: Vec<(&str, Vec<Box<dyn Expectation>>)>) {
    let expectations_map: HashMap<String, Vec<Box<dyn Expectation>>> = triplemap_tests
        .into_iter()
        .map(|(id, expectations)| (id.to_string(), expectations))
        .collect();
    
    // Extract the TriplesMap objects from the file
    let triplesmaps = extract_triplesmaps_from_test(filename)
        .expect(&format!("Failed to extract TriplesMap objects from {}", filename));
    
    // Create a map from ID to TriplesMap for quick lookup
    let triplesmaps_by_id: HashMap<String, &TriplesMap> = triplesmaps
        .iter()
        .map(|tm| (get_triplemap_identifier(tm), tm))
        .collect();
    
    // Test each expected TriplesMap
    for (expected_id, expectations) in expectations_map {
        let triplemap = triplesmaps_by_id.get(&expected_id)
            .expect(&format!("TriplesMap with ID '{}' not found in mapping file", expected_id));
        
        // Run all expectations for this TriplesMap
        for expectation in expectations {
            if !expectation.check(triplemap) {
                panic!("{}", expectation.describe_failure(triplemap));
            }
        }
    }
}

pub fn expect_parse_fail(filename: &str) {
    let test_path = format!("resources/test/{}", filename);
    let path = PathBuf::from(test_path);
    
    match parse_file(path) {
        Ok(_) => panic!("Expected parsing to fail for file '{}', but it succeeded", filename),
        Err(_) => {
            // This is expected - parsing should fail
            println!("✓ Parsing correctly failed for file '{}'", filename);
        }
    }
}
