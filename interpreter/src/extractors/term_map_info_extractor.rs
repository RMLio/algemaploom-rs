

use sophia_api::graph::Graph;
use sophia_api::term::{TermKind};
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;
use sophia_term::matcher::ANY;
use sophia_term::RcTerm;

use super::error::ParseError;
use super::store::{get_object, get_objects};
use super::{Extractor, ExtractorResult, FromVocab};
use crate::rml_model::source_target::LogicalTarget;
use crate::rml_model::term_map::{TermMapInfo, TermMapType};
use crate::TermString;

fn extract_term_map_type_value(
    subject_ref: &RcTerm,
    graph_ref: &FastGraph,
) -> ExtractorResult<(TermMapType, TermString)> {
    //template-map
    let temp_pred: RcTerm = vocab::r2rml::PROPERTY::TEMPLATE.to_term();

    //constant-map
    let const_pred: RcTerm = vocab::r2rml::PROPERTY::CONSTANT.to_term();

    //reference-map
    let ref_pred: RcTerm = vocab::rml::PROPERTY::REFERENCE.to_term();
    let col_pred: RcTerm = vocab::r2rml::PROPERTY::COLUMN.to_term();

    let pred_query = &[&ref_pred, &col_pred, &const_pred, &temp_pred];

    let mut results_query: Vec<_> = graph_ref
        .triples_matching(subject_ref, pred_query, &ANY)
        .filter_map(|trip| trip.ok())
        .collect();

    if results_query.len() > 1 {
        return Err(ParseError::GenericError(
                    "More than one occurences of rr:template, rml:reference, rr:constant, or rr:column".to_string()
                    ));
    }

    let trip = results_query.pop().ok_or(ParseError::GenericError("Term map doesn't have rr:constant, rr:template, rr:reference nor rr:column.".to_string()))?;
    let fetched_pred = trip.p();

    let term_map_type_res = match fetched_pred {
        ref_map if *ref_map == ref_pred || *ref_map == col_pred => {
            Ok(TermMapType::Reference)
        }
        const_map if *const_map == const_pred => Ok(TermMapType::Constant),
        temp_map if *temp_map == temp_pred => Ok(TermMapType::Template),
        _ => Err(ParseError::Infallible),
    };

    let term_value = trip.o().to_owned().map(|i| i.to_string());

    term_map_type_res.map(|map_type| (map_type, term_value))
}

impl Extractor<TermMapInfo> for TermMapInfo {
    // TODO: Logical targets parsing <29-08-23, Sitt Min Oo> //
    fn extract_self(
        subj_ref: &RcTerm,
        graph_ref: &FastGraph,
    ) -> super::ExtractorResult<TermMapInfo> {
        let (term_map_type, term_value) =
            extract_term_map_type_value(subj_ref, graph_ref)?;

        let term_type_pred = vocab::r2rml::PROPERTY::TERMTYPE.to_term();

        let mut term_type = None;

        if let Ok(term_type_soph) =
            get_object(graph_ref, subj_ref, &term_type_pred)
        {
            let lit_class = vocab::r2rml::CLASS::LITERAL.to_term();
            let iri_class = vocab::r2rml::CLASS::IRI.to_term();
            let bnode_class = vocab::r2rml::CLASS::BLANKNODE.to_term();

            term_type = match term_type_soph {
                sophia_term::Term::Iri(iri) if iri == iri_class => {
                    Some(TermKind::Iri)
                }
                sophia_term::Term::Iri(iri) if iri == bnode_class => {
                    Some(TermKind::BlankNode)
                }
                sophia_term::Term::Iri(iri) if iri == lit_class => {
                    Some(TermKind::Literal)
                }
                _ => None,
            };
        }

        let logical_target_iris = get_objects(
            graph_ref,
            subj_ref,
            &vocab::rml::PROPERTY::LOGICALTARGET.to_term(),
        );
        let logical_targets =
            logical_target_iris.into_iter().flat_map(|log_targ_iri| {
                LogicalTarget::extract_self(&log_targ_iri, graph_ref)
            }).collect();

        let identifier = subj_ref.to_string();

        Ok(TermMapInfo {
            identifier,
            logical_targets,
            term_map_type,
            term_value,
            term_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    use sophia_api::graph::Graph;
    use sophia_api::triple::Triple;

    use super::*;
    use crate::extractors::io::load_graph_bread;
    use crate::extractors::ExtractorResult;
    use crate::rml_model::term_map::TermMapType;
    use crate::{load_graph, test_case};

    #[test]
    fn term_map_info_extraction_test() -> ExtractorResult<()> {
        let graph: FastGraph = load_graph!("sample_mapping.ttl")?;
        let sub_pred = vocab::r2rml::PROPERTY::SUBJECTMAP.to_term();
        let triple = graph.triples_with_p(&sub_pred).next().unwrap().unwrap();
        let sub_ref = triple.o();

        let tm_info = TermMapInfo::extract_self(sub_ref, &graph)?;

        assert!(tm_info.term_type.is_none());
        assert!(tm_info.term_map_type == TermMapType::Template);
        println!("{:?}", tm_info);
        assert!(
            tm_info.term_value.value() == "http://airport.example.com/{id}"
        );

        Ok(())
    }
}
