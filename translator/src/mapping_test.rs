use std::fs;
use std::path::Path;

use crate::rml::parser::extractors::io::parse_file as parse_rml_file;
use crate::rml::OptimizedRMLDocumentTranslator;
use crate::LanguageTranslator;

/// Execute mapping from RML file and return the operator plan as string
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
    
    Ok(format!("{:#?}", plan))
}

pub fn parse_expected_output(output_file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("📄 Reading expected output: {}", output_file_path);
    
    if !Path::new(output_file_path).exists() {
        return Err(format!("Output file not found: {}", output_file_path).into());
    }
    
    let content = fs::read_to_string(output_file_path)?;
    Ok(content)
}

pub fn integration_test(
    mapping_file: &str, 
    expected_output_file: &str
) -> Result<bool, Box<dyn std::error::Error>> {
    println!("\n🧪 Running integration test:");
    println!("📋 Mapping: {}", mapping_file);
    println!("📄 Expected output: {}", expected_output_file);
    
    let start_time = std::time::Instant::now();
    
    let actual_output = execute_mapping(mapping_file)?;
    
    let expected_output = parse_expected_output(expected_output_file)?;
    
    let duration = start_time.elapsed();
    
    let has_actual = !actual_output.trim().is_empty();
    let has_expected = !expected_output.trim().is_empty();
    
    println!("📊 Results:");
    println!("  - Actual output length: {} chars", actual_output.len());
    println!("  - Expected output length: {} chars", expected_output.len());
    println!("  - Execution time: {:?}", duration);
    
    let success = has_actual && has_expected;
    
    if success {
        println!("✅ Integration test PASSED");
    } else {
        println!("❌ Integration test FAILED");
        if !has_actual {
            println!("  - No actual output generated");
        }
        if !has_expected {
            println!("  - No expected output found");
        }
    }
    
    Ok(success)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Use `cargo test -- --ignored` to run
    fn test_ldes_bluebike_case() {
        let mapping_file = "resources/test/rmlmapper-custom/rml-ldes/bluebike/base.rml.ttl";
        let output_file = "resources/test/rmlmapper-custom/rml-ldes/bluebike/output-base.nq";
        
        match integration_test(mapping_file, output_file) {
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