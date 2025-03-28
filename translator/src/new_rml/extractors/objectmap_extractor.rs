use sophia_api::term::Term;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use super::{stringify_rcterm, TermMapExtractor};
use crate::new_rml::extractors::store::get_object_with_ps;
use crate::new_rml::extractors::{Extractor, FromVocab};
use crate::new_rml::rml_model::v2::core::expression_map::term_map::{
    GraphMap, ObjectMap, TermMap,
};
use crate::new_rml::rml_model::v2::core::expression_map::{
    ExpressionMap, ExpressionMapKind,
};

fn extract_sub_expr_maps<TS, TCP, TMP>(
    subj_ref: TS,
    graph_ref: &FastGraph,
    const_preds: &Vec<TCP>,
    map_preds: &Vec<TMP>,
) -> Option<ExpressionMap>
where
    TS: Term,
    TMP: Term,
    TCP: Term,
{
    let datatype_const_opt =
        get_object_with_ps(graph_ref, subj_ref.borrow_term(), const_preds).ok();

    if let Some(datatype_iri) = datatype_const_opt {
        return Some(ExpressionMap {
            map_type_pred_iri: vocab::rml_core::PROPERTY::CONSTANT.to_rcterm(),
            kind:              ExpressionMapKind::NonFunction(
                stringify_rcterm(datatype_iri).unwrap(),
            ),
        });
    }

    get_object_with_ps(graph_ref, subj_ref.borrow_term(), map_preds)
        .ok()
        .map(|dtype_map_iri| {
            ExpressionMap::extract_self(&dtype_map_iri, graph_ref).ok()
        })
        .flatten()
}

impl TermMapExtractor<ObjectMap> for ObjectMap {
    fn create_constant_map(term_map: TermMap) -> ObjectMap {
        if term_map.is_bnode_term_type() {
            panic!("Constant-valued ObjectMap has to have an IRI or a Literal as value");
        }

        Self {
            term_map,
            language_map: None,
            datatype_map: None,
        }
    }

    fn create_term_map<TTerm>(
        subj_ref: TTerm,
        graph_ref: &FastGraph,
    ) -> super::ExtractorResult<ObjectMap>
    where
        TTerm: Term + Clone,
    {
        let datatype_map = extract_sub_expr_maps(
            subj_ref.borrow_term(),
            graph_ref,
            &vec![
                &vocab::r2rml::PROPERTY::DATATYPE.to_rcterm(),
                &vocab::rml_core::PROPERTY::DATATYPE.to_rcterm(),
            ],
            &vec![&vocab::rml_core::PROPERTY::DATATYPE_MAP.to_rcterm()],
        );

        let language_map = extract_sub_expr_maps(
            subj_ref.borrow_term(),
            graph_ref,
            &vec![
                &vocab::r2rml::PROPERTY::LANGUAGE.to_rcterm(),
                &vocab::rml_core::PROPERTY::LANGUAGE.to_rcterm(),
            ],
            &vec![
                &vocab::rml_core::PROPERTY::LANGUAGE_MAP.to_rcterm(),
                &vocab::rml::PROPERTY::LANGUAGE_MAP.to_rcterm(),
            ],
        );

        let term_map =
            TermMap::extract_self(subj_ref.borrow_term(), graph_ref)?;

        Ok(ObjectMap {
            term_map,
            language_map,
            datatype_map,
        })
    }

    fn get_const_preds() -> Vec<RcTerm> {
        vec![
            vocab::r2rml::PROPERTY::OBJECT.to_rcterm(),
            vocab::rml_core::PROPERTY::OBJECT.to_rcterm(),
        ]
    }

    fn get_map_preds() -> Vec<RcTerm> {
        vec![
            vocab::r2rml::PROPERTY::OBJECTMAP.to_rcterm(),
            vocab::rml_core::PROPERTY::OBJECT_MAP.to_rcterm(),
        ]
    }
}

#[cfg(test)]
mod tests {


    use super::*;
    use crate::new_rml::error::NewRMLTranslationError;
    use crate::new_rml::extractors::error::ParseError;
    use crate::import_test_mods;
    use crate::new_rml::rml_model::v2::core::expression_map::ExpressionValueEnum;
    use sophia_api::graph::Graph;
    use sophia_api::prelude::Any;
    use sophia_api::triple::Triple;

    import_test_mods!(new_rml);

    #[test]
    fn map_object_test() -> ExtractorResult<()> {
        let graph: FastGraph = load_graph!("rml/sample_mapping.ttl")?;
        let map_pred = vocab::r2rml::PROPERTY::OBJECTMAP.to_rcterm();
        let container_vec = graph
            .triples_matching(Any, [map_pred], Any)
            .flatten()
            .map(|trip| trip.s().to_owned());

        let obj_maps: Vec<_> = container_vec
            .flat_map(|objmap_container| {
                ObjectMap::extract_many_from_container(
                    &graph,
                    &objmap_container,
                )
            })
            .flatten()
            .collect();

        assert_eq!(obj_maps.len(), 2);
        let _ = obj_maps.iter().try_for_each(|om| {
            assert_eq!(
                om.term_map.term_type,
                vocab::rml_core::CLASS::LITERAL.to_rcterm()
            );
            assert_eq!(
                om.term_map.expression.get_value_type_enum()?,
                ExpressionValueEnum::Reference
            );
            Ok::<(), NewRMLTranslationError>(())
        });

        Ok(())
    }
}
