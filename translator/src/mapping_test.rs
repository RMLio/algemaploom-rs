// Tests for during development to debug potential issues

use std::path::Path;

use crate::rml::parser::extractors::io::parse_file as parse_rml_file;
use crate::rml::OptimizedRMLDocumentTranslator;
use crate::new_rml::extractors::io::parse_file as parse_new_rml_file;
use crate::new_rml::translator::NewRMLDocumentTranslator;
use crate::LanguageTranslator;

// This is a testing file for debugging and finding out issues during development.

pub fn execute_mapping(mapping_name: &str, mapping_file_path: &str, use_new_rml: bool) -> Result<String, Box<dyn std::error::Error>> {
    let rml_type = if use_new_rml { "New RML" } else { "RML" };
    println!("🔄 Executing {} mapping ({}): {}", rml_type, mapping_name, mapping_file_path);
    
    if !Path::new(mapping_file_path).exists() {
        return Err(format!("Mapping file not found: {}", mapping_file_path).into());
    }
    
    let plan_json = if use_new_rml {
        let document = parse_new_rml_file(mapping_file_path.into())
            .map_err(|e| format!("New RML parsing failed: {:?}", e))?;
    
        let plan = NewRMLDocumentTranslator::translate_to_plan(document)
            .map_err(|e| format!("New RML translation error: {:?}", e))?;
        
        plan.to_json_string()
            .map_err(|e| format!("JSON serialization failed: {:?}", e))?
    } else {
        let document = parse_rml_file(mapping_file_path.into())
            .map_err(|e| format!("RML parsing failed: {:?}", e))?;
    
        let plan = OptimizedRMLDocumentTranslator::translate_to_plan(document)
            .map_err(|e| format!("Translation error: {:?}", e))?;
        
        plan.to_json_string()
            .map_err(|e| format!("JSON serialization failed: {:?}", e))?
    };
    
    println!("✅ {} mapping ({}) executed successfully", rml_type, mapping_name);
    
    // Print the final mapping JSON in a structured way
    println!("\n📋 Final Mapping Plan JSON for {} ({}):", rml_type, mapping_name);
    println!("{}", "=".repeat(80));
    
    // Try to pretty-print the JSON, fall back to raw if parsing fails
    match serde_json::from_str::<serde_json::Value>(&plan_json) {
        Ok(parsed) => {
            match serde_json::to_string_pretty(&parsed) {
                Ok(pretty) => println!("{}", pretty),
                Err(_) => println!("{}", plan_json),
            }
        },
        Err(_) => println!("{}", plan_json),
    }
    
    println!("{}", "=".repeat(80));
    println!();

    Ok(plan_json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_ldes_bluebike_case() {
        execute_mapping("LDES BlueBike", "resources/test/rmlmapper-custom/rml-ldes/bluebike/base.rml.ttl", false).unwrap();
            }

    #[test]
    #[ignore]
    fn test_kafka_mapping() {
        execute_mapping("Kafka", "resources/test/rmlstreamer/RMLTC0007e-XML-STREAM-KAFKA/mapping.ttl", false).unwrap();
    }

    #[test]
    #[ignore]
    fn test_csv_2a() {
        execute_mapping("CSV 2a", "resources/test/csv-testcases/RMLTC0002a-CSV/mapping.ttl", false).unwrap();
    }

    #[test]
    #[ignore]
    fn test_function_mapping() {
        execute_mapping("Function", "resources/test/rml/function_mapping.ttl", false).unwrap();
    }

    #[test]
    #[ignore]
    fn test_new_rmlfnml() {
        execute_mapping("RMLFNML", "resources/test/rmlfnml/RMLFNMLTC0001-CSV/mapping.ttl", true).unwrap();
    }

    #[test]
    #[ignore]
    fn test_new_rmlstar() {
        execute_mapping("RMLStar", "resources/test/rmlstar/RMLSTARTC001b/mapping.ttl", true).unwrap();
            }
}
