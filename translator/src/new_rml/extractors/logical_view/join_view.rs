use std::rc::Rc;

use sophia_api::term::Term;
use sophia_inmem::graph::FastGraph;

use crate::new_rml::error::NewRMLTranslationError;
use crate::new_rml::extractors::error::ParseError;
use crate::new_rml::extractors::store::{get_object, get_objects};
use crate::new_rml::extractors::{Extractor, ExtractorResult, FromVocab};
use crate::new_rml::rml_model::v2::core::JoinCondition;
use crate::new_rml::rml_model::v2::lv::RMLFieldKind::Iterable;
use crate::new_rml::rml_model::v2::lv::{LogicalView, LogicalViewJoin, RMLField};

impl Extractor<LogicalViewJoin> for LogicalViewJoin {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &FastGraph,
    ) -> ExtractorResult<LogicalViewJoin>
    where
        TTerm: Term,
    {
        let join_condition = get_object(
            graph_ref,
            subject_ref.borrow_term(),
            &vocab::rml_core::PROPERTY::JOIN_CONDITION.to_rcterm(),
        )
        .and_then(|term| JoinCondition::extract_self(&term, graph_ref))?;

        let parent_view_term = get_object(
            graph_ref,
            subject_ref.borrow_term(),
            &vocab::rml_lv::PROPERTY::PARENT_LOGICAL_VIEW.to_rcterm(),
        )?;
        let parent_view =
            Rc::new(LogicalView::extract_self(&parent_view_term, graph_ref)?);

        let fields = get_objects(
            graph_ref,
            subject_ref.borrow_term(),
            &vocab::rml_lv::PROPERTY::FIELD.to_rcterm(),
        )
        .iter()
        .try_fold(Vec::new(), |mut acc, t| -> Result<Vec<RMLField>, NewRMLTranslationError> {
            let res = RMLField::extract_self(t, graph_ref);
            match res {
                Ok(field) => {
                    if let Iterable(_) = field.kind{
                        Err(ParseError::GenericError(format!("Logical view join's field cannot be an iterable {:?}", t)).into())
                    }else{
                        acc.push(field);
                        Ok(acc)
                    }
                }
                Err(e) => Err(e),
            }
        })?;

        Ok(LogicalViewJoin {
            join_condition,
            parent_view,
            fields,
        })
    }
}
