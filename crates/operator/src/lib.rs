pub mod display;
pub mod formats;
mod test_util;
pub mod tuples;
pub mod value;
pub mod source;
pub mod join;
pub mod projection;

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub use crate::join::Join;
use crate::projection::Projection;
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rename {
    pub alias:        Option<String>,
    #[serde(flatten)]
    pub rename_pairs: HashMap<String, String>,
}

impl PrettyDisplay for Rename {
    fn pretty_string(&self) -> Result<String> {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Extend {
    #[serde(flatten)]
    pub extend_pairs: HashMap<String, Function>,
}

impl Extend {
    pub fn extend_with(self, other: Self) -> Self {
        let mut this_pairs = self.extend_pairs;
        let other_pairs = other.extend_pairs;

        this_pairs.extend(other_pairs);

        Extend {
            extend_pairs: this_pairs,
        }
    }
}

impl PrettyDisplay for Extend {
    fn pretty_string(&self) -> Result<String> {
        let vec_pairs: Vec<_> = self
            .extend_pairs
            .iter()
            .map(|pair| format!("{} -> {:?}", pair.0, pair.1))
            .collect();

        Ok(format!("Extended pairs: \n {}", vec_pairs.join("\n")))
    }
}

pub type RcExtendFunction = Rc<Function>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TermType {
    Literal,
    IRI,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Function {
    Nop,
    SimpleConcat {
        inner_function: Option<RcExtendFunction>,
    },
    Concatenate {
        left_value:  RcExtendFunction,
        separator:   String,
        right_value: RcExtendFunction,
    },
    Reference {
        value: String,
    },
    TypedConstant {
        value:     String,
        term_type: TermType,
    },

    Constant {
        value: String,
    },
    TemplateString {
        value: String,
    },

    Replace {
        replace_map:    HashMap<String, HashSet<String>>,
        inner_function: RcExtendFunction,
    },

    TemplateFunctionValue {
        template:                String,
        variable_function_pairs: Vec<(String, RcExtendFunction)>,
    },
    IriEncode {
        inner_function: RcExtendFunction,
    },
    UriEncode {
        inner_function: RcExtendFunction,
    },
    Iri {
        base_iri:       Option<String>,
        inner_function: RcExtendFunction,
    },
    Literal {
        inner_function:    RcExtendFunction,
        dtype_function:    Option<RcExtendFunction>,
        langtype_function: Option<RcExtendFunction>,
    },
    BlankNode {
        inner_function: RcExtendFunction,
    },
    Upper {
        inner_function: RcExtendFunction,
    },
    Lower {
        inner_function: RcExtendFunction,
    },
    FnO {
        fno_identifier: String,
        parameters:     HashMap<String, RcExtendFunction>,
        return_type:    Option<String>,
    },
    Star {
        // TODO: Implement star function
    },
}

// Post-mapping operators

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Serializer {
    pub template: String,
    #[serde(flatten)]
    pub options:  Option<HashMap<String, String>>,
    pub format:   DataFormat,
}

impl PrettyDisplay for Serializer {
    fn pretty_string(&self) -> Result<String> {
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