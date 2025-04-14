use sophia_api::term::Term;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use super::{Extractor, TermMapExtractor};
use crate::new_rml::extractors::FromVocab;
use crate::new_rml::rml_model::v2::core::expression_map::term_map::{
    PredicateMap, TermMap,
};

impl TermMapExtractor<PredicateMap> for PredicateMap {
    fn create_constant_map(term_map: TermMap) -> PredicateMap {
        if !term_map.is_iri_term_type() {
            panic!("Constant-valued PredicateMap has to have an IRI as value");
        }
        PredicateMap { term_map }
    }

    fn create_term_map<TS>(
        subj_ref: TS,
        graph_ref: &FastGraph,
    ) -> super::ExtractorResult<PredicateMap>
    where
        TS: Term + Clone,
    {
        let mut term_map = TermMap::extract_self(subj_ref, graph_ref)?;
        Ok(PredicateMap { term_map })
    }

    fn get_const_preds() -> Vec<RcTerm> {
        vec![
            vocab::r2rml::PROPERTY::PREDICATE.to_rcterm(),
            vocab::rml_core::PROPERTY::PREDICATE.to_rcterm(),
        ]
    }

    fn get_map_preds() -> Vec<RcTerm> {
        vec![
            vocab::r2rml::PROPERTY::PREDICATEMAP.to_rcterm(),
            vocab::rml_core::PROPERTY::PREDICATE_MAP.to_rcterm(),
        ]
    }
}

#[cfg(test)]
mod tests {

    use sophia_api::graph::Graph;
    use sophia_api::prelude::Any;
    use sophia_api::triple::Triple;

    use super::*;
    use crate::new_rml::error::NewRMLTranslationError;
    use crate::new_rml::extractors::error::ParseError;
    use crate::import_test_mods;
    use crate::new_rml::rml_model::v2::core::expression_map::ExpressionMapTypeEnum;

    import_test_mods!(new_rml);

    #[test]
    fn create_const_predicatemap_test() -> ExtractorResult<()> {
        let graph = load_graph!("rml/sample_mapping.ttl")?;
        let pm_const_pred = vocab::r2rml::PROPERTY::PREDICATE.to_rcterm();
        let triples = graph.triples_matching(Any, [pm_const_pred], Any);
        let values = triples.flatten().map(|trip| trip.o().to_owned());
        let pms: Vec<PredicateMap> = values
            .map(|map_const| {
                PredicateMap::extract_constant_term_map(&map_const)
            })
            .collect::<ExtractorResult<_>>()?;

        assert_eq!(pms.len(), 2);

        let _ = pms.iter().try_for_each(|pm| {
            assert_eq!(
                pm.term_map.expression.get_map_type_enum()?,
                ExpressionMapTypeEnum::Constant
            );
            assert!(pm.term_map.is_iri_term_type());
            Ok::<(), NewRMLTranslationError>(())
        });

        Ok(())
    }
}
