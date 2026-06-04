//! This module defines the data formats and reference formulations supported by operators
//! Source, Target and Serializer.

use serde::{Deserialize, Serialize};
use xml::XPathConfig;
pub mod xml;

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Default,
)]
pub enum ReferenceFormulation {
    #[default]
    CSVRows,
    JSONPath,
    XMLPath(XPathConfig),
    XMLQuery,
    SQLQuery,
    SPARQL,
    CSS3,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Default,
)]
pub enum DataFormat {
    JSONLD,
    JSON,
    XML,
    #[default]
    CSV,
    TTL,
    NQuads,
    NTriples,
    SQL,
}
