use std::collections::HashSet;
use std::hash::Hash;

use lazy_static::lazy_static;
use regex::Regex;
use sophia_api::term::{TTerm, TermKind};
use sophia_term::{RcTerm, Term};

use super::join::JoinCondition;
use super::source_target::LogicalTarget;
use crate::rml::parser::{IriString, TermString};

lazy_static! {
    static ref TEMPLATE_REGEX: Regex = Regex::new(r"\{([^\{\}]+)\}").unwrap();
}
fn prefix_attributes_from_template(template: &str, prefix: &str) -> String {
    let sanitized = template.replace("\\{", "\\(").replace("\\}", "\\)");
    TEMPLATE_REGEX
        .replace_all(&sanitized, format!("{{{}_$1}}", prefix))
        .replace("\\(", "\\{")
        .replace("\\)", "\\}")
}

fn get_attributes_from_template(template: &str) -> Vec<String> {
    let sanitized = template.replace("\\{", "").replace("\\}", "");
    let captured = TEMPLATE_REGEX.captures_iter(&sanitized);
    captured
        .filter_map(|cap| cap.get(1).map(|c| c.as_str().to_owned()))
        .collect()
}
#[derive(Debug, Clone)]
pub struct TermMapInfo {
    pub identifier:      String,
    pub logical_targets: HashSet<LogicalTarget>,
    pub term_map_type:   TermMapType,
    pub term_value:      TermString,
    pub term_type:       Option<TermKind>,
    pub fun_map_opt:     Option<FunctionMap>,
}

impl Hash for TermMapInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.term_value.hash(state);
        self.term_type.hash(state);
        self.term_map_type.hash(state);
    }
}

impl Default for TermMapInfo {
    fn default() -> Self {
        let mut logical_targets = HashSet::new();

        logical_targets.insert(LogicalTarget::default());
        Self {
            identifier: Default::default(),
            term_map_type: TermMapType::Constant,
            term_value: Term::new_bnode("qsdkfldsfj").unwrap(),
            term_type: Default::default(),
            fun_map_opt: Default::default(),
            logical_targets,
        }
    }
}

impl TermMapInfo {
    pub fn prefix_attributes(&mut self, prefix: &str) {
        let mut term_value = self.term_value.clone();
        term_value = match self.term_map_type {
            TermMapType::Constant => term_value,
            TermMapType::Reference => {
                term_value.map(|val| format!("{}_{}", prefix, val))
            }
            TermMapType::Template => {
                term_value
                    .map(|val| prefix_attributes_from_template(&val, prefix))
            }
            TermMapType::Function => {
                self.fun_map_opt
                    .as_mut()
                    .unwrap()
                    .param_om_pairs
                    .iter_mut()
                    .for_each(|(_, om)| om.tm_info.prefix_attributes(prefix));
                term_value
            }
        };

        self.term_value = term_value;
    }

    pub fn get_attributes(&self) -> HashSet<String> {
        let tm_info = self;
        let value = tm_info.term_value.value().to_string();
        match tm_info.term_map_type {
            TermMapType::Constant => HashSet::new(),
            TermMapType::Reference => vec![value].into_iter().collect(),
            TermMapType::Template => {
                get_attributes_from_template(&value).into_iter().collect()
            }
            TermMapType::Function => {
                tm_info
                    .fun_map_opt
                    .as_ref()
                    .unwrap()
                    .param_om_pairs
                    .iter()
                    .flat_map(|(_, om)| om.tm_info.get_attributes())
                    .collect()
            }
        }
    }
    pub fn from_constant_value(const_value: RcTerm) -> TermMapInfo {
        let identifier = match const_value.clone() {
            Term::Iri(iri) => Term::Iri(iri.map(|i| i.to_string())),
            Term::BNode(bnode) => Term::BNode(bnode.map(|i| i.to_string())),
            Term::Literal(lit) => {
                Term::new_bnode(format!(
                    "{}-{}",
                    lit.txt(),
                    uuid::Uuid::new_v4()
                ))
                .unwrap()
            }
            Term::Variable(_) => {
                panic!("Variable not supported yet!")
            }
        }
        .to_string();

        let term_type = Some(const_value.kind());

        TermMapInfo {
            identifier,
            term_map_type: TermMapType::Constant,
            term_value: const_value.map(|i| i.to_string()),
            term_type,
            fun_map_opt: None,
            ..Default::default()
        }
    }
}
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum TermMapType {
    Constant,
    Reference,
    Template,
    Function,
}

#[derive(Debug, Clone, Hash)]
pub struct SubjectMap {
    pub tm_info:    TermMapInfo,
    pub classes:    Vec<IriString>,
    pub graph_maps: Vec<GraphMap>,
}


#[derive(Debug, Clone, Hash)]
pub struct PredicateMap {
    pub tm_info:    TermMapInfo,
    pub graph_maps: Vec<GraphMap>,
}

#[derive(Debug, Clone, Hash)]
pub struct ObjectMap {
    pub tm_info:        TermMapInfo,
    pub parent_tm:      Option<IriString>,
    pub join_condition: Option<JoinCondition>,
    pub data_type:      Option<IriString>,
    pub language:       Option<String>,
    pub graph_maps:     Vec<GraphMap>,
}

#[derive(Debug, Clone)]
pub struct FunctionMap {
    pub identifier:     String,
    pub function_iri:   String,
    pub param_om_pairs: Vec<(String, ObjectMap)>,
}

#[derive(Debug, Clone, Hash)]
pub struct GraphMap {
    pub tm_info: TermMapInfo,
}
