use crate::display::PrettyDisplay;
use crate::formats::DataFormat;
use crate::utils::hash_hashmap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Serializer {
    pub template: String,
    #[serde(flatten)]
    pub options:  Option<HashMap<String, String>>,
    pub format:   DataFormat,
}

impl PrettyDisplay for Serializer {
    fn pretty_string(&self) -> anyhow::Result<String> {
        let format_type = format!("Format type: {:?}", self.format);

        Ok(format!("{}\nTemplate: {}", format_type, self.template))
    }
}

impl Hash for Serializer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.template.hash(state);
        if let Some(option_map) = self.options.as_ref() {
            hash_hashmap(option_map, state);
        }
        self.format.hash(state);
    }
}
