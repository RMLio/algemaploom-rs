mod fields;
mod kind;

use std::collections::HashMap;

use operator::formats::{self, ReferenceFormulation};
use operator::Iterator;
use sophia_inmem::graph::FastGraph;
use sophia_term::{ArcTerm, RcTerm};

use super::error::TranslationError;
use super::OperatorTranslator;
use crate::new_rml::error::NewRMLTranslationResult;
use crate::new_rml::extractors::store::get_object;
use crate::new_rml::extractors::{
    stringify_rcterm, ExtractorResult, FromVocab,
};
use crate::new_rml::rml_model::v2::core::{
    AbstractLogicalSource, AbstractSourceEnum,
};
use crate::new_rml::rml_model::v2::io::source::{
    LogicalSource, Source, SourceKind,
};
use crate::new_rml::translator::source::kind::{
    kafka_source, rdb_source, tcp_source,
};

mod iterator;

pub fn extract_parse_config(
    dialect_subject: &RcTerm,
    graph: &FastGraph,
    predicates: &[(String, ArcTerm)],
) -> ExtractorResult<HashMap<String, String>> {
    let mut result = HashMap::new();
    let _ = predicates.iter().try_for_each(
        |(key, config_pred)| -> ExtractorResult<()> {
            // Retrieve the config value for the subject-predicate pair
            let config_val = get_object(graph, dialect_subject, config_pred);

            if let Ok(val) = config_val {
                result.insert(key.to_string(), stringify_rcterm(val).unwrap());
            }

            Ok(())
        },
    );

    Ok(result)
}

#[derive(Debug, Clone)]
pub struct AbstractLogicalSourceTranslator {}

fn extract_source_specific_config(
    subject_ref: &RcTerm,
    source: &Source,
) -> ExtractorResult<HashMap<String, String>> {
    let kind = &source.kind;
    if kind.type_iri == vocab::rmls::CLASS::KAFKASTREAM.to_rcterm() {
        Ok(kafka_source::extract_kafka_source(
            subject_ref,
            &kind.metadata,
        )?)
    } else if kind.type_iri == vocab::rmls::CLASS::TCPSOCKETSTREAM.to_rcterm() {
        Ok(tcp_source::extract_tcp_source(subject_ref, &kind.metadata)?)
    } else if kind.type_iri == vocab::d2rq::CLASS::DATABASE.to_rcterm() {
        Ok(rdb_source::extract_rdb_source(subject_ref, &kind.metadata)?)
    } else {
        Err(TranslationError::SourceError(format!(
            "Cannot generate config hash maps for the given source : {:?}",
            source
        ))
        .into())
    }
}

impl OperatorTranslator for AbstractLogicalSourceTranslator {
    type Input = AbstractLogicalSource;

    type Output = operator::Source;

    fn translate(
        abs_ls: &Self::Input,
    ) -> NewRMLTranslationResult<Self::Output> {
        let source = abs_ls.get_source();
        let source_kind_ref = &source.kind;
        let mut config = HashMap::new();

        if let Some(encoding) = &source.encoding {
            config.insert(
                "encoding".to_string(),
                stringify_rcterm(encoding).unwrap(),
            );
        }

        if let Some(compression) = &source.compression {
            config.insert(
                "compression".to_string(),
                stringify_rcterm(compression).unwrap(),
            );
        }

        if !source.nullable_vec.is_empty() {
            config.insert(
                "nullable_vec".to_string(),
                source.nullable_vec.join(","),
            );
        }

        let source_kind_config = extract_source_specific_config(
            &abs_ls.get_identifier(),
            abs_ls.get_source(),
        )?;

        config.extend(source_kind_config);

        let root_iterator = iterator::IteratorTranslator::translate(abs_ls)?;

        Ok(operator::Source {
            config,
            source_type: source_kind_ref.try_into()?,
            root_iterator,
        })
    }
}
