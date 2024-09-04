use operator::formats::ReferenceFormulation;
use operator::{Field, Iterator, Source};
use rml_interpreter::rml_model::source_target::SourceType;
use rml_interpreter::rml_model::TriplesMap;
use sophia_api::term::TTerm;
use vocab::ToString;

use crate::rmlalgebra::util::extract_references_in_tm;
use crate::OperatorTranslator;
#[derive(Debug, Clone)]
pub struct SourceOpTranslator<'a> {
    pub tm: &'a TriplesMap,
}

impl<'a> OperatorTranslator<Source> for SourceOpTranslator<'a> {
    fn translate(&self) -> Source {
        let tm = self.tm;
        let reference_formulation =
            match tm.logical_source.reference_formulation.value().to_string() {
                iri if iri == vocab::query::CLASS::JSONPATH.to_string() => {
                    ReferenceFormulation::JSONPath
                }
                iri if iri == vocab::query::CLASS::XPATH.to_string() => {
                    ReferenceFormulation::XMLPath
                }
                _ => ReferenceFormulation::CSVRows,
            };

        let mut fields = Vec::new();
        if reference_formulation != ReferenceFormulation::CSVRows {
            let references = extract_references_in_tm(tm);

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
        };

        Source {
            config,
            source_type,
            root_iterator,
        }
    }
}
