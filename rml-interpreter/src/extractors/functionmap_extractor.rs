use sophia_api::term::TTerm;

use super::store::get_objects;
use super::{Extractor, FromVocab};
use crate::rml_model::term_map::FunctionMap;
use crate::rml_model::PredicateObjectMap;

impl Extractor<FunctionMap> for FunctionMap {
    fn extract_self(
        subject_ref: &sophia_term::RcTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<FunctionMap> {
        let pom_pred = vocab::r2rml::PROPERTY::PREDICATEOBJECTMAP.to_term();

        let po_maps = get_objects(graph_ref, subject_ref, &pom_pred)
            .into_iter()
            .filter_map(|pom_subj| {
                PredicateObjectMap::extract_self(&pom_subj, graph_ref).ok()
            });

        let executes_pred_iri = vocab::fno::PROPERTY::EXECUTES.to_term();
        let (execute_poms, params_poms): (Vec<_>, Vec<_>) =
            po_maps.partition(|pom| {
                pom.predicate_maps
                    .iter()
                    .filter(|pm| pm.tm_info.term_value == executes_pred_iri)
                    .count()
                    == 1
            });

        let function_iri = execute_poms
            .into_iter()
            .flat_map(|pom| pom.object_maps)
            .map(|om| om.tm_info.term_value.value().to_string())
            .nth(0)
            .unwrap();

        let param_om_pairs: Vec<_> = params_poms
            .into_iter()
            .map(|mut pom| {
                (
                    pom.predicate_maps.pop().unwrap(),
                    pom.object_maps.pop().unwrap(),
                )
            })
            .map(|(pm, om)| (pm.tm_info.term_value.value().to_string(), om))
            .collect();

        Ok(FunctionMap {
            identifier: subject_ref.value().to_string(),
            function_iri,
            param_om_pairs,
        })
    }
}
#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    use sophia_api::graph::Graph;
    use sophia_api::triple::Triple;
    use sophia_inmem::graph::FastGraph;
    use sophia_term::literal::Literal;
    use sophia_term::Term;

    use super::*;
    use crate::extractors::io::load_graph_bread;
    use crate::extractors::ExtractorResult;
    use crate::rml_model::source_target::SourceType;
    use crate::rml_model::term_map::{ObjectMap, TermMapInfo};
    use crate::{load_graph, test_case};

    #[test]
    fn fno_function_test() -> ExtractorResult<()> {
        let graph: FastGraph = load_graph!("function_mapping.ttl")?;

        // Function map IRI extraction
        let predicate_object_map_pred =
            vocab::fnml::PROPERTY::FUNCTION_VALUE.to_term();
        let predicate_object_map_triple = graph
            .triples_with_p(&predicate_object_map_pred)
            .next()
            .unwrap()?;
        let function_map_ref = predicate_object_map_triple.o();

        let function_map = FunctionMap::extract_self(function_map_ref, &graph)?;
        assert_eq!(
            function_map.function_iri,
            "http://users.ugent.be/~bjdmeest/function/grel.ttl#toUpperCase"
        );
        let param_om_pair =
            function_map.param_om_pairs.into_iter().next().unwrap();

        let expected_term_info = TermMapInfo {
            identifier:      "".to_string(),
            logical_targets: HashSet::new(),
            term_map_type:   crate::rml_model::term_map::TermMapType::Reference,
            term_value:      Term::new_literal_dt(
                "Name",
                vocab::xsd::TYPE::XSD_STRING.to_term(),
            )?,
            term_type:       Some(sophia_api::term::TermKind::Literal),
            fun_map_opt:     None,
        };

        assert_eq!(
            param_om_pair.0,
            "http://users.ugent.be/~bjdmeest/function/grel.ttl#valueParameter"
        );

        let generated_om = param_om_pair.1; 

        assert_eq!(generated_om.tm_info.term_value, expected_term_info.term_value); 
        assert_eq!(generated_om.tm_info.term_map_type, expected_term_info.term_map_type); 
        assert_eq!(generated_om.tm_info.term_type, expected_term_info.term_type); 
        Ok(())
    }
}
