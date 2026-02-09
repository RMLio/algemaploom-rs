use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Default,
)]
pub enum ReferenceFormulation {
    #[default]
    CSVRows,
    JSONPath,
    XMLPath,
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
