use std::collections::HashMap;
use sophia_api::term::TTerm;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;
use crate::rml::parser::extractors::ExtractorResult;
use crate::rml::parser::extractors::store::get_object;
use super::TermString;

pub fn extract_parse_config(
    dialect_subject: &RcTerm,
    graph: &FastGraph,
    predicates: &Vec<(String, TermString)>
) -> ExtractorResult<HashMap<String, String>> {
    let mut result = HashMap::new();
    let _ = predicates.iter().try_for_each(
        |(key, config_pred)| -> ExtractorResult<()> {
            // Retrieve the config value for the subject-predicate pair
            let config_val = get_object(graph, dialect_subject, config_pred);

            if let Ok(val) = config_val {
                result.insert(key.to_string(), val.value().to_string());
            }

            Ok(())
        },
    );

    Ok(result)
}
