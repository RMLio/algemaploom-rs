use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

use operator::formats::DataFormat;
use operator::{IOType, Target};
use sophia_api::prelude::Iri;
use sophia_api::term::{FromTerm, Term};
use sophia_term::RcTerm;
use vocab::ToString;

use crate::rml::parser::extractors::{rcterm_to_string, FromVocab};

#[derive(Debug, Clone, Eq)]
pub struct LogicalSource {
    pub identifier:            String,
    pub iterator:              Option<String>,
    pub source:                Source,
    pub reference_formulation: RcTerm,
}
impl PartialEq for LogicalSource {
    fn eq(&self, other: &Self) -> bool {
        self.iterator == other.iterator
            && self.source == other.source
            && self.reference_formulation == other.reference_formulation
    }
}

#[derive(Debug, Clone)]
pub enum FileMode {
    Append,
    Overwrite,
}

#[derive(Debug, Clone)]
pub enum Output {
    FileOutput { path: String, mode: FileMode },
}

pub fn default_file_output(path: String) -> Output {
    Output::FileOutput {
        path,
        mode: FileMode::Overwrite,
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LdesInformation{
    pub identifier: String, 
    pub ldes_eventstream: HashMap<String, String>,
    pub ldes_base_iri: RcTerm,
    pub ldes_generate_immutable_iri: bool
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalTarget {
    pub identifier:    String,
    pub compression:   Option<RcTerm>,
    pub serialization: RcTerm,
    pub output_type:   IOType,
    pub ldes: Option<LdesInformation>,
    pub config:        HashMap<String, String>,
}

impl Default for LogicalTarget {
    fn default() -> Self {
        Self {
            identifier:    String::from("default"),
            compression:   Default::default(),
            serialization: RcTerm::from_term(Iri::new_unchecked(
                vocab::formats::CLASS::NQUADS.to_string(),
            )),
            output_type:   Default::default(),
            ldes:          None,
            config:        Default::default(),
        }
    }
}

impl Hash for LogicalTarget {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
    }
}

fn serialization_to_dataformat(serialization: &RcTerm) -> DataFormat {
    match serialization.to_owned() {
        ser_iri_string
            if ser_iri_string == vocab::formats::CLASS::TURTLE.to_rcterm() =>
        {
            DataFormat::TTL
        }
        ser_iri_string
            if ser_iri_string
                == vocab::formats::CLASS::NTRIPLES.to_rcterm() =>
        {
            DataFormat::NTriples
        }
        ser_iri_string
            if ser_iri_string == vocab::formats::CLASS::JSONLD.to_rcterm() =>
        {
            DataFormat::JSONLD
        }
        ser_iri_string
            if ser_iri_string == vocab::formats::CLASS::NQUADS.to_rcterm() =>
        {
            DataFormat::NQuads
        }

        _ => DataFormat::NQuads,
    }
}

impl From<&LogicalTarget> for operator::Target {
    fn from(val: &LogicalTarget) -> Self {
        let mut configuration = HashMap::new();

        if let Some(comp_iri) = val.compression.as_ref() {
            configuration.insert(
                "compresssion".to_string(),
                rcterm_to_string(comp_iri),
            );
        }
        configuration.extend(val.config.clone());

        let data_format = serialization_to_dataformat(&val.serialization);
        Target {
            configuration,
            data_format,
            target_type: val.output_type.clone(),
        }
    }
}

impl From<LogicalTarget> for operator::Target {
    fn from(val: LogicalTarget) -> Self {
        (&val).into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    pub source_type: SourceType,
    pub config:      HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceType {
    CSVW,
    FileInput,
    RDB,
    Kafka,
    TCP,
    HTML,
}

impl Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceType::CSVW => write!(f, "CSVW"),
            SourceType::FileInput => write!(f, "FileInput"),
            SourceType::RDB => write!(f, "RDataBase"),
            SourceType::Kafka => write!(f, "Kafka Stream"),
            SourceType::TCP => write!(f, "Websocket"),
            SourceType::HTML => write!(f, "HTML"),
        }
    }
}

impl From<Source> for HashMap<String, String> {
    fn from(val: Source) -> Self {
        let mut map = HashMap::new();

        map.extend(val.config);
        map.insert("type".to_string(), format!("{}", val.source_type));
        map
    }
}

fn source_config_map(ls: &LogicalSource) -> HashMap<String, String> {
    let mut map = HashMap::new();

    map.insert("identifier".to_string(), ls.identifier.to_string());

    if let Some(iter) = &ls.iterator {
        map.insert("iterator".to_string(), iter.to_owned());
    }

    let source_map: HashMap<String, String> = ls.source.clone().into();

    map.extend(source_map);

    map
}
