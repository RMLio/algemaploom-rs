use sophia_api::term::Term;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use super::error::ParseError;
use super::{Extractor, ExtractorResult, TermMapExtractor};
use crate::new_rml::extractors::FromVocab;
use crate::new_rml::rml_model::v2::core::expression_map::term_map::{
    CommonTermMapInfo, PredicateMap,
};
use crate::new_rml::rml_model::v2::TermMapEnum;

impl TermMapExtractor<TermMapEnum> for PredicateMap {
    fn create_shortcut_map(
        term_map_info: CommonTermMapInfo,
    ) -> ExtractorResult<TermMapEnum> {
        if !term_map_info.is_iri_term_type() {
            return Err(ParseError::GenericError(
                "Constant-valued PredicateMap has to have an IRI as value"
                    .to_string(),
            ));
        }
        Ok(TermMapEnum::PredicateMap(PredicateMap { term_map_info }))
    }

    fn extract_self_term_map<TS>(
        subj_ref: TS,
        graph_ref: &FastGraph,
    ) -> super::ExtractorResult<TermMapEnum>
    where
        TS: Term + Clone,
    {
        let term_map_info =
            CommonTermMapInfo::extract_self(subj_ref, graph_ref)?;
        Ok(TermMapEnum::PredicateMap(PredicateMap { term_map_info }))
    }

    fn get_shortcut_preds() -> Vec<RcTerm> {
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
    use crate::import_test_mods;
    use crate::new_rml::extractors::error::ParseError;
    use crate::new_rml::rml_model::v2::core::expression_map::{
        BaseExpressionMapEnum, ExpressionMapEnum,
    };

    import_test_mods!(new_rml);

    #[test]
    fn create_const_predicatemap_test() -> ExtractorResult<()> {
        let graph = load_graph!("rml/sample_mapping.ttl")?;
        let pm_const_pred = vocab::r2rml::PROPERTY::PREDICATE.to_rcterm();
        let triples = graph.triples_matching(Any, [pm_const_pred], Any);
        let values = triples.flatten().map(|trip| trip.o().to_owned());
        let pms: Vec<TermMapEnum> = values
            .map(|map_const| {
                PredicateMap::extract_constant_term_map(&map_const)
            })
            .collect::<ExtractorResult<_>>()?;

        assert_eq!(pms.len(), 2);

        for pm in pms.iter() {
            assert!(pm.as_ref().is_iri_term_type());
            match &pm.as_ref().expression {
                ExpressionMapEnum::BaseExpressionMap(base_expr) => {
                    match base_expr {
                        BaseExpressionMapEnum::Constant(_) => {}
                        _ => {
                            return Err(ParseError::GenericError(format!(
                                "Predicate map is not a constant term map {:?}",
                                pm
                            )))
                        }
                    }
                }
                _ => {
                    return Err(ParseError::GenericError(
                            format!("Predicate map's expression map is not an expression map defined in RML-Core {:?}", pm.as_ref().expression)))
                },
            }
        }

        Ok(())
    }
}
