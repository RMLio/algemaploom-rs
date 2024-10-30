mod csvw_source;
mod file_source;
mod rdb_source;

use std::collections::HashMap;

use sophia_api::term::TTerm;
use sophia_inmem::graph::FastGraph;

use self::csvw_source::extract_csvw_source;
use super::error::ParseError;
use super::{Extractor, ExtractorResult, RcTerm};
use crate::extractors::store::get_object;
use crate::extractors::FromVocab;
use crate::rml_model::source_target::{Source, SourceType};

impl Extractor<Source> for Source {
    fn extract_self(
        subject_ref: &RcTerm,
        graph_ref: &FastGraph,
    ) -> ExtractorResult<Source> {
        match subject_ref.kind() {
            sophia_api::term::TermKind::Iri
            | sophia_api::term::TermKind::BlankNode => {
                extract_typed_source(subject_ref, graph_ref)
            }
            sophia_api::term::TermKind::Literal => {
                let mut config = HashMap::new();
                config.insert(
                    "path".to_string(),
                    subject_ref.value().to_string(),
                );
                Ok(Source {
                    source_type: SourceType::FileInput,
                    config,
                })
            }

            _ => {
                Err(ParseError::GenericError(format!(
                    "Variables cannot be parsed as Source {}",
                    subject_ref
                )))
            }
        }
    }
}

fn extract_typed_source(
    subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<Source> {
    let type_pred = vocab::rdf::PROPERTY::TYPE.to_rcterm();
    let source_type = get_object(graph, subject, &type_pred)?;

    let match_result = match source_type {
        sophia_term::Term::Iri(iri) => Ok(iri),
        _ => {
            Err(ParseError::GenericError(
                "Object of predicate 'a' cannot be Literal".to_string(),
            ))
        }
    }?;

    match match_result {
        iri_string if iri_string == vocab::csvw::CLASS::TABLE.to_rcterm() => {
            extract_csvw_source(subject, graph)
        }

        iri_string if iri_string == vocab::d2rq::CLASS::DATABASE.to_rcterm() => {
            rdb_source::extract_rdb_source(subject, graph)
        }

        invalid_iri => {
            Err(ParseError::GenericError(format!(
                "Source type extraction not yet supported {:#?}",
                invalid_iri
            )))
        }
    }
}
