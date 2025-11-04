use std::collections::HashMap;

use lazy_static::lazy_static;
use sophia_api::term::Term;
use sophia_inmem::graph::FastGraph;
use sophia_term::{ArcTerm, RcTerm};

use crate::new_rml::extractors::error::ParseError;
use crate::new_rml::extractors::store::{get_object, get_objects};
use crate::new_rml::extractors::{stringify_term, ExtractorResult, FromVocab};

lazy_static! {
    static ref PARSE_CONFIGS_PREDICATES: Vec<(String, ArcTerm)> = vec![
        // Support for direct target URL
        (
            "url".to_string(),
            vocab::hctl::PROPERTY::HAS_TARGET.to_arcterm()
        ),
        // Support for content type
        (
            "contentType".to_string(),
            vocab::hctl::PROPERTY::FOR_CONTENT_TYPE.to_arcterm()
        ),
        // Support for sub-protocol (e.g., mqtt, stomp)
        (
            "subProtocol".to_string(),
            vocab::hctl::PROPERTY::FOR_SUB_PROTOCOL.to_arcterm()
        ),
    ];
}


pub fn extract_websocket_source(
    subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<HashMap<String, String>> {
    let mut config = HashMap::new();

    let property_affordance_pred = vocab::td::PROPERTY::HAS_PROPERTY_AFFORDANCE.to_rcterm();
    let property_affordances = get_objects(graph, subject, &property_affordance_pred);
    
        for property_affordance in property_affordances {
        let form_pred = vocab::td::PROPERTY::HAS_FORM.to_rcterm();
        let forms = get_objects(graph, property_affordance.borrow_term(), &form_pred);
        
        for form in forms {
            // Extract hasTarget (URL)
            let target_pred = vocab::hctl::PROPERTY::HAS_TARGET.to_arcterm();
            if let Ok(target) = get_object(graph, form.borrow_term(), &target_pred) {
                if let Some(url) = stringify_term(target) {
                    config.insert("url".to_string(), url);
                }
            }
            
            // Extract forContentType
            let content_type_pred = vocab::hctl::PROPERTY::FOR_CONTENT_TYPE.to_arcterm();
            if let Ok(content_type) = get_object(graph, form.borrow_term(), &content_type_pred) {
                if let Some(ct) = stringify_term(content_type) {
                    config.insert("contentType".to_string(), ct);
                }
            }
            
            // Extract forSubProtocol
            let sub_protocol_pred = vocab::hctl::PROPERTY::FOR_SUB_PROTOCOL.to_arcterm();
            if let Ok(sub_protocol) = get_object(graph, form.borrow_term(), &sub_protocol_pred) {
                if let Some(sp) = stringify_term(sub_protocol) {
                    config.insert("subProtocol".to_string(), sp);
                }
            }
        }
    }

    // Validate that we have at least a URL
    if !config.contains_key("url") {
        return Err(ParseError::GenericError(
            "WebSocket source must have a URL specified via hctl:hasTarget".to_string()
        ).into());
    }

    Ok(config)
}
