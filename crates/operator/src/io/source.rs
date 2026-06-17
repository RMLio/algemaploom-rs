use crate::display::PrettyDisplay;
use crate::io::io_type::IOType;
use crate::utils::hash_hashmap;
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

impl Source {

    /// "Merges" effective equally source `other` into this source.
    /// Note that checking for effectively equalness is completely the responsibility of the caller!
    /// At this moment, this only merges fields in the root iterator.
    pub fn merge(&mut self, other: &Source) {
        self.root_iterator.merge(&other.root_iterator);
    }
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