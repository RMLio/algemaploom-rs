pub mod display;
pub mod formats;
mod test_util;
pub mod tuples;
pub mod value;

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use anyhow::Result;
use display::{JsonDisplay, PrettyDisplay};
use formats::{DataFormat, ReferenceFormulation};
use serde::{Deserialize, Serialize};

pub type RcOperator = Rc<Operator>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Operator {
    SourceOp { config: Source },
    JoinOp { config: Join },
    UnionOp, 
    ProjectOp { config: Projection },
    ExtendOp { config: Extend },
    RenameOp { config: Rename },
    SerializerOp { config: Serializer },
    TargetOp { config: Target },
    FragmentOp { config: Fragmenter },
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
            Operator::FragmentOp { config } => {
                ("Fragment Operator".to_string(), config.pretty_string()?)
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

// TODO: Turn Field and Iterator into an Enum since a field itself can be an iterator! 
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    pub alias:                 String,
    pub reference:             Option<String>,
    pub constant:              Option<String>, 
    pub iterator:              Option<String>, 
    pub reference_formulation: ReferenceFormulation,
    pub inner_fields:          Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Iterator {
    pub reference:             Option<String>,
    pub reference_formulation: ReferenceFormulation,
    pub fields:                Vec<Field>,
    pub alias:                 Option<String>,
}

impl Hash for Iterator {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.reference.hash(state);
        self.reference_formulation.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Source {
    #[serde(flatten)]
    pub config:      HashMap<String, String>,
    pub source_type:    IOType,
    pub root_iterator:    Iterator,
}

impl PrettyDisplay for Source {
    fn pretty_string(&self) -> Result<String> {
        let result = format!(
            "type: {:?} \nreference iterator: {:#?} \nconfig: {}
             ",
            self.source_type,
            self.root_iterator,
            serde_json::to_string_pretty(&self.config)?
        );
        Ok(result)
    }
}

impl Hash for Source {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        hash_hashmap(&self.config, state);
        self.source_type.hash(state);
        self.root_iterator.hash(state);
    }
}

/// Enums to denote different types of joins
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum JoinType {
    LeftJoin,
    RightJoin,
    InnerJoin,
    CrossJoin,
    NaturalJoin,
}


/// Type of predicate function used in a θ-join operator
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum PredicateType {
    Greater,
    GEqual,
    Less,
    LEqual,
    Equal,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Join {
    pub left_right_attr_pairs: Vec<(String, String)>,
    pub join_type:             JoinType,
    pub predicate_type:        PredicateType,
}

impl Default for Join {
    fn default() -> Self {
        Self {
            left_right_attr_pairs: Default::default(),
            join_type:             JoinType::InnerJoin,
            predicate_type:        PredicateType::Equal,
        }
    }
}

impl Hash for Join {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.left_right_attr_pairs.hash(state);
        self.join_type.hash(state);
        self.predicate_type.hash(state);
    }
}

impl PrettyDisplay for Join {
    fn pretty_string(&self) -> Result<String> {
        let result = format!(
            "type: {:?}\npredicate_type: {:?}\nattribute_pairs: {}\n",
            self.join_type,
            self.predicate_type,
            serde_json::to_string_pretty(&self.left_right_attr_pairs)?, 
        );

        Ok(result)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Projection {
    pub projection_attributes: HashSet<String>,
}

impl PrettyDisplay for Projection {
    fn pretty_string(&self) -> Result<String> {
        let attributes = self
            .projection_attributes
            .iter()
            .fold(String::new(), |acc, val| acc + val + ", ");

        Ok(format!("Projected attributes: {}", attributes))
    }
}

impl Hash for Projection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for val in self.projection_attributes.iter() {
            val.hash(state);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rename {
    pub alias: Option<String>, 
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

        Ok(format!("Renaming pairs:\n {}{}", alias_string, pairs_string))
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
    IRI
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Function {
    Nop, 
    SimpleConcat{
        inner_function: Option<RcExtendFunction>
    }, 
    Concatenate{
        left_value: RcExtendFunction,
        separator: String, 
        right_value: RcExtendFunction,
    },
    Reference {
        value: String,
    },
    TypedConstant{

        value:String, 
        term_type: TermType, 
    },

    Constant {
        value: String,
    },
    TemplateString {
        value: String,
    },

    Replace{
        replace_map:    HashMap<String, HashSet<String>>,
        inner_function: RcExtendFunction, 
    },

    TemplateFunctionValue{
        template: String, 
        variable_function_pairs: Vec<(String, RcExtendFunction)>, 
    },

    UriEncode {
        inner_function: RcExtendFunction,
    },
    Iri {
        base_iri: Option<String>, 
        inner_function: RcExtendFunction,
    },
    Literal {
        inner_function: RcExtendFunction,
        dtype_function: Option<RcExtendFunction>, 
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
        fno_identifier:   String,
        parameters: HashMap<String, RcExtendFunction>,
    },
    Star {
       // TODO: Implement star function
    }
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

impl Default for Target{
    fn default() -> Self {
        Self { configuration: Default::default(), target_type: IOType::StdOut, data_format: DataFormat::NQuads }
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Fragmenter {
    pub from: String,
    pub to:   Vec<String>,
}

impl Fragmenter {
    pub fn target_fragment_exist(&self, target_fragment: &str) -> bool {
        self.to
            .iter()
            .filter(|frag| *frag == target_fragment)
            .count()
            == 1
    }
}

impl PrettyDisplay for Fragmenter {
    fn pretty_string(&self) -> Result<String> {
        let result = format!(
            "from_fragment: {} \n to_fragments: {}",
            self.from,
            serde_json::to_string(&self.to)?,
        );

        Ok(result)
    }
}
