use std::rc::Rc;

use operator::formats::xml::XPathConfig;
use operator::formats::{self};
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;
use xml::XPathRefFormParser;

use super::source::{ReferenceFormulation, ReferenceFormulationKind};
use crate::extractors::error::ParseError;
use crate::extractors::store::get_object;
use crate::extractors::{stringify_term, FromVocab};

pub mod xml;

impl TryFrom<ReferenceFormulation> for formats::ReferenceFormulation {
    type Error = ParseError;

    fn try_from(value: ReferenceFormulation) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}

impl TryFrom<&ReferenceFormulation>
    for formats::ReferenceFormulation
{
    type Error = ParseError;

    fn try_from(value: &ReferenceFormulation) -> Result<Self, Self::Error> {
        match &value.kind {
            ReferenceFormulationKind::Iri => {
                match value.iri.clone() {
                    value
                        if value
                            == vocab::d2rq::class::DATABASE.to_rcterm()
                            || value
                                == vocab::rml_io::class::SQL_QUERY
                                    .to_rcterm()
                            || value
                                == vocab::rml_io::class::SQL_TABLE
                                    .to_rcterm()
                            || value
                                == vocab::rml_io::class::SQL2008_QUERY
                                    .to_rcterm()
                            || value
                                == vocab::rml_io::class::SQL2008_TABLE
                                    .to_rcterm() =>
                    {
                        Ok(formats::ReferenceFormulation::SQLQuery)
                    }
                    value
                        if value == vocab::query::class::CSV.to_rcterm()
                            || value
                                == vocab::rml_io::class::CSV.to_rcterm() =>
                    {
                        Ok(formats::ReferenceFormulation::CSVRows)
                    }
                    value
                        if value
                            == vocab::query::class::JSONPATH.to_rcterm()
                            || value
                                == vocab::rml_io::class::JSONPATH
                                    .to_rcterm() =>
                    {
                        Ok(formats::ReferenceFormulation::JSONPath)
                    }
                    value
                        if value == vocab::query::class::XPATH.to_rcterm()
                            || value
                                == vocab::rml_io::class::XPATH.to_rcterm() =>
                    {
                        Ok(formats::ReferenceFormulation::XMLPath(
                            XPathConfig::default(),
                        ))
                    }
                    value if value == vocab::query::class::HTML.to_rcterm() => {
                        Ok(formats::ReferenceFormulation::CSS3)
                    }
                    value => {
                        Err(ParseError::GenericError(format!(
                            "Unsupported reference formulation: {}",
                            stringify_term(value).unwrap()
                        )))
                    }
                }
            }
            ReferenceFormulationKind::CustomReferenceFormulation {
                meta_data_graph,
            } => try_from_custom_ref_form(&value, meta_data_graph),
        }
    }
}

pub trait ComplexRefFormulationParser {
    fn parse_complex_ref_form(
        subj_ref: &RcTerm,
        meta_data_graph: &Rc<FastGraph>,
    ) -> Result<operator::formats::ReferenceFormulation, ParseError>;
}

pub fn try_from_custom_ref_form(
    value: &ReferenceFormulation,
    meta_data_graph: &Rc<FastGraph>,
) -> Result<operator::formats::ReferenceFormulation, ParseError> {
    let subj_ref = &value.iri;
    let reference_form_type = get_object(
        &meta_data_graph,
        subj_ref,
        vocab::rdf::property::TYPE.to_rcterm(),
    )?;

    match reference_form_type {
        value if value == vocab::rml_io::class::XPATH_REF_FORM.to_rcterm() => {
            XPathRefFormParser::parse_complex_ref_form(
                subj_ref,
                meta_data_graph,
            )
        }
        _ => {
            Err(ParseError::GenericError(format!(
                "Complex reference formulation unsupported: {:?}",
                value
            )))
        }
    }
}
