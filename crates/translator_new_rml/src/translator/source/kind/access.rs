use std::collections::HashMap;

use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use crate::extractors::ExtractorResult;


pub fn extract_access(
    _subject: &RcTerm,
    _graph: &FastGraph,
) -> ExtractorResult<HashMap<String, String>> {
    Ok(HashMap::new())
}
