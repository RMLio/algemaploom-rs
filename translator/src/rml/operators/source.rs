use std::collections::HashSet;

use operator::formats::ReferenceFormulation;
use operator::{Field, Iterator, Source};
use crate::rml::parser::extractors::FromVocab;
use crate::rml::parser::rml_model::source_target::SourceType;
use crate::rml::parser::rml_model::TriplesMap;
use sophia_api::term::Term;
use vocab::ToString;

use crate::rml::util::extract_references_in_tm;
use crate::OperatorTranslator;
#[derive(Debug, Clone)]
pub struct SourceOpTranslator<'a> {
    pub tm:        &'a TriplesMap,
    pub other_tms: Vec<&'a TriplesMap>,
}

impl<'a> OperatorTranslator<Source> for SourceOpTranslator<'a> {
    fn translate(&self) -> Source {
        let tm = self.tm;
        let reference_formulation =
            match &tm.logical_source.reference_formulation {
                iri if *iri == vocab::query::CLASS::CSV.to_rcterm() => {
                    ReferenceFormulation::CSVRows
                }
                iri if *iri == vocab::query::CLASS::JSONPATH.to_rcterm() => {
                    ReferenceFormulation::JSONPath
                }
                iri if *iri == vocab::query::CLASS::XPATH.to_rcterm() => {
                    ReferenceFormulation::XMLPath
                }
                _ => ReferenceFormulation::CSVRows,
            };

        let mut fields = Vec::new();
        if reference_formulation != ReferenceFormulation::CSVRows {
            let references = extract_references_in_tm(tm, &self.other_tms);

            fields.extend(references.into_iter().map(|reference| {
                Field {
                    alias:                 reference.clone(),
                    reference:             reference.clone(),
                    reference_formulation: reference_formulation.clone(),
                    inner_fields:          vec![],
                }
            }));
        }

        let root_iterator = Iterator {
            reference: tm.logical_source.iterator.clone(),
            reference_formulation,
            fields,
            alias: None,
        };

        let config = tm.logical_source.source.config.clone();
        let source_type = match tm.logical_source.source.source_type {
            SourceType::CSVW => operator::IOType::File,
            SourceType::FileInput => operator::IOType::File,
            SourceType::RDB => operator::IOType::RDB,
            SourceType::TCP => operator::IOType::Websocket,
            SourceType::Kafka => operator::IOType::Kafka,
        };

        Source {
            config,
            source_type,
            root_iterator,
        }
    }
}
