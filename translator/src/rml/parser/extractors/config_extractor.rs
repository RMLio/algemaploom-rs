use std::collections::HashMap;

use sophia_api::term::Term;
use sophia_inmem::graph::FastGraph;
use sophia_term::{ArcTerm, RcTerm};

use super::rcterm_to_string;
use crate::rml::parser::extractors::store::get_object;
use crate::rml::parser::extractors::ExtractorResult;

pub fn extract_parse_config(
    dialect_subject: &RcTerm,
    graph: &FastGraph,
    predicates: &Vec<(String, ArcTerm)>,
) -> ExtractorResult<HashMap<String, String>> {
    let mut result = HashMap::new();
    let _ = predicates.iter().try_for_each(
        |(key, config_pred)| -> ExtractorResult<()> {
            // Retrieve the config value for the subject-predicate pair
            let config_val = get_object(graph, dialect_subject, config_pred);

            if let Ok(val) = config_val {
                result.insert(key.to_string(), rcterm_to_string(&val));
            }

            Ok(())
        },
    );

    Ok(result)
}
