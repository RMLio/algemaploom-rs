use crate::display::PrettyDisplay;
use crate::formats::DataFormat;
use crate::hash_hashmap;
use crate::io::io_type::IOType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Target {
    #[serde(flatten)]
    pub configuration: HashMap<String, String>,
    pub target_type:   IOType,
    pub data_format:   DataFormat,
}

impl Default for Target {
    fn default() -> Self {
        Self {
            configuration: Default::default(),
            target_type:   IOType::StdOut,
            data_format:   DataFormat::NQuads,
        }
    }
}

impl PrettyDisplay for Target {
    fn pretty_string(&self) -> anyhow::Result<String> {
        let result = format!(
            "type: {:?} \ndata format: {:?} \nconfig: {}
             ",
            self.target_type,
            self.data_format,
            serde_json::to_string_pretty(&self.configuration)?
        );
        Ok(result)
    }
}

impl Hash for Target {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_hashmap(&self.configuration, state);
        self.target_type.hash(state);
        self.data_format.hash(state);
    }
}