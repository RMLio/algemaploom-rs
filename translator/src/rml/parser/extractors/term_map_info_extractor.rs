use std::collections::HashSet;

use sophia_api::graph::Graph;
use sophia_api::prelude::Any;
use sophia_api::term::{FromTerm, Term, TermKind};
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use super::error::ParseError;
use super::store::{get_object, get_objects};
use super::{rcterm_to_string, Extractor, ExtractorResult, FromVocab};
use crate::rml::parser::rml_model::source_target::LogicalTarget;
use crate::rml::parser::rml_model::term_map::{
    FunctionMap, TermMapInfo, TermMapType,
};

fn extract_term_map_type_value(
    subject_ref: &RcTerm,
    graph_ref: &FastGraph,
) -> ExtractorResult<(TermMapType, RcTerm)> {
    //function-map
    let fno_pred: RcTerm = vocab::fnml::PROPERTY::FUNCTION_VALUE.to_rcterm();

    //template-map
    let temp_pred: RcTerm = vocab::r2rml::PROPERTY::TEMPLATE.to_rcterm();

    //constant-map
    let const_pred: RcTerm = vocab::r2rml::PROPERTY::CONSTANT.to_rcterm();

    //reference-map
    let ref_pred: RcTerm = vocab::rml::PROPERTY::REFERENCE.to_rcterm();
    let col_pred: RcTerm = vocab::r2rml::PROPERTY::COLUMN.to_rcterm();

    let pred_query = [&ref_pred, &col_pred, &const_pred, &temp_pred, &fno_pred];

    let mut results_query: Vec<_> = graph_ref
        .triples_matching([subject_ref], pred_query, Any)
        .filter_map(|trip| trip.ok())
        .collect();

    if results_query.len() > 1 {
        return Err(ParseError::GenericError(
                    "More than one occurences of rr:template, rml:reference, rr:constant, fnml:functionValue or rr:column".to_string()
                    ).into());
    }

    let trip = results_query
        .pop()
        .ok_or(ParseError::GenericError("Term map doesn't have rr:constant, rr:template, rr:reference, fnml:functionValue nor rr:column.".to_string()))?;

    let fetched_pred = trip.p();

    let term_map_type_res = match fetched_pred {
        ref_map if *ref_map == ref_pred || *ref_map == col_pred => {
            Ok(TermMapType::Reference)
        }
        const_map if *const_map == const_pred => Ok(TermMapType::Constant),
        temp_map if *temp_map == temp_pred => Ok(TermMapType::Template),
        func_map if *func_map == fno_pred => Ok(TermMapType::Function),
        leftover => {
            Err(ParseError::GenericError(format!(
                "Term map type not handled {:?}",
                leftover
            )))
        }
    };

    let term_value = RcTerm::from_term(trip.o());

    Ok(term_map_type_res.map(|map_type| (map_type, term_value))?)
}

impl Extractor<TermMapInfo> for TermMapInfo {
    fn extract_self(
        subj_ref: &RcTerm,
        graph_ref: &FastGraph,
    ) -> super::ExtractorResult<TermMapInfo> {
        let (term_map_type, term_value) =
            extract_term_map_type_value(subj_ref, graph_ref)?;

        let term_type_pred = vocab::r2rml::PROPERTY::TERMTYPE.to_rcterm();

        let mut term_type = None;

        //Explicit term type casting trough rr:termtype predicate
        if let Ok(term_type_soph) =
            get_object(graph_ref, subj_ref, &term_type_pred)
        {
            let lit_class = vocab::r2rml::CLASS::LITERAL.to_rcterm();
            let iri_class = vocab::r2rml::CLASS::IRI.to_rcterm();
            let bnode_class = vocab::r2rml::CLASS::BLANKNODE.to_rcterm();

            term_type = match term_type_soph {
                iri if iri == iri_class => Some(TermKind::Iri),
                iri if iri == bnode_class => Some(TermKind::BlankNode),
                iri if iri == lit_class => Some(TermKind::Literal),
                _ => None,
            };
        }

        //Implicit term type derivation for constant-valued term maps
        if term_map_type == TermMapType::Constant {
            term_type = match term_value {
                RcTerm::Iri(_) => Some(TermKind::Iri),
                RcTerm::BlankNode(_) => Some(TermKind::BlankNode),
                RcTerm::Literal(_) => Some(TermKind::Literal),
                _ => None,
            };
        }

        let logical_target_iris = get_objects(
            graph_ref,
            subj_ref,
            &vocab::rml::PROPERTY::LOGICALTARGET.to_rcterm(),
        );

        let mut logical_targets: HashSet<LogicalTarget> = logical_target_iris
            .into_iter()
            .flat_map(|log_targ_iri| {
                LogicalTarget::extract_self(&log_targ_iri, graph_ref)
            })
            .collect();

        // Add a default logical target if it is empty
        if logical_targets.is_empty() {
            logical_targets.insert(LogicalTarget::default());
        }

        let identifier = rcterm_to_string(subj_ref);
        let mut fun_map_opt = None;
        if term_map_type == TermMapType::Function {
            fun_map_opt =
                FunctionMap::extract_self(&term_value, graph_ref).ok();
        }

        Ok(TermMapInfo {
            identifier,
            logical_targets,
            term_map_type,
            term_value,
            term_type,
            fun_map_opt,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    use sophia_api::graph::Graph;
    use sophia_api::term::Term;
    use sophia_api::triple::Triple;

    use super::*;
    use crate::rml::parser::extractors::io::load_graph_bread;
    use crate::rml::parser::extractors::ExtractorResult;
    use crate::rml::parser::rml_model::term_map::TermMapType;
    use crate::{load_graph, test_case};

    #[test]
    fn term_map_info_extraction_test() -> ExtractorResult<()> {
        let graph: FastGraph = load_graph!("rml/sample_mapping.ttl")?;
        let sub_pred = vocab::r2rml::PROPERTY::SUBJECTMAP.to_rcterm();
        let triple = graph.triples_matching(Any,[sub_pred], Any).next().unwrap().unwrap();
        let sub_ref = RcTerm::from_term(triple.o());

        let tm_info = TermMapInfo::extract_self(&sub_ref, &graph)?;

        assert!(tm_info.term_type.is_none());
        assert!(tm_info.term_map_type == TermMapType::Template);
        println!("{:?}", tm_info);
        assert!(rcterm_to_string(&tm_info.term_value) == "example/{brand}");

        Ok(())
    }
}
