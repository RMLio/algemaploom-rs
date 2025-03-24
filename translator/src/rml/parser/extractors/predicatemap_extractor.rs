use sophia_api::term::TermKind;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use super::{FromVocab, TermMapExtractor};
use crate::rml::parser::extractors::error::ParseError;
use crate::rml::parser::extractors::Extractor;
use crate::rml::parser::rml_model::term_map::{GraphMap, PredicateMap, TermMapInfo};

impl TermMapExtractor<PredicateMap> for PredicateMap {
    fn create_constant_map(tm_info: TermMapInfo) -> PredicateMap {
        if tm_info.term_type != Some(TermKind::Iri) {
            panic!("Constant-valued PredicateMap has to have an IRI as value");
        }
        PredicateMap {
            tm_info,
            graph_maps: vec![],
        }
    }

    fn create_term_map(
        subj_ref: &RcTerm,
        graph_ref: &FastGraph,
    ) -> super::ExtractorResult<PredicateMap> {
        let mut tm_info = TermMapInfo::extract_self(subj_ref, graph_ref)?;

        tm_info = match tm_info.term_type {
            Some(ttype) if ttype != TermKind::Iri => {
                return Err(ParseError::GenericError(
                    "PredicateMap can only have rr:Iri as rr:termType!"
                        .to_string(),
                ))
            }
            Some(_) => tm_info,
            None => {
                TermMapInfo {
                    term_type: Some(TermKind::Iri),
                    ..tm_info
                }
            }
        };

        let graph_maps =
            GraphMap::extract_many_from_container(graph_ref, subj_ref)?;

        Ok(PredicateMap {
            tm_info,
            graph_maps,
        })
    }

    fn get_const_pred() -> RcTerm {
        vocab::r2rml::PROPERTY::PREDICATE.to_rcterm()
    }

    fn get_map_pred() -> RcTerm {
        vocab::r2rml::PROPERTY::PREDICATEMAP.to_rcterm()
    }

    fn get_term_map_info(&self) -> TermMapInfo {
        self.tm_info.clone()
    }
}

#[cfg(test)]
mod tests {

    use sophia_api::prelude::Any;
    use sophia_api::term::FromTerm;
    use sophia_api::triple::Triple;
    use sophia_api::graph::Graph;
    use super::*;
    use crate::import_test_mods;
    use crate::rml::parser::rml_model::term_map::TermMapType;

    import_test_mods!();

    #[test]
    fn create_const_predicatemap_test() -> ExtractorResult<()> {
        let graph = load_graph!("rml/sample_mapping.ttl")?;
        let pm_const_pred = vocab::r2rml::PROPERTY::PREDICATE.to_rcterm();
        let triples = graph.triples_matching(Any, [pm_const_pred], Any);
        let values = triples.flatten().map(|trip| trip.o().to_owned());
        let pms: Vec<PredicateMap> = values
            .map(|map_const_simp| RcTerm::from_term(map_const_simp))
            .map(|map_const| {
                PredicateMap::extract_constant_term_map(&map_const)
            })
            .collect::<ExtractorResult<_>>()?;

        assert_eq!(pms.len(), 2);

        pms.iter().for_each(|pm| {
            assert_eq!(pm.tm_info.term_map_type, TermMapType::Constant);
            assert_eq!(pm.tm_info.term_type, Some(TermKind::Iri));
        });

        Ok(())
    }
}
