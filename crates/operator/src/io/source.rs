use crate::display::PrettyDisplay;
use crate::hash_hashmap;
use crate::io::io_type::IOType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub mod iterator;
pub mod field;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Source {
    #[serde(flatten)]
    pub config:        HashMap<String, String>,
    pub access:        HashMap<String, String>,
    pub source_type:   IOType,
    pub root_iterator: iterator::Iterator,
}

impl PrettyDisplay for Source {
    fn pretty_string(&self) -> anyhow::Result<String> {
        let result = format!(
            "type: {:?} \nreference iterator: {:#?} \nconfig: {}\naccess: {}
            ",
            self.source_type,
            self.root_iterator,
            serde_json::to_string_pretty(&self.config)?,
            serde_json::to_string_pretty(&self.access)?
        );
        Ok(result)
    }
}

impl Hash for Source {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_hashmap(&self.config, state);
        hash_hashmap(&self.access, state);
        self.source_type.hash(state);
        self.root_iterator.hash(state);
    }
}