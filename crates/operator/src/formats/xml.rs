use serde::{Deserialize, Serialize};
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Default,
)]
pub struct XPathConfig {
    pub namespaces: Vec<Namespaces>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Default,
)]
pub struct Namespaces {
    pub prefix: String,
    pub url:    String,
}
