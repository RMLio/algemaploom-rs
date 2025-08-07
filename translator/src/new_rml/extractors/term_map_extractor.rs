use std::fmt::Debug;

use log::debug;
use sophia_api::graph::Graph;
use sophia_api::prelude::Any;
use sophia_api::term::{BnodeId, FromTerm, Term, TermKind};
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use super::store::get_objects_with_ps;
use super::{Extractor, ExtractorResult, FromVocab};
use crate::new_rml::extractors::store::{get_object, get_object_with_ps};
use crate::new_rml::extractors::ParseError;
use crate::new_rml::rml_model::v2::core::expression_map::term_map::{
    termkind_to_rml_rcterm, CommonTermMapInfo,
};
use crate::new_rml::rml_model::v2::core::expression_map::{
    BaseExpressionMapEnum, ExpressionMapEnum,
};
use crate::new_rml::rml_model::v2::io::target::LogicalTarget;

impl Extractor<CommonTermMapInfo> for CommonTermMapInfo {
    fn extract_self<TTerm>(
        subject_ref: TTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<CommonTermMapInfo>
    where
        TTerm: Term + Clone,
    {
        let ltarget_old_pred = &vocab::rml::PROPERTY::LOGICALTARGET.to_rcterm();
        let ltarget_new_pred =
            &vocab::rml_core::PROPERTY::LOGICAL_TARGET.to_rcterm();
        let ltarget_terms = get_objects_with_ps(
            graph_ref,
            subject_ref.borrow_term(),
            &[ltarget_old_pred, ltarget_new_pred],
        );
        let logical_targets_res: ExtractorResult<_> = ltarget_terms
            .into_iter()
            .try_fold(Vec::new(), |mut acc, ltarget_term| {
                acc.push(LogicalTarget::extract_self(ltarget_term, graph_ref)?);
                Ok(acc)
            });
        let mut logical_targets = logical_targets_res?;
        if logical_targets.is_empty() {
            logical_targets.push(LogicalTarget::default());
        }

        let expression = ExpressionMapEnum::extract_self(
            subject_ref.borrow_term(),
            graph_ref,
        )?;

        let ttype_old_pred = &vocab::r2rml::PROPERTY::TERMTYPE.to_rcterm();
        let ttype_pred = &vocab::rml_core::PROPERTY::TERMTYPE.to_rcterm();
        let ttype_iri_opt = get_object_with_ps(
            graph_ref,
            subject_ref.borrow_term(),
            &[ttype_old_pred, ttype_pred],
        )
        .ok();

        let term_type = if let Some(ttype_iri) = ttype_iri_opt {
            if ttype_iri.kind() != TermKind::Iri {
                return Err(ParseError::GenericError(format!(
                    "Term type node for {:?} has value {:?} which is not an IRI",
                    subject_ref, ttype_iri
                )).into());
            }
            if let Ok(BaseExpressionMapEnum::Constant(val)) =
                expression.try_unwrap_base_expression_map_ref()
            {
                return Err(ParseError::GenericError(
                        format!("Term type is explicity defined for node {:?} even though it is a constant term map with value {:?}", 
                            subject_ref, val)).into());
            }
            Ok(ttype_iri)
        } else {
            infer_term_type(subject_ref.borrow_term(), graph_ref)
        }?;

        Ok(CommonTermMapInfo {
            identifier: RcTerm::from_term(subject_ref),
            term_type,
            expression,
            logical_targets,
        })
    }
}

fn infer_term_type<TTerm>(
    subject_ref: TTerm,
    graph_ref: &FastGraph,
) -> Result<RcTerm, ParseError>
where
    TTerm: Term + Debug,
{
    let triple =  graph_ref.triples_matching(Any,Any,  [subject_ref.borrow_term()])
        .flatten()
        .next()
        .ok_or(ParseError::GenericError(
                format!("Dangling term map which is not used anywhere in the mapping document {:?}", subject_ref)
                ))?;

    debug!("Inferring term type for term map {:?}", subject_ref);
    let rml_termmap_pred: RcTerm = RcTerm::from_term(triple.p());

    let rml_termmap_type: RMLTermMapType = rml_termmap_pred.try_into()?;
    match rml_termmap_type {
        RMLTermMapType::ObjectMap => {
            debug!("{:?} is an object map", subject_ref);
            debug!("Inferring term type for object map");
            let datatype_lang_opt = graph_ref
                .triples_matching(
                    [subject_ref.borrow_term()],
                    [
                        vocab::rml_core::PROPERTY::LANGUAGE.to_rcterm(),
                        vocab::rml_core::PROPERTY::LANGUAGE_MAP.to_rcterm(),
                        vocab::rml_core::PROPERTY::DATATYPE.to_rcterm(),
                        vocab::rml_core::PROPERTY::DATATYPE_MAP.to_rcterm(),
                        vocab::rml_core::PROPERTY::REFERENCE.to_rcterm(),
                    ],
                    Any,
                )
                .flatten()
                .next();

            let constant_value_opt = get_object(
                graph_ref,
                subject_ref.borrow_term(),
                vocab::rml_core::PROPERTY::CONSTANT.to_rcterm(),
            )
            .ok();

            if datatype_lang_opt.is_some() {
                Ok(vocab::rml_core::CLASS::LITERAL.to_rcterm())
            } else if let Some(term) = constant_value_opt {
                termkind_to_rml_rcterm(term.kind())
            } else {
                Ok(vocab::rml_core::CLASS::IRI.to_rcterm())
            }
        }
        _ => Ok(vocab::rml_core::CLASS::IRI.to_rcterm()),
    }
}

#[derive(Debug, Clone)]
enum RMLTermMapType {
    SubjectMap,
    PredicateMap,
    ObjectMap,
    GraphMap,
}

impl TryInto<RMLTermMapType> for RcTerm {
    type Error = ParseError;

    fn try_into(self) -> Result<RMLTermMapType, Self::Error> {
        (&self).try_into()
    }
}

impl<'a> TryInto<RMLTermMapType> for &'a RcTerm {
    type Error = ParseError;

    fn try_into(self) -> Result<RMLTermMapType, Self::Error> {
        match self {
            value
                if value == &vocab::r2rml::PROPERTY::SUBJECTMAP.to_rcterm()
                    || value
                        == &vocab::rml_core::PROPERTY::SUBJECT_MAP
                            .to_rcterm() =>
            {
                Ok(RMLTermMapType::SubjectMap)
            }
            value
                if value
                    == &vocab::r2rml::PROPERTY::PREDICATEMAP.to_rcterm()
                    || value
                        == &vocab::rml_core::PROPERTY::PREDICATE_MAP
                            .to_rcterm() =>
            {
                Ok(RMLTermMapType::PredicateMap)
            }
            value
                if value == &vocab::r2rml::PROPERTY::OBJECTMAP.to_rcterm()
                    || value
                        == &vocab::rml_core::PROPERTY::OBJECT_MAP
                            .to_rcterm() =>
            {
                Ok(RMLTermMapType::ObjectMap)
            }
            value
                if value == &vocab::r2rml::PROPERTY::GRAPHMAP.to_rcterm()
                    || value
                        == &vocab::rml_core::PROPERTY::GRAPH_MAP
                            .to_rcterm() =>
            {
                Ok(RMLTermMapType::GraphMap)
            }

            _ => {
                Err(ParseError::GenericError(format!(
                    "Term map type cannot be inferred for {:?}",
                    self
                )))
            }
        }
    }
}
