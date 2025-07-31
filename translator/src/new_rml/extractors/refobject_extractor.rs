use sophia_api::term::{Term, TermKind};
use sophia_inmem::graph::FastGraph;

use super::store::{get_object_with_ps, get_objects_with_ps};
use super::{stringify_rcterm, Extractor, ExtractorResult, FromVocab};
use crate::new_rml::rml_model::v2::core::expression_map::{
    BaseExpressionMapEnum, ExpressionMapEnum, ExpressionMapKind,
};
use crate::new_rml::rml_model::v2::core::{JoinCondition, RefObjectMap};

fn parent_child_extract<TS, TP>(
    subject_ref: TS,
    graph_ref: &FastGraph,
    preds: &[TP],
) -> ExtractorResult<ExpressionMapEnum>
where
    TS: Term,
    TP: Term,
{
    let term = get_object_with_ps(graph_ref, subject_ref, preds)?;
    if term.kind() == TermKind::Literal {
        Ok(ExpressionMapEnum::new_constant_term(term))
    } else {
        ExpressionMapEnum::extract_self(&term, graph_ref)
    }
}

impl Extractor<JoinCondition> for JoinCondition {
    fn extract_self<TS>(
        subject_ref: TS,
        graph_ref: &FastGraph,
    ) -> super::ExtractorResult<JoinCondition>
    where
        TS: Term,
    {
        let parent = parent_child_extract(
            subject_ref.borrow_term(),
            graph_ref,
            &[
                vocab::r2rml::PROPERTY::PARENT.to_rcterm(),
                vocab::rml_core::PROPERTY::PARENT.to_rcterm(),
                vocab::rml_core::PROPERTY::PARENT_MAP.to_rcterm(),
            ],
        )?;

        let child = parent_child_extract(
            subject_ref,
            graph_ref,
            &[
                vocab::r2rml::PROPERTY::CHILD.to_rcterm(),
                vocab::rml_core::PROPERTY::CHILD.to_rcterm(),
                vocab::rml_core::PROPERTY::CHILD_MAP.to_rcterm(),
            ],
        )?;

        Ok(JoinCondition { parent, child })
    }
}

impl Extractor<RefObjectMap> for RefObjectMap {
    fn extract_self<TS>(
        subject_ref: TS,
        graph_ref: &FastGraph,
    ) -> super::ExtractorResult<RefObjectMap>
    where
        TS: Term,
    {
        let ptm_iri = get_object_with_ps(
            graph_ref,
            subject_ref.borrow_term(),
            &[
                &vocab::r2rml::PROPERTY::PARENTTRIPLESMAP.to_rcterm(),
                &vocab::rml_core::PROPERTY::PARENT_TRIPLES_MAP.to_rcterm(),
            ],
        )?;

        let join_condi_vec = get_objects_with_ps(
            graph_ref,
            subject_ref,
            &[
                &vocab::r2rml::PROPERTY::JOINCONDITION.to_rcterm(),
                &vocab::rml_core::PROPERTY::JOIN_CONDITION.to_rcterm(),
            ],
        );

        let join_condition = join_condi_vec
            .into_iter()
            .flat_map(|join_condi_node| {
                JoinCondition::extract_self(&join_condi_node, graph_ref).ok()
            })
            .collect();

        Ok(RefObjectMap {
            ptm_iri,
            join_condition,
        })
    }
}
