use operator::formats::ReferenceFormulation;
use operator::{Field, Iterator, Source};

use crate::parser::extractors::FromVocab;
use crate::parser::rml_model::source_target::SourceType;
use crate::parser::rml_model::TriplesMap;
use crate::util::extract_references_in_tm;
use translator_api::OperatorTranslator;
#[derive(Debug, Clone)]
pub struct SourceOpTranslator<'a> {
    pub tm:        &'a TriplesMap,
    pub other_tms: Vec<&'a TriplesMap>,
}

impl<'a> OperatorTranslator<Source> for SourceOpTranslator<'a> {
    fn translate(&self) -> Source {
        let tm = self.tm;
        log::debug!("Translating source operator for triples map {:#?}", tm);
        let reference_formulation =
            match &tm.logical_source.reference_formulation {
                iri if *iri == vocab::query::class::CSV.to_rcterm() => {
                    ReferenceFormulation::CSVRows
                }
                iri if *iri == vocab::query::class::JSONPATH.to_rcterm() => {
                    ReferenceFormulation::JSONPath
                }
                iri if *iri == vocab::query::class::XPATH.to_rcterm() => {
                    ReferenceFormulation::XMLPath
                }
                _ => ReferenceFormulation::CSVRows,
            };

        log::debug!("Reference formulation is {:?}", reference_formulation);
        let mut fields = Vec::new();
        let references = extract_references_in_tm(tm, &self.other_tms);

        fields.extend(references.into_iter().map(|reference| {
            Field {
                absolute_path:         Some(reference.clone()),
                alias:                 reference.clone(),
                constant:              None,
                iterator:              None,
                reference:             Some(reference.clone()),
                reference_formulation: reference_formulation.clone(),
                inner_fields:          vec![],
            }
        }));

        log::debug!("RML fields for the source: {:#?}", fields);
        let root_iterator = Iterator {
            reference: tm.logical_source.iterator.clone(),
            reference_formulation,
            fields,
            alias: None,
        };

        // keep old RML behavior: do not use access, leave it empty
        let config = tm.logical_source.source.config.clone();
        let source_type = match tm.logical_source.source.source_type {
            SourceType::CSVW => operator::IOType::File,
            SourceType::FileInput => operator::IOType::File,
            SourceType::RDB => operator::IOType::RDB,
            SourceType::TCP => operator::IOType::Websocket,
            SourceType::Kafka => operator::IOType::Kafka,
            SourceType::HTML => operator::IOType::File,
        };

        Source {
            config,
            access: Default::default(),
            source_type,
            root_iterator,
        }
    }
}
