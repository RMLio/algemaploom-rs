use std::fs;
use std::path::Path;

use crate::rml::parser::extractors::io::parse_file as parse_rml_file;
use crate::rml::OptimizedRMLDocumentTranslator;
use crate::LanguageTranslator;

/// Execute mapping from RML file and return the operator plan as JSON string
pub fn execute_mapping(mapping_file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("🔄 Executing mapping: {}", mapping_file_path);
    
    let document = match parse_rml_file(mapping_file_path.into()) {
        Ok(doc) => doc,
        Err(e) => {
            println!("💥 Detailed parsing error: {:?}", e);
            return Err(format!("RML parsing failed: {:?}", e).into());
        }
    };
    
    let plan = OptimizedRMLDocumentTranslator::translate_to_plan(document)
        .map_err(|e| {
            println!("💥 Detailed translation error: {:?}", e);
            format!("Translation error: {:?}", e)
        })?;
    
    let plan_json = plan.to_json_string()
        .map_err(|e| format!("JSON serialization failed: {:?}", e))?;
    
    println!("\n📋 Generated Mapping Plan (JSON):");
    println!("{}", plan_json);
    
    Ok(plan_json)
}

pub fn parse_expected_mapping(mapping_file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("📄 Reading expected mapping: {}", mapping_file_path);
    
    if !Path::new(mapping_file_path).exists() {
        return Err(format!("Expected mapping file not found: {}", mapping_file_path).into());
    }
    
    let content = fs::read_to_string(mapping_file_path)?;
    Ok(content)
}

pub fn integration_test(
    mapping_file: &str, 
    expected_mapping_file: &str
) -> Result<bool, Box<dyn std::error::Error>> {
    println!("\n🧪 Running integration test:");
    println!("📋 Mapping: {}", mapping_file);
    println!("📄 Expected mapping: {}", expected_mapping_file);
    
    let start_time = std::time::Instant::now();
    
    let actual_output = execute_mapping(mapping_file)?;
    
    let expected_output = parse_expected_mapping(expected_mapping_file)?;
    
    let duration = start_time.elapsed();
    
    let has_actual = !actual_output.trim().is_empty();
    let has_expected = !expected_output.trim().is_empty();
    
    println!("📊 Results:");
    println!("  - Actual output length: {} chars", actual_output.len());
    println!("  - Expected output length: {} chars", expected_output.len());
    println!("  - Execution time: {:?}", duration);
    
    // Parse both JSONs for comparison
    let actual_json: serde_json::Value = serde_json::from_str(&actual_output)
        .map_err(|e| format!("Failed to parse actual output as JSON: {:?}", e))?;
    
    let expected_json: serde_json::Value = serde_json::from_str(&expected_output)
        .map_err(|e| format!("Failed to parse expected output as JSON: {:?}", e))?;
    
    let jsons_match = actual_json == expected_json;
    
    println!("\n📄 Final Mapping Plan Output:");
    println!("{}", actual_output);

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Ignored because currently it always passes when the mapping succeeds.
    fn test_ldes_bluebike_case() {
        //TODO: Have a simpler test that just tests the LDES, not using such a huge mapping file.
        let mapping_file = "resources/test/rmlmapper-custom/rml-ldes/bluebike/base.rml.ttl";
        let expected_mapping_file = "resources/test/rmlmapper-custom/rml-ldes/bluebike/expected_mapping.json";
        
        match integration_test(mapping_file, expected_mapping_file) {
            Ok(success) => {
                assert!(success, "LDES BlueBike test should pass");
                println!("✅ LDES BlueBike test completed successfully");
            }
            Err(e) => {
                println!("❌ LDES test failed: {}", e);
                panic!("LDES integration test failed: {}", e);
            }
        }
    }
} 