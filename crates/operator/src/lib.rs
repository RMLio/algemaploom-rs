pub mod display;
pub mod formats;
mod test_util;
pub mod tuples;
pub mod value;
pub mod source;
pub mod join;
pub mod projection;
pub mod rename;
pub mod extend;
pub mod serializer;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub use crate::extend::Extend;
pub use crate::join::Join;
use crate::projection::Projection;
use crate::rename::Rename;
pub use crate::serializer::Serializer;
pub use crate::source::Source;
use anyhow::Result;
use display::{JsonDisplay, PrettyDisplay};
use formats::DataFormat;
use serde::{Deserialize, Serialize};

pub type RcOperator = Rc<Operator>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Operator {
    SourceOp {
        config: Source,
    },
    JoinOp {
        config: Join,
    },
    UnionOp,
    ProjectOp {
        config: Projection,
    },
    ExtendOp {
        config: Extend,
    },
    RenameOp {
        config: Rename,
    },
    SerializerOp {
        config: Serializer,
    },
    TargetOp {
        config: Target,
    }
}

impl From<Extend> for Operator {
    fn from(config: Extend) -> Self {
        Self::ExtendOp { config }
    }
}

impl JsonDisplay for Operator {
    fn json_string(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl PrettyDisplay for Operator {
    fn pretty_string(&self) -> Result<String> {
        let (title_string, content_string) = match self {
            Operator::SourceOp { config } => {
                ("Source Operator".to_string(), config.pretty_string()?)
            }
            Operator::ProjectOp { config } => {
                ("Projection Operator".to_string(), config.pretty_string()?)
            }
            Operator::ExtendOp { config } => {
                ("Extension Operator".to_string(), config.pretty_string()?)
            }
            Operator::RenameOp { config } => {
                ("Rename Operator".to_string(), config.pretty_string()?)
            }
            Operator::SerializerOp { config } => {
                ("Serializer Operator".to_string(), config.pretty_string()?)
            }
            Operator::TargetOp { config } => {
                ("Target Operator".to_string(), config.pretty_string()?)
            }
            Operator::JoinOp { config } => {
                ("Join Operator".to_string(), config.pretty_string()?)
            }
            Operator::UnionOp => ("Union Operator".to_string(), "".to_string()),
        };

        Ok(format!("{}\n{}", title_string, content_string))
    }
}

fn hash_hashmap<H, K, V>(hash_map: &HashMap<K, V>, state: &mut H)
where
    H: Hasher,
    K: Hash + Ord,
    V: Hash,
{
    let mut pairs: Vec<_> = hash_map.iter().collect();
    pairs.sort_by(|pair1, pair2| pair1.0.cmp(pair2.0));
    for (key, value) in pairs {
        key.hash(state);
        value.hash(state);
    }
}

// Post-mapping operators

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum IOType {
    StdIn,
    StdOut,
    File,
    Kafka,
    Websocket,
    RDB,
    SPARQLEndpoint,
}

impl Default for IOType {
    fn default() -> Self {
        Self::StdOut
    }
}

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
    fn pretty_string(&self) -> Result<String> {
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