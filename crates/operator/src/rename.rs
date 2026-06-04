use crate::display::PrettyDisplay;
use crate::utils::hash_hashmap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rename {
    pub alias:        Option<String>,
    #[serde(flatten)]
    pub rename_pairs: HashMap<String, String>,
}

impl PrettyDisplay for Rename {
    fn pretty_string(&self) -> anyhow::Result<String> {
        let alias_string = match &self.alias {
            Some(inner) => format!("{}\n", inner),
            None => "".to_string(),
        };

        let pairs_string = self
            .rename_pairs
            .iter()
            .map(|kv_pair| format!("{} -> {}", kv_pair.0, kv_pair.1))
            .collect::<Vec<String>>()
            .join("\n");

        Ok(format!(
            "Renaming pairs:\n {}{}",
            alias_string, pairs_string
        ))
    }
}

impl Hash for Rename {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_hashmap(&self.rename_pairs, state);
    }
}