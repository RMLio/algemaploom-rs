use log::debug;
use sophia_api::term::Term;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use super::{stringify_rcterm, TermMapExtractor};
use crate::new_rml::extractors::store::get_object_with_ps;
use crate::new_rml::extractors::{Extractor, FromVocab};
use crate::new_rml::rml_model::v2::core::expression_map::term_map::{
    CommonTermMapInfo, ObjectMap,
};
use crate::new_rml::rml_model::v2::core::expression_map::{
    BaseExpressionMapEnum, ExpressionMapEnum
};
use crate::new_rml::rml_model::v2::TermMapEnum;

fn extract_sub_expr_maps<TS, TCP, TMP>(
    subj_ref: TS,
    graph_ref: &FastGraph,
    const_preds: &[TCP],
    map_preds: &[TMP],
) -> Option<ExpressionMapEnum>
where
    TS: Term,
    TMP: Term,
    TCP: Term,
{
    let datatype_const_opt =
        get_object_with_ps(graph_ref, subj_ref.borrow_term(), const_preds).ok();

    if let Some(datatype_iri) = datatype_const_opt {
        return Some(ExpressionMapEnum::new_constant_term(datatype_iri));
    }

    get_object_with_ps(graph_ref, subj_ref.borrow_term(), map_preds)
        .ok()
        .and_then(|dtype_map_iri| {
            ExpressionMapEnum::extract_self(&dtype_map_iri, graph_ref).ok()
        })
}

impl TermMapExtractor<TermMapEnum> for ObjectMap {
    fn create_shortcut_map(term_map: CommonTermMapInfo) -> TermMapEnum {
        if term_map.is_bnode_term_type() {
            panic!("Constant-valued ObjectMap has to have an IRI or a Literal as value");
        }

        TermMapEnum::ObjectMap(Self {
            term_map_info: term_map,
            language_map:  None,
            datatype_map:  None,
        })
    }

    fn create_term_map<TTerm>(
        subj_ref: TTerm,
        graph_ref: &FastGraph,
    ) -> super::ExtractorResult<TermMapEnum>
    where
        TTerm: Term + Clone,
    {
        debug!("Object map is extracting for subj ref: {:?}", subj_ref);
        let datatype_map = extract_sub_expr_maps(
            subj_ref.borrow_term(),
            graph_ref,
            &[
                &vocab::r2rml::PROPERTY::DATATYPE.to_rcterm(),
                &vocab::rml_core::PROPERTY::DATATYPE.to_rcterm(),
            ],
            &[&vocab::rml_core::PROPERTY::DATATYPE_MAP.to_rcterm()],
        );

        let language_map = extract_sub_expr_maps(
            subj_ref.borrow_term(),
            graph_ref,
            &[
                &vocab::r2rml::PROPERTY::LANGUAGE.to_rcterm(),
                &vocab::rml_core::PROPERTY::LANGUAGE.to_rcterm(),
            ],
            &[
                &vocab::rml_core::PROPERTY::LANGUAGE_MAP.to_rcterm(),
                &vocab::rml::PROPERTY::LANGUAGE_MAP.to_rcterm(),
            ],
        );

        let term_map_info =
            CommonTermMapInfo::extract_self(subj_ref.borrow_term(), graph_ref)?;
        debug!(
            "Object map is extracting with term map info: {:?}",
            term_map_info
        );

        Ok(TermMapEnum::ObjectMap(ObjectMap {
            term_map_info,
            language_map,
            datatype_map,
        }))
    }

    fn get_shortcut_preds() -> Vec<RcTerm> {
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
